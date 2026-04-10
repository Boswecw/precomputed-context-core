use std::path::Path;

use precomputed_context_core::proof_bundle::run_proof_bundle;

fn main() {
    let report = run_proof_bundle(Path::new("."));

    for step in &report.steps {
        let status = if step.passed { "PASS" } else { "FAIL" };
        println!("[{}] {} :: {}", status, step.step, step.detail);
    }

    if !report.schema_paths.is_empty() {
        for path in &report.schema_paths {
            println!("schema {}", path.display());
        }
    }

    if report.passed() {
        println!("proof bundle passed");
        std::process::exit(0);
    }

    println!("proof bundle failed");
    std::process::exit(1);
}