mod common;

use std::path::Path;

use common::{base_artifact, base_packet};
use precomputed_context_core::{
    apply_artifact_invalidation, apply_packet_constituent_change, run_governed_flow_proof,
    validate_artifact_state, validate_packet_state, AdmissibilityState,
    ArtifactInvalidationDecision, EventRecord, EventType, FreshnessState,
};

#[test]
fn governed_flow_proof_passes_and_preserves_affected_boundaries() {
    let report = run_governed_flow_proof(Path::new("."));

    let failed_steps: Vec<String> = report
        .steps
        .iter()
        .filter(|step| !step.passed)
        .map(|step| format!("{} :: {}", step.step, step.detail))
        .collect();

    assert!(
        failed_steps.is_empty(),
        "governed flow proof failures: {:?}",
        failed_steps
    );

    assert_eq!(report.scenario_id, "governed_flow_v1");
    assert_eq!(report.deduped_events, 1);
    assert_eq!(report.coalesced_batches, 2);

    assert_eq!(report.affected_artifact_ids, vec!["art-001"]);
    assert_eq!(report.unaffected_artifact_ids, vec!["art-002"]);
    assert_eq!(report.affected_packet_ids, vec!["pkt-001"]);
    assert_eq!(report.unaffected_packet_ids, vec!["pkt-002"]);

    assert_eq!(report.initial_artifact_freshness, "fresh");
    assert_eq!(report.final_artifact_freshness, "invalidated");
    assert_eq!(report.initial_packet_admissibility, "admissible");
    assert_eq!(report.final_packet_admissibility, "not_admissible");

    assert!(
        report
            .steps
            .iter()
            .any(|step| step.step == "source_invalidation" && step.passed)
    );
    assert!(
        report
            .steps
            .iter()
            .any(|step| step.step == "authority_invalidation" && step.passed)
    );

    assert!(report.remediation_required);
    assert_eq!(report.remediation_count, 2);
    assert_eq!(report.triggering_event_ids.len(), 2);
    assert!(report.passed());
}

#[test]
fn governed_flow_mutation_detects_broken_artifact_and_packet_state() {
    let report = run_governed_flow_proof(Path::new("."));
    assert!(
        report.passed(),
        "baseline governed flow should be green before mutation"
    );

    let mut artifact = base_artifact();
    artifact.freshness_state = FreshnessState::Invalidated;
    artifact.admissibility_state = AdmissibilityState::Admissible;

    let artifact_err = validate_artifact_state(&artifact).unwrap_err();
    assert!(
        artifact_err.contains("invalidated") || artifact_err.contains("not_admissible"),
        "unexpected artifact validation error: {}",
        artifact_err
    );

    let mut packet = base_packet();
    packet.reevaluation_required = true;

    let packet_err = validate_packet_state(&packet).unwrap_err();
    assert!(
        packet_err.contains("reevaluation") || packet_err.contains("not_admissible"),
        "unexpected packet validation error: {}",
        packet_err
    );
}

#[test]
fn source_deleted_trigger_invalidates_artifact_and_downgrades_packet() {
    let artifact = base_artifact();
    let packet = base_packet();

    let event = EventRecord {
        event_id: "evt-source-deleted-001".into(),
        event_type: EventType::SourceDeleted,
        schema_version: "1.0".into(),
        emitted_at: "2026-04-09T00:00:05-04:00".into(),
        emitter_service: "repo-watcher".into(),
        repo_id: "forge-command".into(),
        related_artifact_ids: vec!["art-001".into()],
        related_packet_ids: vec!["pkt-001".into()],
        source_refs: vec!["src-tauri/src".into()],
        causation_id: None,
        correlation_id: "corr-source-deleted-001".into(),
        idempotency_key: "idem-source-deleted-001".into(),
        event_payload: "source deleted for governed flow coverage".into(),
    };

    let artifact_outcome = apply_artifact_invalidation(&event, &artifact);

    assert!(artifact_outcome.overlapped);
    assert_eq!(
        artifact_outcome.decision,
        ArtifactInvalidationDecision::Invalidated
    );
    assert_eq!(
        artifact_outcome.artifact.freshness_state,
        FreshnessState::Invalidated
    );
    assert_eq!(
        artifact_outcome.artifact.admissibility_state,
        AdmissibilityState::NotAdmissible
    );
    assert!(artifact_outcome.remediation_required);

    let packet_outcome =
        apply_packet_constituent_change(&packet, &[artifact_outcome.artifact.clone()]);

    assert_eq!(packet_outcome.affected_artifact_ids, vec!["art-001"]);
    assert!(packet_outcome.reevaluation_required);
    assert!(packet_outcome.remediation_required);
    assert!(packet_outcome.packet.reevaluation_required);
    assert_eq!(
        packet_outcome.packet.admissibility_state,
        AdmissibilityState::NotAdmissible
    );
}

#[test]
fn authority_record_changed_invalidates_artifact_and_downgrades_packet() {
    let artifact = base_artifact();
    let packet = base_packet();

    let event = EventRecord {
        event_id: "evt-authority-record-changed-001".into(),
        event_type: EventType::AuthorityRecordChanged,
        schema_version: "1.0".into(),
        emitted_at: "2026-04-10T01:00:05-04:00".into(),
        emitter_service: "authority-governance".into(),
        repo_id: "forge-command".into(),
        related_artifact_ids: vec!["art-001".into()],
        related_packet_ids: vec!["pkt-001".into()],
        source_refs: vec!["doc/system/fcSYSTEM.md".into()],
        causation_id: None,
        correlation_id: "corr-authority-record-changed-001".into(),
        idempotency_key: "idem-authority-record-changed-001".into(),
        event_payload: "authority precedence changed for governed flow coverage".into(),
    };

    let artifact_outcome = apply_artifact_invalidation(&event, &artifact);

    assert!(artifact_outcome.overlapped);
    assert_ne!(
        artifact_outcome.artifact.freshness_state,
        FreshnessState::Fresh
    );
    assert_ne!(
        artifact_outcome.artifact.admissibility_state,
        AdmissibilityState::Admissible
    );
    assert!(artifact_outcome.remediation_required);

    let packet_outcome =
        apply_packet_constituent_change(&packet, &[artifact_outcome.artifact.clone()]);

    assert_eq!(packet_outcome.affected_artifact_ids, vec!["art-001"]);
    assert!(packet_outcome.reevaluation_required);
    assert!(packet_outcome.remediation_required);
    assert!(packet_outcome.packet.reevaluation_required);
    assert_ne!(
        packet_outcome.packet.admissibility_state,
        AdmissibilityState::Admissible
    );
}