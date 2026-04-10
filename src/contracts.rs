use serde::{Deserialize, Serialize};

use crate::enums::ArtifactClass;
use crate::models::{ArtifactRecord, PacketRecord};
use crate::state_machine::{validate_artifact_state, validate_packet_state};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepoNavigationMapContract {
    pub base: ArtifactRecord,
    pub primary_directories: Vec<String>,
    pub entry_points: Vec<String>,
    pub canonical_docs: Vec<String>,
    pub build_test_commands: Vec<String>,
}

impl RepoNavigationMapContract {
    pub fn validate(&self) -> Result<(), String> {
        if self.base.artifact_class != ArtifactClass::RepoNavigationMap {
            return Err("repo_navigation_map base artifact_class mismatch".into());
        }
        validate_artifact_state(&self.base)?;
        if self.primary_directories.is_empty() {
            return Err("primary_directories must not be empty".into());
        }
        if self.entry_points.is_empty() {
            return Err("entry_points must not be empty".into());
        }
        if self.canonical_docs.is_empty() {
            return Err("canonical_docs must not be empty".into());
        }
        if self.build_test_commands.is_empty() {
            return Err("build_test_commands must not be empty".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeyFilePacketContract {
    pub base: ArtifactRecord,
    pub file_path: String,
    pub why_it_matters: String,
    pub dependent_surfaces: Vec<String>,
    pub edit_cautions: Vec<String>,
    pub read_before_edit_refs: Vec<String>,
}

impl KeyFilePacketContract {
    pub fn validate(&self) -> Result<(), String> {
        if self.base.artifact_class != ArtifactClass::KeyFilePacket {
            return Err("key_file_packet base artifact_class mismatch".into());
        }
        validate_artifact_state(&self.base)?;
        if self.file_path.trim().is_empty() {
            return Err("file_path is required".into());
        }
        if self.why_it_matters.trim().is_empty() {
            return Err("why_it_matters is required".into());
        }
        if self.dependent_surfaces.is_empty() {
            return Err("dependent_surfaces must not be empty".into());
        }
        if self.edit_cautions.is_empty() {
            return Err("edit_cautions must not be empty".into());
        }
        if self.read_before_edit_refs.is_empty() {
            return Err("read_before_edit_refs must not be empty".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationCommandPacketContract {
    pub base: ArtifactRecord,
    pub commands: Vec<String>,
    pub execution_order: Vec<String>,
    pub expected_pass_conditions: Vec<String>,
    pub environment_requirements: Vec<String>,
}

impl ValidationCommandPacketContract {
    pub fn validate(&self) -> Result<(), String> {
        if self.base.artifact_class != ArtifactClass::ValidationCommandPacket {
            return Err("validation_command_packet base artifact_class mismatch".into());
        }
        validate_artifact_state(&self.base)?;
        if self.commands.is_empty() {
            return Err("commands must not be empty".into());
        }
        if self.execution_order.is_empty() {
            return Err("execution_order must not be empty".into());
        }
        if self.expected_pass_conditions.is_empty() {
            return Err("expected_pass_conditions must not be empty".into());
        }
        if self.environment_requirements.is_empty() {
            return Err("environment_requirements must not be empty".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepoNavigationAssistPacketContract {
    pub base: PacketRecord,
}

impl RepoNavigationAssistPacketContract {
    pub fn validate(&self) -> Result<(), String> {
        validate_packet_state(&self.base)
    }
}
