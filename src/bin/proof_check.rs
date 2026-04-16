use std::env;
use std::path::Path;

use precomputed_context_core::{
    export_proof_package, run_governed_flow_proof, run_replay_scenario_proof,
};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if let Some(code) = handle_export_mode(&args) {
        std::process::exit(code);
    }

    run_console_proof();
}

fn handle_export_mode(args: &[String]) -> Option<i32> {
    if args.is_empty() {
        return None;
    }

    let export_requested = args.iter().any(|arg| arg == "--export-package");
    let unknown_args: Vec<&String> = args
        .iter()
        .filter(|arg| arg.as_str() != "--export-package")
        .collect();

    if !unknown_args.is_empty() {
        eprintln!(
            "unknown argument(s): {}",
            unknown_args
                .iter()
                .map(|arg| arg.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
        return Some(1);
    }

    if !export_requested {
        return None;
    }

    match export_proof_package(Path::new(".")) {
        Ok(report) => {
            println!(
                "slice13_export_package_root {}",
                report.package_root.display()
            );
            println!(
                "slice13_export_replay_bundle_id {}",
                report.replay_bundle_id
            );
            println!("slice13_export_written true");
            Some(0)
        }
        Err(err) => {
            eprintln!("slice13_export_error {}", err);
            Some(1)
        }
    }
}

fn run_console_proof() {
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
