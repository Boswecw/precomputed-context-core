use std::path::Path;

use precomputed_context_core::fixture_bundle::{bundle_passed, run_fixture_bundle};

fn main() {
    let reports = run_fixture_bundle(Path::new("."));

    for report in &reports {
        let status = if report.passed { "PASS" } else { "FAIL" };
        println!("[{}] {} :: {}", status, report.label, report.detail);
    }

    if bundle_passed(&reports) {
        println!("fixture validation passed");
        std::process::exit(0);
    }

    println!("fixture validation failed");
    std::process::exit(1);
}