use std::fs;
use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;

use crate::authority::AuthorityResolutionRecord;
use crate::contracts::{
    KeyFilePacketContract, RepoNavigationAssistPacketContract, RepoNavigationMapContract,
    ValidationCommandPacketContract,
};
use crate::events::EventRecord;
use crate::models::{OverrideRecord, RemediationItem};

#[derive(Debug, Clone)]
pub struct FixtureReport {
    pub label: String,
    pub path: PathBuf,
    pub passed: bool,
    pub detail: String,
}

const EXPECTED_VALID: &[&str] = &[
    "authority_resolution_record_forgecommand.json",
    "repo_navigation_map.json",
    "key_file_packet.json",
    "validation_command_packet.json",
    "repo_navigation_assist_packet.json",
    "invalidation_event_source_move.json",
    "remediation_item_source_move.json",
    "override_record_controlled_packet_admission.json",
];

const EXPECTED_INVALID: &[&str] = &[
    "authority_resolution_record_disallowed_overlap.json",
    "repo_navigation_map_candidate_admissible.json",
    "key_file_packet_hash_mismatch.json",
    "validation_command_packet_invalidated_admissible.json",
    "repo_navigation_assist_packet_missing_constituent_gate.json",
    "invalidation_event_missing_idempotency_key.json",
];

fn read_json<T: DeserializeOwned>(path: &Path) -> Result<T, String> {
    let raw = fs::read_to_string(path)
        .map_err(|err| format!("failed to read {}: {}", path.display(), err))?;
    serde_json::from_str::<T>(&raw)
        .map_err(|err| format!("failed to parse {}: {}", path.display(), err))
}

fn inventory_reports(dir: &Path, expected: &[&str], label_prefix: &str) -> Vec<FixtureReport> {
    let mut reports = Vec::new();

    let actual_entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(err) => {
            reports.push(FixtureReport {
                label: format!("{}_inventory", label_prefix),
                path: dir.to_path_buf(),
                passed: false,
                detail: format!("failed to read fixture directory: {}", err),
            });
            return reports;
        }
    };

    let mut actual_files: Vec<String> = actual_entries
        .filter_map(Result::ok)
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect();

    actual_files.sort();

    for expected_name in expected {
        let path = dir.join(expected_name);
        reports.push(FixtureReport {
            label: format!("{}_required_file", label_prefix),
            path: path.clone(),
            passed: path.exists(),
            detail: if path.exists() {
                "required fixture present".into()
            } else {
                "required fixture missing".into()
            },
        });
    }

    for actual in actual_files {
        let known = expected
            .iter()
            .any(|expected_name| *expected_name == actual);
        reports.push(FixtureReport {
            label: format!("{}_unexpected_file_check", label_prefix),
            path: dir.join(&actual),
            passed: known,
            detail: if known {
                "fixture file is in expected inventory".into()
            } else {
                "unexpected fixture file present".into()
            },
        });
    }

    reports
}

fn check_valid<T, F>(label: &str, path: PathBuf, validate: F) -> FixtureReport
where
    T: DeserializeOwned,
    F: FnOnce(&T) -> Result<(), String>,
{
    match read_json::<T>(&path).and_then(|value| validate(&value)) {
        Ok(()) => FixtureReport {
            label: label.into(),
            path,
            passed: true,
            detail: "valid fixture accepted".into(),
        },
        Err(err) => FixtureReport {
            label: label.into(),
            path,
            passed: false,
            detail: err,
        },
    }
}

fn check_invalid<T, F>(label: &str, path: PathBuf, validate: F) -> FixtureReport
where
    T: DeserializeOwned,
    F: FnOnce(&T) -> Result<(), String>,
{
    match read_json::<T>(&path) {
        Ok(value) => match validate(&value) {
            Ok(()) => FixtureReport {
                label: label.into(),
                path,
                passed: false,
                detail: "invalid fixture was accepted".into(),
            },
            Err(err) => FixtureReport {
                label: label.into(),
                path,
                passed: true,
                detail: format!("invalid fixture rejected: {}", err),
            },
        },
        Err(err) => FixtureReport {
            label: label.into(),
            path,
            passed: true,
            detail: format!("invalid fixture failed parse: {}", err),
        },
    }
}

pub fn run_fixture_bundle(root: &Path) -> Vec<FixtureReport> {
    let valid = root.join("fixtures").join("valid");
    let invalid = root.join("fixtures").join("invalid");

    let mut reports = Vec::new();
    reports.extend(inventory_reports(&valid, EXPECTED_VALID, "valid"));
    reports.extend(inventory_reports(&invalid, EXPECTED_INVALID, "invalid"));

    reports.extend(vec![
        check_valid::<AuthorityResolutionRecord, _>(
            "valid_authority_resolution_record",
            valid.join("authority_resolution_record_forgecommand.json"),
            |value| value.validate(),
        ),
        check_valid::<RepoNavigationMapContract, _>(
            "valid_repo_navigation_map",
            valid.join("repo_navigation_map.json"),
            |value| value.validate(),
        ),
        check_valid::<KeyFilePacketContract, _>(
            "valid_key_file_packet",
            valid.join("key_file_packet.json"),
            |value| value.validate(),
        ),
        check_valid::<ValidationCommandPacketContract, _>(
            "valid_validation_command_packet",
            valid.join("validation_command_packet.json"),
            |value| value.validate(),
        ),
        check_valid::<RepoNavigationAssistPacketContract, _>(
            "valid_repo_navigation_assist_packet",
            valid.join("repo_navigation_assist_packet.json"),
            |value| value.validate(),
        ),
        check_valid::<EventRecord, _>(
            "valid_invalidation_event",
            valid.join("invalidation_event_source_move.json"),
            |value| value.validate(),
        ),
        check_valid::<RemediationItem, _>(
            "valid_remediation_item",
            valid.join("remediation_item_source_move.json"),
            |value| value.validate(),
        ),
        check_valid::<OverrideRecord, _>(
            "valid_override_record",
            valid.join("override_record_controlled_packet_admission.json"),
            |value| value.validate(),
        ),
        check_invalid::<AuthorityResolutionRecord, _>(
            "invalid_authority_resolution_record",
            invalid.join("authority_resolution_record_disallowed_overlap.json"),
            |value| value.validate(),
        ),
        check_invalid::<RepoNavigationMapContract, _>(
            "invalid_repo_navigation_map",
            invalid.join("repo_navigation_map_candidate_admissible.json"),
            |value| value.validate(),
        ),
        check_invalid::<KeyFilePacketContract, _>(
            "invalid_key_file_packet",
            invalid.join("key_file_packet_hash_mismatch.json"),
            |value| value.validate(),
        ),
        check_invalid::<ValidationCommandPacketContract, _>(
            "invalid_validation_command_packet",
            invalid.join("validation_command_packet_invalidated_admissible.json"),
            |value| value.validate(),
        ),
        check_invalid::<RepoNavigationAssistPacketContract, _>(
            "invalid_repo_navigation_assist_packet",
            invalid.join("repo_navigation_assist_packet_missing_constituent_gate.json"),
            |value| value.validate(),
        ),
        check_invalid::<EventRecord, _>(
            "invalid_invalidation_event",
            invalid.join("invalidation_event_missing_idempotency_key.json"),
            |value| value.validate(),
        ),
    ]);

    reports
}

pub fn bundle_passed(reports: &[FixtureReport]) -> bool {
    reports.iter().all(|report| report.passed)
}
