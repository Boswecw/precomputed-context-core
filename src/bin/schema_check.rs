use std::path::Path;

use precomputed_context_core::schema_validation::{bundle_passed, run_schema_validation};

fn main() {
    let reports = run_schema_validation(Path::new("."));

    for report in &reports {
        let status = if report.passed { "PASS" } else { "FAIL" };
        println!("[{}] {} :: {}", status, report.label, report.detail);
    }

    if bundle_passed(&reports) {
        println!("schema validation passed");
        std::process::exit(0);
    }

    println!("schema validation failed");
    std::process::exit(1);
}