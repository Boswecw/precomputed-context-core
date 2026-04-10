use std::path::Path;

use precomputed_context_core::schema_validation::{bundle_passed, run_schema_validation};

fn main() {
    let reports = run_schema_validation(Path::new("."));

    for report in &reports {
        let status = if report.passed { "PASS" } else { "FAIL" };
        println!(
            "[{}] {} :: fixture={} :: schema={} :: {}",
            status,
            report.label,
            report.fixture_path.display(),
            report.schema_path.display(),
            report.detail
        );
    }

    if bundle_passed(&reports) {
        println!("schema validation bundle passed");
        std::process::exit(0);
    }

    println!("schema validation bundle failed");
    std::process::exit(1);
}
