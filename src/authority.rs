use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::enums::{ArtifactClass, RepoArchetype, SourceFamily};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AuthorityResolutionRecord {
    pub schema_version: String,
    pub repo_id: String,
    pub repo_name: String,
    pub repo_archetype: RepoArchetype,
    pub authority_order: Vec<SourceFamily>,
    pub approved_source_families: Vec<SourceFamily>,
    pub disallowed_source_families: Vec<SourceFamily>,
    pub ambiguity_rules: Vec<String>,
    pub escalation_required_conditions: Vec<String>,
    pub approved_derivation_scope: Vec<ArtifactClass>,
    pub operator_review_required_conditions: Vec<String>,
    pub notes_on_known_authority_gaps: Option<String>,
    pub created_at: String,
    pub last_reviewed_at: String,
}

impl AuthorityResolutionRecord {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version.trim().is_empty() {
            return Err("schema_version is required".into());
        }
        if self.repo_id.trim().is_empty() {
            return Err("repo_id is required".into());
        }
        if self.repo_name.trim().is_empty() {
            return Err("repo_name is required".into());
        }
        if self.authority_order.is_empty() {
            return Err("authority_order is required and must not be empty".into());
        }
        if self.approved_source_families.is_empty() {
            return Err("approved_source_families is required and must not be empty".into());
        }
        if self.approved_derivation_scope.is_empty() {
            return Err("approved_derivation_scope is required and must not be empty".into());
        }
        if self.created_at.trim().is_empty() || self.last_reviewed_at.trim().is_empty() {
            return Err("created_at and last_reviewed_at are required".into());
        }

        for family in &self.disallowed_source_families {
            if self.approved_source_families.contains(family) {
                return Err("a source family cannot be both approved and disallowed".into());
            }
        }

        Ok(())
    }

    pub fn validate_derivation_request(
        &self,
        requested_artifact_class: ArtifactClass,
        requested_sources: &[SourceFamily],
    ) -> Result<(), String> {
        self.validate()?;

        if !self
            .approved_derivation_scope
            .contains(&requested_artifact_class)
        {
            return Err("requested artifact class is outside approved_derivation_scope".into());
        }

        if requested_sources.is_empty() {
            return Err("requested_sources must not be empty".into());
        }

        for source in requested_sources {
            if self.disallowed_source_families.contains(source) {
                return Err("derivation request includes a disallowed source family".into());
            }
            if !self.approved_source_families.contains(source) {
                return Err(
                    "derivation request includes a source family that is not approved".into(),
                );
            }
        }

        Ok(())
    }
}
