pub mod authority;
pub mod enums;
pub mod events;
pub mod models;
pub mod state_machine;

pub use authority::AuthorityResolutionRecord;
pub use enums::*;
pub use events::{EventBatch, EventLedger, EventProcessingDecision, EventRecord};
pub use models::{ArtifactRecord, OverrideRecord, PacketRecord, RemediationItem};
pub use state_machine::{
    can_transition_artifact_lifecycle,
    can_transition_freshness,
    compute_default_artifact_admissibility,
    compute_default_packet_admissibility,
    validate_artifact_state,
    validate_packet_state,
};

#[cfg(test)]
mod tests {
    use crate::*;

    fn base_authority_record() -> AuthorityResolutionRecord {
        AuthorityResolutionRecord {
            schema_version: "1.0".into(),
            repo_id: "forge-command".into(),
            repo_name: "ForgeCommand".into(),
            repo_archetype: RepoArchetype::DesktopAppRepo,
            authority_order: vec![
                SourceFamily::ContractSchema,
                SourceFamily::CodeRuntime,
                SourceFamily::TestVerification,
                SourceFamily::RepoTruthDoc,
                SourceFamily::ProtocolDoc,
            ],
            approved_source_families: vec![
                SourceFamily::CodeRuntime,
                SourceFamily::ContractSchema,
                SourceFamily::TestVerification,
                SourceFamily::RepoTruthDoc,
                SourceFamily::ProtocolDoc,
            ],
            disallowed_source_families: vec![SourceFamily::GeneratedOutput, SourceFamily::AdvisoryNote],
            ambiguity_rules: vec![
                "code_vs_repo_truth_doc_conflict_blocks".into(),
                "protocol_vs_runtime_conflict_requires_review".into(),
            ],
            escalation_required_conditions: vec!["authority_conflict_on_required_truth".into()],
            approved_derivation_scope: vec![
                ArtifactClass::RepoNavigationMap,
                ArtifactClass::KeyFilePacket,
                ArtifactClass::ValidationCommandPacket,
            ],
            operator_review_required_conditions: vec!["authority_ambiguity".into()],
            notes_on_known_authority_gaps: Some("Initial proof-slice record.".into()),
            created_at: "2026-04-09T00:00:00-04:00".into(),
            last_reviewed_at: "2026-04-09T00:00:00-04:00".into(),
        }
    }

    fn base_artifact() -> ArtifactRecord {
        ArtifactRecord {
            schema_version: "1.0".into(),
            artifact_id: "art-001".into(),
            artifact_class: ArtifactClass::RepoNavigationMap,
            repo_id: "forge-command".into(),
            title: "ForgeCommand Repo Navigation".into(),
            operational_purpose: "Provide first-wave repo structure".into(),
            summary_block: "Bounded navigational summary".into(),
            source_refs: vec!["src-tauri/src".into(), "doc/fcSYSTEM.md".into()],
            source_ref_hashes: vec!["hash-1".into(), "hash-2".into()],
            authority_level: AuthorityLevel::Canonical,
            lifecycle_state: LifecycleState::Approved,
            freshness_state: FreshnessState::Fresh,
            critic_status: CriticStatus::Passed,
            admissibility_state: AdmissibilityState::Admissible,
            related_artifact_refs: vec![],
            supersedes_artifact_id: None,
            protocol_refs: vec!["BDS_BACKEND_ENGINEERING_PROTOCOL".into()],
            created_at: "2026-04-09T00:00:00-04:00".into(),
            last_validated_at: "2026-04-09T00:00:00-04:00".into(),
            producer_identity: "proof-slice-core".into(),
            sensitivity_classification: SensitivityClassification::InternalGeneral,
        }
    }

    #[test]
    fn approved_fresh_passed_artifact_is_admissible() {
        let artifact = base_artifact();
        assert!(validate_artifact_state(&artifact).is_ok());
        assert_eq!(
            compute_default_artifact_admissibility(
                artifact.lifecycle_state,
                artifact.freshness_state,
                artifact.critic_status
            ),
            AdmissibilityState::Admissible
        );
    }

    #[test]
    fn candidate_artifact_cannot_be_admissible() {
        let mut artifact = base_artifact();
        artifact.lifecycle_state = LifecycleState::Candidate;
        artifact.admissibility_state = AdmissibilityState::Admissible;
        let err = validate_artifact_state(&artifact).unwrap_err();
        assert!(err.contains("candidate"));
    }

    #[test]
    fn blocked_artifact_cannot_be_fresh() {
        let mut artifact = base_artifact();
        artifact.lifecycle_state = LifecycleState::Blocked;
        artifact.freshness_state = FreshnessState::Fresh;
        let err = validate_artifact_state(&artifact).unwrap_err();
        assert!(err.contains("blocked"));
    }

    #[test]
    fn critic_failed_cannot_remain_approved() {
        let mut artifact = base_artifact();
        artifact.critic_status = CriticStatus::Failed;
        let err = validate_artifact_state(&artifact).unwrap_err();
        assert!(err.contains("critic failed"));
    }

    #[test]
    fn packet_with_invalidated_required_constituent_is_not_admissible() {
        let packet = PacketRecord {
            schema_version: "1.0".into(),
            packet_id: "pkt-001".into(),
            packet_role: PacketRole::RepoNavigationAssist,
            repo_id: "forge-command".into(),
            included_artifact_ids: vec!["art-001".into()],
            included_artifact_hashes: vec!["hash-art-001".into()],
            packet_constraints: vec!["bounded".into()],
            packet_budget_band: "small".into(),
            lane_compatibility: vec!["neuroforge".into(), "neuronforge".into()],
            lifecycle_state: PacketLifecycleState::Approved,
            admissibility_state: AdmissibilityState::Admissible,
            created_at: "2026-04-09T00:00:00-04:00".into(),
            last_evaluated_at: "2026-04-09T00:00:00-04:00".into(),
            required_constituents_present: true,
            reevaluation_required: false,
            sensitivity_classification: SensitivityClassification::InternalGeneral,
        };

        let result = compute_default_packet_admissibility(
            packet.lifecycle_state,
            packet.required_constituents_present,
            packet.reevaluation_required,
            &[FreshnessState::Invalidated],
            &[LifecycleState::Approved],
        );

        assert_eq!(result, AdmissibilityState::NotAdmissible);
    }

    #[test]
    fn authority_record_fails_closed_when_missing_precedence() {
        let mut record = base_authority_record();
        record.authority_order.clear();
        let err = record.validate().unwrap_err();
        assert!(err.contains("authority_order"));
    }

    #[test]
    fn authority_record_rejects_disallowed_source_use() {
        let record = base_authority_record();
        let err = record
            .validate_derivation_request(
                ArtifactClass::RepoNavigationMap,
                &[SourceFamily::CodeRuntime, SourceFamily::GeneratedOutput],
            )
            .unwrap_err();
        assert!(err.contains("disallowed"));
    }

    #[test]
    fn event_ledger_dedupes_by_idempotency_key() {
        let mut ledger = EventLedger::default();
        let event = EventRecord {
            event_id: "evt-001".into(),
            event_type: EventType::SourceChanged,
            schema_version: "1.0".into(),
            emitted_at: "2026-04-09T00:00:00-04:00".into(),
            emitter_service: "repo-watcher".into(),
            repo_id: "forge-command".into(),
            related_artifact_ids: vec!["art-001".into()],
            related_packet_ids: vec!["pkt-001".into()],
            source_refs: vec!["src-tauri/src/lib.rs".into()],
            causation_id: None,
            correlation_id: "corr-001".into(),
            idempotency_key: "idem-001".into(),
            event_payload: "source hash changed".into(),
        };

        let first = ledger.accept(event.clone()).unwrap();
        let second = ledger.accept(event).unwrap();
        assert_eq!(first, EventProcessingDecision::Accepted);
        assert_eq!(second, EventProcessingDecision::DuplicateIgnored);
    }

    #[test]
    fn event_batch_coalesces_same_repo_same_source_ref() {
        let mut ledger = EventLedger::default();

        let event1 = EventRecord {
            event_id: "evt-001".into(),
            event_type: EventType::SourceChanged,
            schema_version: "1.0".into(),
            emitted_at: "2026-04-09T00:00:00-04:00".into(),
            emitter_service: "repo-watcher".into(),
            repo_id: "forge-command".into(),
            related_artifact_ids: vec![],
            related_packet_ids: vec![],
            source_refs: vec!["src-tauri/src/lib.rs".into()],
            causation_id: None,
            correlation_id: "corr-001".into(),
            idempotency_key: "idem-001".into(),
            event_payload: "first".into(),
        };

        let event2 = EventRecord {
            event_id: "evt-002".into(),
            event_type: EventType::SourceChanged,
            schema_version: "1.0".into(),
            emitted_at: "2026-04-09T00:00:05-04:00".into(),
            emitter_service: "repo-watcher".into(),
            repo_id: "forge-command".into(),
            related_artifact_ids: vec![],
            related_packet_ids: vec![],
            source_refs: vec!["src-tauri/src/lib.rs".into()],
            causation_id: Some("evt-001".into()),
            correlation_id: "corr-001".into(),
            idempotency_key: "idem-002".into(),
            event_payload: "second".into(),
        };

        ledger.accept(event1).unwrap();
        ledger.accept(event2).unwrap();

        let batch = ledger.coalesce_pending();
        assert_eq!(batch.len(), 1);
        assert_eq!(batch[0].events.len(), 2);
    }

    #[test]
    fn override_does_not_mutate_underlying_artifact_truth() {
        let artifact = base_artifact();
        let override_record = OverrideRecord {
            schema_version: "1.0".into(),
            override_id: "ovr-001".into(),
            actor_identity: "charlie".into(),
            reason: "Temporary controlled use during review.".into(),
            scope: "packet_admission_only".into(),
            start_time: "2026-04-09T00:00:00-04:00".into(),
            expiry_time: "2026-04-10T00:00:00-04:00".into(),
            affected_object_type: AffectedObjectType::Artifact,
            affected_object_ids: vec!["art-001".into()],
            review_required_by: "2026-04-10T00:00:00-04:00".into(),
            created_at: "2026-04-09T00:00:00-04:00".into(),
        };

        assert!(override_record.validate().is_ok());
        assert_eq!(artifact.freshness_state, FreshnessState::Fresh);
        assert_eq!(artifact.lifecycle_state, LifecycleState::Approved);
    }
}
