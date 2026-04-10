use std::path::{Path, PathBuf};

use crate::fixture_bundle;
use crate::schema_bundle;
use crate::schema_validation;

#[derive(Debug, Clone)]
pub struct ProofStepReport {
    pub step: String,
    pub passed: bool,
    pub detail: String,
}

#[derive(Debug, Clone)]
pub struct ProofBundleReport {
    pub schema_paths: Vec<PathBuf>,
    pub steps: Vec<ProofStepReport>,
}

impl ProofBundleReport {
    pub fn passed(&self) -> bool {
        self.steps.iter().all(|step| step.passed)
    }
}

pub fn run_proof_bundle(root: &Path) -> ProofBundleReport {
    let mut steps = Vec::new();

    let schema_paths = match schema_bundle::export_schemas(root) {
        Ok(paths) => {
            steps.push(ProofStepReport {
                step: "export_schemas".into(),
                passed: true,
                detail: format!("exported {} schema files", paths.len()),
            });
            paths
        }
        Err(err) => {
            steps.push(ProofStepReport {
                step: "export_schemas".into(),
                passed: false,
                detail: err,
            });

            return ProofBundleReport {
                schema_paths: Vec::new(),
                steps,
            };
        }
    };

    let schema_reports = schema_validation::run_schema_validation(root);
    let schema_passed = schema_validation::bundle_passed(&schema_reports);
    let schema_failures: Vec<String> = schema_reports
        .iter()
        .filter(|report| !report.passed)
        .map(|report| format!("{} :: {}", report.label, report.detail))
        .collect();

    steps.push(ProofStepReport {
        step: "schema_validation".into(),
        passed: schema_passed,
        detail: if schema_passed {
            format!("validated {} fixture/schema pairs", schema_reports.len())
        } else {
            schema_failures.join(" | ")
        },
    });

    let fixture_reports = fixture_bundle::run_fixture_bundle(root);
    let fixture_passed = fixture_bundle::bundle_passed(&fixture_reports);
    let fixture_failures: Vec<String> = fixture_reports
        .iter()
        .filter(|report| !report.passed)
        .map(|report| format!("{} :: {}", report.label, report.detail))
        .collect();

    steps.push(ProofStepReport {
        step: "fixture_semantic_validation".into(),
        passed: fixture_passed,
        detail: if fixture_passed {
            format!("validated {} fixture checks", fixture_reports.len())
        } else {
            fixture_failures.join(" | ")
        },
    });

    ProofBundleReport {
        schema_paths,
        steps,
    }
}
