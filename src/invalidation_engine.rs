use crate::{
    compute_default_artifact_admissibility,
    compute_default_packet_admissibility,
    remediation_required_for_packet,
    plan_artifact_remediation,
    AdmissibilityState,
    ArtifactRecord,
    CriticStatus,
    EventRecord,
    EventType,
    FreshnessState,
    LifecycleState,
    PacketRecord,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactInvalidationDecision {
    NoEffect,
    ReviewDue,
    Stale,
    Invalidated,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactInvalidationOutcome {
    pub artifact: ArtifactRecord,
    pub decision: ArtifactInvalidationDecision,
    pub overlapped: bool,
    pub reason: String,
    pub remediation_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketInvalidationOutcome {
    pub packet: PacketRecord,
    pub affected_artifact_ids: Vec<String>,
    pub reevaluation_required: bool,
    pub reason: String,
    pub remediation_required: bool,
}

fn artifact_event_overlaps(event: &EventRecord, artifact: &ArtifactRecord) -> bool {
    if event.repo_id != artifact.repo_id {
        return false;
    }

    if event
        .related_artifact_ids
        .iter()
        .any(|id| id == &artifact.artifact_id)
    {
        return true;
    }

    if event
        .source_refs
        .iter()
        .any(|source_ref| artifact.source_refs.iter().any(|artifact_ref| artifact_ref == source_ref))
    {
        return true;
    }

    matches!(event.event_type, EventType::AuthorityRecordChanged)
}

fn classify_freshness_downgrade(
    event_type: EventType,
    current: FreshnessState,
) -> ArtifactInvalidationDecision {
    match event_type {
        EventType::SourceChanged => match current {
            FreshnessState::Fresh => ArtifactInvalidationDecision::ReviewDue,
            FreshnessState::ReviewDue => ArtifactInvalidationDecision::Stale,
            FreshnessState::Stale => ArtifactInvalidationDecision::Stale,
            FreshnessState::Invalidated => ArtifactInvalidationDecision::Invalidated,
        },
        EventType::SourceValidationChanged => match current {
            FreshnessState::Fresh | FreshnessState::ReviewDue => ArtifactInvalidationDecision::Stale,
            FreshnessState::Stale => ArtifactInvalidationDecision::Stale,
            FreshnessState::Invalidated => ArtifactInvalidationDecision::Invalidated,
        },
        EventType::SourceMoved
        | EventType::SourceDeleted
        | EventType::AuthorityRecordChanged
        | EventType::ArtifactInvalidated => ArtifactInvalidationDecision::Invalidated,
        _ => ArtifactInvalidationDecision::NoEffect,
    }
}

pub fn apply_artifact_invalidation(
    event: &EventRecord,
    artifact: &ArtifactRecord,
) -> ArtifactInvalidationOutcome {
    let overlapped = artifact_event_overlaps(event, artifact);

    if !overlapped {
        return ArtifactInvalidationOutcome {
            artifact: artifact.clone(),
            decision: ArtifactInvalidationDecision::NoEffect,
            overlapped: false,
            reason: "event did not overlap artifact dependency surface".into(),
            remediation_required: false,
        };
    }

    let decision = classify_freshness_downgrade(event.event_type, artifact.freshness_state);

    if decision == ArtifactInvalidationDecision::NoEffect {
        return ArtifactInvalidationOutcome {
            artifact: artifact.clone(),
            decision,
            overlapped: true,
            reason: "event overlapped artifact but did not require freshness downgrade".into(),
            remediation_required: false,
        };
    }

    let mut updated = artifact.clone();

    updated.freshness_state = match decision {
        ArtifactInvalidationDecision::NoEffect => updated.freshness_state,
        ArtifactInvalidationDecision::ReviewDue => FreshnessState::ReviewDue,
        ArtifactInvalidationDecision::Stale => FreshnessState::Stale,
        ArtifactInvalidationDecision::Invalidated => FreshnessState::Invalidated,
    };

    updated.admissibility_state = compute_default_artifact_admissibility(
        updated.lifecycle_state,
        updated.freshness_state,
        updated.critic_status,
    );

    let remediation_required = plan_artifact_remediation(&updated).is_some();

    ArtifactInvalidationOutcome {
        artifact: updated,
        decision,
        overlapped: true,
        reason: format!(
            "event {:?} downgraded artifact {}",
            event.event_type, artifact.artifact_id
        ),
        remediation_required,
    }
}

pub fn apply_packet_constituent_change(
    packet: &PacketRecord,
    constituent_artifacts: &[ArtifactRecord],
) -> PacketInvalidationOutcome {
    let affected_artifacts: Vec<&ArtifactRecord> = constituent_artifacts
        .iter()
        .filter(|artifact| {
            packet
                .included_artifact_ids
                .iter()
                .any(|included_id| included_id == &artifact.artifact_id)
        })
        .collect();

    if affected_artifacts.is_empty() {
        return PacketInvalidationOutcome {
            packet: packet.clone(),
            affected_artifact_ids: Vec::new(),
            reevaluation_required: false,
            reason: "no changed constituent artifacts were included in the packet".into(),
            remediation_required: false,
        };
    }

    let affected_artifact_ids: Vec<String> = affected_artifacts
        .iter()
        .map(|artifact| artifact.artifact_id.clone())
        .collect();

    let freshness: Vec<FreshnessState> = affected_artifacts
        .iter()
        .map(|artifact| artifact.freshness_state)
        .collect();

    let lifecycle: Vec<LifecycleState> = affected_artifacts
        .iter()
        .map(|artifact| artifact.lifecycle_state)
        .collect();

    let reevaluation_required = freshness.iter().any(|state| {
        matches!(state, FreshnessState::Stale | FreshnessState::Invalidated)
    }) || lifecycle.iter().any(|state| {
        matches!(state, LifecycleState::Blocked | LifecycleState::Superseded)
    });

    let mut updated = packet.clone();
    if reevaluation_required {
        updated.reevaluation_required = true;
    }

    updated.admissibility_state = compute_default_packet_admissibility(
        updated.lifecycle_state,
        updated.required_constituents_present,
        updated.reevaluation_required,
        &freshness,
        &lifecycle,
    );

    let remediation_required =
        remediation_required_for_packet(&updated, &freshness, &lifecycle);

    PacketInvalidationOutcome {
        packet: updated,
        affected_artifact_ids,
        reevaluation_required,
        reason: if reevaluation_required {
            "constituent degradation requires packet reevaluation".into()
        } else {
            "changed constituents did not degrade packet trust posture".into()
        },
        remediation_required,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        AdmissibilityState,
        ArtifactClass,
        ArtifactRecord,
        AuthorityLevel,
        CriticStatus,
        EventRecord,
        EventType,
        FreshnessState,
        LifecycleState,
        PacketLifecycleState,
        PacketRecord,
        PacketRole,
        SensitivityClassification,
    };

    use super::{
        apply_artifact_invalidation,
        apply_packet_constituent_change,
        ArtifactInvalidationDecision,
    };

    fn base_artifact() -> ArtifactRecord {
        ArtifactRecord {
            schema_version: "1.0".into(),
            artifact_id: "art-001".into(),
            artifact_class: ArtifactClass::RepoNavigationMap,
            repo_id: "forge-command".into(),
            title: "ForgeCommand Repo Navigation".into(),
            operational_purpose: "Provide first-wave repo structure".into(),
            summary_block: "Bounded navigational summary".into(),
            source_refs: vec!["src-tauri/src/lib.rs".into(), "doc/fcSYSTEM.md".into()],
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

    fn base_packet() -> PacketRecord {
        PacketRecord {
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
        }
    }

    fn source_changed_event() -> EventRecord {
        EventRecord {
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
            event_payload: "source changed".into(),
        }
    }

    fn source_deleted_event() -> EventRecord {
        EventRecord {
            event_id: "evt-002".into(),
            event_type: EventType::SourceDeleted,
            schema_version: "1.0".into(),
            emitted_at: "2026-04-09T00:00:05-04:00".into(),
            emitter_service: "repo-watcher".into(),
            repo_id: "forge-command".into(),
            related_artifact_ids: vec!["art-001".into()],
            related_packet_ids: vec!["pkt-001".into()],
            source_refs: vec!["src-tauri/src/lib.rs".into()],
            causation_id: None,
            correlation_id: "corr-002".into(),
            idempotency_key: "idem-002".into(),
            event_payload: "source deleted".into(),
        }
    }

    #[test]
    fn source_changed_downgrades_fresh_artifact_to_review_due() {
        let artifact = base_artifact();
        let outcome = apply_artifact_invalidation(&source_changed_event(), &artifact);

        assert!(outcome.overlapped);
        assert_eq!(outcome.decision, ArtifactInvalidationDecision::ReviewDue);
        assert_eq!(outcome.artifact.freshness_state, FreshnessState::ReviewDue);
        assert_eq!(
            outcome.artifact.admissibility_state,
            AdmissibilityState::AdmissibleWithWarning
        );
    }

    #[test]
    fn source_deleted_invalidates_artifact_and_requires_remediation() {
        let artifact = base_artifact();
        let outcome = apply_artifact_invalidation(&source_deleted_event(), &artifact);

        assert!(outcome.overlapped);
        assert_eq!(outcome.decision, ArtifactInvalidationDecision::Invalidated);
        assert_eq!(outcome.artifact.freshness_state, FreshnessState::Invalidated);
        assert_eq!(
            outcome.artifact.admissibility_state,
            AdmissibilityState::NotAdmissible
        );
        assert!(outcome.remediation_required);
    }

    #[test]
    fn unrelated_event_has_no_effect() {
        let mut event = source_changed_event();
        event.repo_id = "other-repo".into();

        let artifact = base_artifact();
        let outcome = apply_artifact_invalidation(&event, &artifact);

        assert!(!outcome.overlapped);
        assert_eq!(outcome.decision, ArtifactInvalidationDecision::NoEffect);
        assert_eq!(outcome.artifact.freshness_state, FreshnessState::Fresh);
    }

    #[test]
    fn invalidated_constituent_marks_packet_for_reevaluation() {
        let packet = base_packet();
        let mut artifact = base_artifact();
        artifact.freshness_state = FreshnessState::Invalidated;
        artifact.admissibility_state = AdmissibilityState::NotAdmissible;

        let outcome = apply_packet_constituent_change(&packet, &[artifact]);

        assert!(outcome.reevaluation_required);
        assert!(outcome.remediation_required);
        assert!(outcome.packet.reevaluation_required);
        assert_eq!(
            outcome.packet.admissibility_state,
            AdmissibilityState::NotAdmissible
        );
    }

    #[test]
    fn healthy_constituent_keeps_packet_admissible() {
        let packet = base_packet();
        let artifact = base_artifact();

        let outcome = apply_packet_constituent_change(&packet, &[artifact]);

        assert!(!outcome.reevaluation_required);
        assert!(!outcome.remediation_required);
        assert_eq!(outcome.packet.admissibility_state, AdmissibilityState::Admissible);
    }
}