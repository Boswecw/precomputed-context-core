use std::path::Path;

use precomputed_context_core::{run_governed_flow_proof, run_replay_scenario_proof};

fn main() {
    let governed_report = run_governed_flow_proof(Path::new("."));

    println!("scenario {}", governed_report.scenario_id);

    for step in &governed_report.steps {
        let status = if step.passed { "PASS" } else { "FAIL" };
        println!("[{}] {} :: {}", status, step.step, step.detail);
    }

    println!(
        "affected_artifacts {}",
        governed_report.affected_artifact_ids.join(",")
    );
    println!(
        "unaffected_artifacts {}",
        governed_report.unaffected_artifact_ids.join(",")
    );
    println!(
        "affected_packets {}",
        governed_report.affected_packet_ids.join(",")
    );
    println!(
        "unaffected_packets {}",
        governed_report.unaffected_packet_ids.join(",")
    );
    println!(
        "triggering_events {}",
        governed_report.triggering_event_ids.join(",")
    );
    println!("deduped_events {}", governed_report.deduped_events);
    println!("coalesced_batches {}", governed_report.coalesced_batches);
    println!(
        "artifact_freshness {} -> {}",
        governed_report.initial_artifact_freshness, governed_report.final_artifact_freshness
    );
    println!(
        "packet_admissibility {} -> {}",
        governed_report.initial_packet_admissibility, governed_report.final_packet_admissibility
    );
    println!(
        "remediation_required {}",
        governed_report.remediation_required
    );
    println!("remediation_count {}", governed_report.remediation_count);

    if !governed_report.schema_paths.is_empty() {
        for path in &governed_report.schema_paths {
            println!("schema {}", path.display());
        }
    }

    let replay_result = match run_replay_scenario_proof(Path::new(".")) {
        Ok(result) => {
            println!("slice12_replay_bundle_id {}", result.replay_bundle_id);
            println!("slice12_proof_root {}", result.proof_root.display());
            println!("slice12_event_receipts {}", result.event_receipt_count);
            println!(
                "slice12_artifact_invalidations {}",
                result.artifact_invalidation_count
            );
            println!(
                "slice12_packet_reevaluations {}",
                result.packet_reevaluation_count
            );
            println!("slice12_remediations {}", result.remediation_count);
            println!("slice12_replay_mismatches {}", result.mismatch_count);
            println!("slice12_replay_equivalent {}", result.replay_ok);
            result
        }
        Err(err) => {
            println!("slice12_replay_proof_error {}", err);
            std::process::exit(1);
        }
    };

    if governed_report.passed() && replay_result.replay_ok {
        println!("governed flow proof passed");
        println!("slice12 replay proof passed");
        std::process::exit(0);
    }

    println!("governed flow proof failed");
    println!("slice12 replay proof failed");
    std::process::exit(1);
}
