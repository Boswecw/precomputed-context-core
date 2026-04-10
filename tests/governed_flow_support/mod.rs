use precomputed_context_core::{EventRecord, EventType, GovernedFlowReport};

pub fn source_deleted_event() -> EventRecord {
    EventRecord {
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
    }
}

pub fn authority_record_changed_event() -> EventRecord {
    EventRecord {
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
    }
}

pub fn assert_governed_flow_report(report: &GovernedFlowReport) {
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
            .any(|step| step.step == "source_invalidation" && step.passed),
        "missing passing source_invalidation step"
    );
    assert!(
        report
            .steps
            .iter()
            .any(|step| step.step == "authority_invalidation" && step.passed),
        "missing passing authority_invalidation step"
    );
    assert!(
        report
            .steps
            .iter()
            .any(|step| step.step == "packet_reevaluation" && step.passed),
        "missing passing packet_reevaluation step"
    );
    assert!(
        report
            .steps
            .iter()
            .any(|step| step.step == "remediation_planning" && step.passed),
        "missing passing remediation_planning step"
    );

    assert!(report.remediation_required);
    assert_eq!(report.remediation_count, 2);
    assert_eq!(report.triggering_event_ids.len(), 2);
    assert!(report.passed());
}