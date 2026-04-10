mod common;

use std::path::Path;

use common::{base_artifact, base_packet};
use precomputed_context_core::{
    run_governed_flow_proof, validate_artifact_state, validate_packet_state,
    AdmissibilityState, FreshnessState,
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
    assert_eq!(report.coalesced_batches, 1);

    assert_eq!(report.affected_artifact_ids, vec!["art-001"]);
    assert_eq!(report.unaffected_artifact_ids, vec!["art-002"]);
    assert_eq!(report.affected_packet_ids, vec!["pkt-001"]);
    assert_eq!(report.unaffected_packet_ids, vec!["pkt-002"]);

    assert_eq!(report.initial_artifact_freshness, "fresh");
    assert_eq!(report.final_artifact_freshness, "invalidated");
    assert_eq!(report.initial_packet_admissibility, "admissible");
    assert_eq!(report.final_packet_admissibility, "not_admissible");

    assert!(report.remediation_required);
    assert_eq!(report.remediation_count, 2);
    assert_eq!(report.triggering_event_ids.len(), 2);
    assert!(report.passed());
}

#[test]
fn governed_flow_mutation_detects_broken_artifact_and_packet_state() {
    let report = run_governed_flow_proof(Path::new("."));
    assert!(report.passed(), "baseline governed flow should be green before mutation");

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