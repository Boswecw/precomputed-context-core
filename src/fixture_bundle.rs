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

fn read_json<T: DeserializeOwned>(path: &Path) -> Result<T, String> {
    let raw = fs::read_to_string(path)
        .map_err(|err| format!("failed to read {}: {}", path.display(), err))?;
    serde_json::from_str::<T>(&raw)
        .map_err(|err| format!("failed to parse {}: {}", path.display(), err))
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

    vec![
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
    ]
}

pub fn bundle_passed(reports: &[FixtureReport]) -> bool {
    reports.iter().all(|report| report.passed)
}
