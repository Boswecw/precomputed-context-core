use std::fs;
use std::path::{Path, PathBuf};

use jsonschema::JSONSchema;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct SchemaValidationReport {
    pub label: String,
    pub fixture_path: PathBuf,
    pub schema_path: PathBuf,
    pub passed: bool,
    pub detail: String,
}

#[derive(Debug, Clone, Copy)]
struct FixtureSchemaMap {
    label: &'static str,
    fixture_rel_path: &'static str,
    schema_rel_path: &'static str,
}

const FIXTURE_SCHEMA_MAPS: &[FixtureSchemaMap] = &[
    FixtureSchemaMap {
        label: "authority_resolution_record_valid_shape",
        fixture_rel_path: "fixtures/valid/authority_resolution_record_forgecommand.json",
        schema_rel_path: "schemas/authority_resolution_record.schema.json",
    },
    FixtureSchemaMap {
        label: "repo_navigation_map_valid_shape",
        fixture_rel_path: "fixtures/valid/repo_navigation_map.json",
        schema_rel_path: "schemas/repo_navigation_map_contract.schema.json",
    },
    FixtureSchemaMap {
        label: "key_file_packet_valid_shape",
        fixture_rel_path: "fixtures/valid/key_file_packet.json",
        schema_rel_path: "schemas/key_file_packet_contract.schema.json",
    },
    FixtureSchemaMap {
        label: "validation_command_packet_valid_shape",
        fixture_rel_path: "fixtures/valid/validation_command_packet.json",
        schema_rel_path: "schemas/validation_command_packet_contract.schema.json",
    },
    FixtureSchemaMap {
        label: "repo_navigation_assist_packet_valid_shape",
        fixture_rel_path: "fixtures/valid/repo_navigation_assist_packet.json",
        schema_rel_path: "schemas/repo_navigation_assist_packet_contract.schema.json",
    },
    FixtureSchemaMap {
        label: "event_record_valid_shape",
        fixture_rel_path: "fixtures/valid/invalidation_event_source_move.json",
        schema_rel_path: "schemas/event_record.schema.json",
    },
    FixtureSchemaMap {
        label: "remediation_item_valid_shape",
        fixture_rel_path: "fixtures/valid/remediation_item_source_move.json",
        schema_rel_path: "schemas/remediation_item.schema.json",
    },
    FixtureSchemaMap {
        label: "override_record_valid_shape",
        fixture_rel_path: "fixtures/valid/override_record_controlled_packet_admission.json",
        schema_rel_path: "schemas/override_record.schema.json",
    },
    FixtureSchemaMap {
        label: "authority_resolution_record_invalid_semantic_but_shape_valid",
        fixture_rel_path: "fixtures/invalid/authority_resolution_record_disallowed_overlap.json",
        schema_rel_path: "schemas/authority_resolution_record.schema.json",
    },
    FixtureSchemaMap {
        label: "repo_navigation_map_invalid_semantic_but_shape_valid",
        fixture_rel_path: "fixtures/invalid/repo_navigation_map_candidate_admissible.json",
        schema_rel_path: "schemas/repo_navigation_map_contract.schema.json",
    },
    FixtureSchemaMap {
        label: "key_file_packet_invalid_semantic_but_shape_valid",
        fixture_rel_path: "fixtures/invalid/key_file_packet_hash_mismatch.json",
        schema_rel_path: "schemas/key_file_packet_contract.schema.json",
    },
    FixtureSchemaMap {
        label: "validation_command_packet_invalid_semantic_but_shape_valid",
        fixture_rel_path: "fixtures/invalid/validation_command_packet_invalidated_admissible.json",
        schema_rel_path: "schemas/validation_command_packet_contract.schema.json",
    },
    FixtureSchemaMap {
        label: "repo_navigation_assist_packet_invalid_semantic_but_shape_valid",
        fixture_rel_path:
            "fixtures/invalid/repo_navigation_assist_packet_missing_constituent_gate.json",
        schema_rel_path: "schemas/repo_navigation_assist_packet_contract.schema.json",
    },
    FixtureSchemaMap {
        label: "event_record_invalid_semantic_but_shape_valid",
        fixture_rel_path: "fixtures/invalid/invalidation_event_missing_idempotency_key.json",
        schema_rel_path: "schemas/event_record.schema.json",
    },
];

fn read_json_value(path: &Path) -> Result<Value, String> {
    let raw = fs::read_to_string(path)
        .map_err(|err| format!("failed to read {}: {}", path.display(), err))?;
    serde_json::from_str::<Value>(&raw)
        .map_err(|err| format!("failed to parse {}: {}", path.display(), err))
}

fn validate_one(root: &Path, map: FixtureSchemaMap) -> SchemaValidationReport {
    let fixture_path = root.join(map.fixture_rel_path);
    let schema_path = root.join(map.schema_rel_path);

    if !fixture_path.exists() {
        return SchemaValidationReport {
            label: map.label.into(),
            fixture_path,
            schema_path,
            passed: false,
            detail: "fixture file missing".into(),
        };
    }

    if !schema_path.exists() {
        return SchemaValidationReport {
            label: map.label.into(),
            fixture_path,
            schema_path,
            passed: false,
            detail: "schema file missing".into(),
        };
    }

    let schema_value = match read_json_value(&schema_path) {
        Ok(value) => value,
        Err(err) => {
            return SchemaValidationReport {
                label: map.label.into(),
                fixture_path,
                schema_path,
                passed: false,
                detail: err,
            };
        }
    };

    let fixture_value = match read_json_value(&fixture_path) {
        Ok(value) => value,
        Err(err) => {
            return SchemaValidationReport {
                label: map.label.into(),
                fixture_path,
                schema_path,
                passed: false,
                detail: err,
            };
        }
    };

    let compiled = match JSONSchema::compile(&schema_value) {
        Ok(compiled) => compiled,
        Err(err) => {
            return SchemaValidationReport {
                label: map.label.into(),
                fixture_path,
                schema_path,
                passed: false,
                detail: format!("schema compilation failed: {}", err),
            };
        }
    };

    let validation_result = compiled.validate(&fixture_value);

    let report = match validation_result {
        Ok(()) => SchemaValidationReport {
            label: map.label.into(),
            fixture_path,
            schema_path,
            passed: true,
            detail: "fixture matches schema shape".into(),
        },
        Err(errors) => {
            let details: Vec<String> = errors.take(3).map(|err| err.to_string()).collect();
            SchemaValidationReport {
                label: map.label.into(),
                fixture_path,
                schema_path,
                passed: false,
                detail: format!("schema validation failed: {}", details.join(" | ")),
            }
        }
    };

    report
}

pub fn run_schema_validation(root: &Path) -> Vec<SchemaValidationReport> {
    FIXTURE_SCHEMA_MAPS
        .iter()
        .copied()
        .map(|map| validate_one(root, map))
        .collect()
}

pub fn bundle_passed(reports: &[SchemaValidationReport]) -> bool {
    reports.iter().all(|report| report.passed)
}
