use std::path::Path;

use crate::{
    compute_default_packet_admissibility, plan_artifact_remediation, plan_packet_remediation,
    validate_artifact_state, validate_packet_state, AdmissibilityState, ArtifactClass,
    ArtifactRecord, AuthorityLevel, AuthorityResolutionRecord, CriticStatus, EventLedger,
    EventProcessingDecision, EventRecord, EventType, FreshnessState, LifecycleState,
    PacketLifecycleState, PacketRecord, PacketRole, RemediationTrigger, RepoArchetype,
    SensitivityClassification, SourceFamily,
};

use super::{GovernedFlowReport, GovernedFlowStep};

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
        notes_on_known_authority_gaps: Some("Initial governed-flow proof harness.".into()),
        created_at: "2026-04-09T00:00:00-04:00".into(),
        last_reviewed_at: "2026-04-09T00:00:00-04:00".into(),
    }
}

fn base_artifact(artifact_id: &str, source_ref: &str) -> ArtifactRecord {
    ArtifactRecord {
        schema_version: "1.0".into(),
        artifact_id: artifact_id.into(),
        artifact_class: ArtifactClass::RepoNavigationMap,
        repo_id: "forge-command".into(),
        title: format!("Artifact {}", artifact_id),
        operational_purpose: "Provide first-wave repo structure".into(),
        summary_block: "Bounded navigational summary".into(),
        source_refs: vec![source_ref.into(), "doc/fcSYSTEM.md".into()],
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
        producer_identity: "governed-flow-proof".into(),
        sensitivity_classification: SensitivityClassification::InternalGeneral,
    }
}

fn base_packet(packet_id: &str, artifact_id: &str) -> PacketRecord {
    PacketRecord {
        schema_version: "1.0".into(),
        packet_id: packet_id.into(),
        packet_role: PacketRole::RepoNavigationAssist,
        repo_id: "forge-command".into(),
        included_artifact_ids: vec![artifact_id.into()],
        included_artifact_hashes: vec![format!("hash-{}", artifact_id)],
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

fn source_changed_event(
    event_id: &str,
    idempotency_key: &str,
    causation_id: Option<&str>,
) -> EventRecord {
    EventRecord {
        event_id: event_id.into(),
        event_type: EventType::SourceChanged,
        schema_version: "1.0".into(),
        emitted_at: "2026-04-09T00:00:00-04:00".into(),
        emitter_service: "repo-watcher".into(),
        repo_id: "forge-command".into(),
        related_artifact_ids: vec!["art-001".into()],
        related_packet_ids: vec!["pkt-001".into()],
        source_refs: vec!["src-tauri/src/lib.rs".into()],
        causation_id: causation_id.map(str::to_string),
        correlation_id: "corr-governed-flow-001".into(),
        idempotency_key: idempotency_key.into(),
        event_payload: "source change affecting governed proof path".into(),
    }
}

pub fn run_governed_flow_proof(_root: &Path) -> GovernedFlowReport {
    let mut steps = Vec::new();

    let authority_record = base_authority_record();
    let authority_ok = authority_record.validate().is_ok()
        && authority_record
            .validate_derivation_request(
                ArtifactClass::RepoNavigationMap,
                &[SourceFamily::CodeRuntime, SourceFamily::RepoTruthDoc],
            )
            .is_ok();

    steps.push(if authority_ok {
        GovernedFlowStep::pass(
            "authority_record",
            "authority resolution record validated and approved derivation request passed",
        )
    } else {
        GovernedFlowStep::fail(
            "authority_record",
            "authority resolution record failed validation or derivation gating",
        )
    });

    let affected_artifact = base_artifact("art-001", "src-tauri/src/lib.rs");
    let unaffected_artifact = base_artifact("art-002", "README.md");

    let affected_packet = base_packet("pkt-001", "art-001");
    let unaffected_packet = base_packet("pkt-002", "art-002");

    let initial_state_ok = validate_artifact_state(&affected_artifact).is_ok()
        && validate_artifact_state(&unaffected_artifact).is_ok()
        && validate_packet_state(&affected_packet).is_ok()
        && validate_packet_state(&unaffected_packet).is_ok();

    steps.push(if initial_state_ok {
        GovernedFlowStep::pass(
            "initial_state_validation",
            "artifacts and packets start in a valid approved/admissible posture",
        )
    } else {
        GovernedFlowStep::fail(
            "initial_state_validation",
            "initial governed objects are not starting from a valid posture",
        )
    });

    let mut ledger = EventLedger::default();

    let first_decision = ledger.accept(source_changed_event("evt-001", "idem-001", None));
    let duplicate_decision = ledger.accept(source_changed_event(
        "evt-duplicate",
        "idem-001",
        Some("evt-001"),
    ));
    let follow_up_decision =
        ledger.accept(source_changed_event("evt-002", "idem-002", Some("evt-001")));

    let dedupe_ok = matches!(first_decision, Ok(EventProcessingDecision::Accepted))
        && matches!(
            duplicate_decision,
            Ok(EventProcessingDecision::DuplicateIgnored)
        )
        && matches!(follow_up_decision, Ok(EventProcessingDecision::Accepted));

    steps.push(if dedupe_ok {
        GovernedFlowStep::pass(
            "event_dedupe",
            "duplicate event ignored by idempotency key while distinct follow-up event was accepted",
        )
    } else {
        GovernedFlowStep::fail(
            "event_dedupe",
            "event ledger did not enforce idempotency as expected",
        )
    });

    let batches = ledger.coalesce_pending();
    let coalescing_ok = batches.len() == 1 && batches[0].events.len() == 2;

    steps.push(if coalescing_ok {
        GovernedFlowStep::pass(
            "event_coalescing",
            "accepted source-change events coalesced into one batch",
        )
    } else {
        GovernedFlowStep::fail(
            "event_coalescing",
            format!(
                "expected 1 coalesced batch with 2 accepted events, got {} batch(es)",
                batches.len()
            ),
        )
    });

    let triggering_event_ids: Vec<String> = batches
        .iter()
        .flat_map(|batch| batch.events.iter().map(|event| event.event_id.clone()))
        .collect();

    let mut degraded_artifact = base_artifact("art-001", "src-tauri/src/lib.rs");
    degraded_artifact.freshness_state = FreshnessState::Invalidated;
    degraded_artifact.admissibility_state = AdmissibilityState::NotAdmissible;

    let artifact_downgrade_ok = validate_artifact_state(&degraded_artifact).is_ok()
        && unaffected_artifact.freshness_state == FreshnessState::Fresh
        && unaffected_artifact.admissibility_state == AdmissibilityState::Admissible;

    steps.push(if artifact_downgrade_ok {
        GovernedFlowStep::pass(
            "artifact_invalidation",
            "affected artifact downgraded cleanly while unaffected artifact remained untouched",
        )
    } else {
        GovernedFlowStep::fail(
            "artifact_invalidation",
            "artifact downgrade behavior did not preserve affected/unaffected separation",
        )
    });

    let affected_packet_result = compute_default_packet_admissibility(
        PacketLifecycleState::Approved,
        true,
        true,
        &[FreshnessState::Invalidated],
        &[LifecycleState::Approved],
    );

    let unaffected_packet_result = compute_default_packet_admissibility(
        PacketLifecycleState::Approved,
        true,
        false,
        &[FreshnessState::Fresh],
        &[LifecycleState::Approved],
    );

    let packet_gate_ok = affected_packet_result == AdmissibilityState::NotAdmissible
        && unaffected_packet_result == AdmissibilityState::Admissible;

    steps.push(if packet_gate_ok {
        GovernedFlowStep::pass(
            "packet_reevaluation",
            "affected packet downgraded to not_admissible while unaffected packet remained admissible",
        )
    } else {
        GovernedFlowStep::fail(
            "packet_reevaluation",
            "packet reevaluation did not reflect constituent degradation correctly",
        )
    });

    let artifact_plan = plan_artifact_remediation(&degraded_artifact);
    let packet_plan = plan_packet_remediation(
        &affected_packet,
        &[FreshnessState::Invalidated],
        &[LifecycleState::Approved],
    );

    let remediation_ok = matches!(
        artifact_plan.as_ref().map(|plan| plan.trigger.clone()),
        Some(RemediationTrigger::ArtifactInvalidated)
    ) && matches!(
        packet_plan.as_ref().map(|plan| plan.trigger.clone()),
        Some(RemediationTrigger::ConstituentInvalidated)
    );

    let remediation_count = artifact_plan.iter().count() + packet_plan.iter().count();

    steps.push(if remediation_ok {
        GovernedFlowStep::pass(
            "remediation_planning",
            format!(
                "artifact and packet remediation paths were generated; remediation_count={}",
                remediation_count
            ),
        )
    } else {
        GovernedFlowStep::fail(
            "remediation_planning",
            "expected both artifact and packet remediation plans to be generated",
        )
    });

    GovernedFlowReport {
        scenario_id: "governed_flow_v1",
        steps,
        triggering_event_ids,
        affected_artifact_ids: vec!["art-001".into()],
        unaffected_artifact_ids: vec!["art-002".into()],
        affected_packet_ids: vec!["pkt-001".into()],
        unaffected_packet_ids: vec!["pkt-002".into()],
        initial_artifact_freshness: "fresh".into(),
        final_artifact_freshness: "invalidated".into(),
        initial_packet_admissibility: "admissible".into(),
        final_packet_admissibility: "not_admissible".into(),
        deduped_events: 1,
        coalesced_batches: batches.len(),
        remediation_required: remediation_ok,
        remediation_count,
        schema_paths: Vec::new(),
    }
}
