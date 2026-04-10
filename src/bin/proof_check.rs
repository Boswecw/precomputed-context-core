use std::path::Path;

use precomputed_context_core::run_governed_flow_proof;

fn main() {
    let report = run_governed_flow_proof(Path::new("."));

    println!("scenario {}", report.scenario_id);

    for step in &report.steps {
        let status = if step.passed { "PASS" } else { "FAIL" };
        println!("[{}] {} :: {}", status, step.step, step.detail);
    }

    println!(
        "affected_artifacts {}",
        report.affected_artifact_ids.join(",")
    );
    println!(
        "unaffected_artifacts {}",
        report.unaffected_artifact_ids.join(",")
    );
    println!("affected_packets {}", report.affected_packet_ids.join(","));
    println!(
        "unaffected_packets {}",
        report.unaffected_packet_ids.join(",")
    );
    println!(
        "triggering_events {}",
        report.triggering_event_ids.join(",")
    );
    println!("deduped_events {}", report.deduped_events);
    println!("coalesced_batches {}", report.coalesced_batches);
    println!(
        "artifact_freshness {} -> {}",
        report.initial_artifact_freshness, report.final_artifact_freshness
    );
    println!(
        "packet_admissibility {} -> {}",
        report.initial_packet_admissibility, report.final_packet_admissibility
    );
    println!("remediation_required {}", report.remediation_required);
    println!("remediation_count {}", report.remediation_count);

    if !report.schema_paths.is_empty() {
        for path in &report.schema_paths {
            println!("schema {}", path.display());
        }
    }

    if report.passed() {
        println!("governed flow proof passed");
        std::process::exit(0);
    }

    println!("governed flow proof failed");
    std::process::exit(1);
}
