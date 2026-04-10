use crate::enums::{
    AdmissibilityState, AffectedObjectType, ArtifactClass, AuthorityLevel, BlockingStatus,
    CriticStatus, FreshnessState, LifecycleState, PacketLifecycleState, PacketRole,
    RemediationStatus, SensitivityClassification, Severity,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactRecord {
    pub schema_version: String,
    pub artifact_id: String,
    pub artifact_class: ArtifactClass,
    pub repo_id: String,
    pub title: String,
    pub operational_purpose: String,
    pub summary_block: String,
    pub source_refs: Vec<String>,
    pub source_ref_hashes: Vec<String>,
    pub authority_level: AuthorityLevel,
    pub lifecycle_state: LifecycleState,
    pub freshness_state: FreshnessState,
    pub critic_status: CriticStatus,
    pub admissibility_state: AdmissibilityState,
    pub related_artifact_refs: Vec<String>,
    pub supersedes_artifact_id: Option<String>,
    pub protocol_refs: Vec<String>,
    pub created_at: String,
    pub last_validated_at: String,
    pub producer_identity: String,
    pub sensitivity_classification: SensitivityClassification,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PacketRecord {
    pub schema_version: String,
    pub packet_id: String,
    pub packet_role: PacketRole,
    pub repo_id: String,
    pub included_artifact_ids: Vec<String>,
    pub included_artifact_hashes: Vec<String>,
    pub packet_constraints: Vec<String>,
    pub packet_budget_band: String,
    pub lane_compatibility: Vec<String>,
    pub lifecycle_state: PacketLifecycleState,
    pub admissibility_state: AdmissibilityState,
    pub created_at: String,
    pub last_evaluated_at: String,
    pub required_constituents_present: bool,
    pub reevaluation_required: bool,
    pub sensitivity_classification: SensitivityClassification,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemediationItem {
    pub schema_version: String,
    pub remediation_id: String,
    pub repo_id: String,
    pub affected_object_type: AffectedObjectType,
    pub affected_object_ids: Vec<String>,
    pub issue_type: String,
    pub severity: Severity,
    pub blocking_status: BlockingStatus,
    pub recommended_action: String,
    pub created_at: String,
    pub status: RemediationStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverrideRecord {
    pub schema_version: String,
    pub override_id: String,
    pub actor_identity: String,
    pub reason: String,
    pub scope: String,
    pub start_time: String,
    pub expiry_time: String,
    pub affected_object_type: AffectedObjectType,
    pub affected_object_ids: Vec<String>,
    pub review_required_by: String,
    pub created_at: String,
}

impl OverrideRecord {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version.trim().is_empty() {
            return Err("schema_version is required".into());
        }
        if self.override_id.trim().is_empty() {
            return Err("override_id is required".into());
        }
        if self.actor_identity.trim().is_empty() {
            return Err("actor_identity is required".into());
        }
        if self.reason.trim().is_empty() {
            return Err("reason is required".into());
        }
        if self.scope.trim().is_empty() {
            return Err("scope is required".into());
        }
        if self.start_time.trim().is_empty() || self.expiry_time.trim().is_empty() {
            return Err("start_time and expiry_time are required".into());
        }
        if self.affected_object_ids.is_empty() {
            return Err("affected_object_ids must not be empty".into());
        }
        if self.review_required_by.trim().is_empty() || self.created_at.trim().is_empty() {
            return Err("review_required_by and created_at are required".into());
        }
        Ok(())
    }
}
