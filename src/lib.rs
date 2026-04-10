pub mod authority;
pub mod contracts;
pub mod enums;
pub mod events;
pub mod fixture_bundle;
pub mod invalidation_engine;
pub mod models;
pub mod proof;
pub mod proof_bundle;
pub mod remediation_flow;
pub mod schema_bundle;
pub mod schema_validation;
pub mod state_machine;

pub use authority::AuthorityResolutionRecord;
pub use contracts::{
    KeyFilePacketContract, RepoNavigationAssistPacketContract, RepoNavigationMapContract,
    ValidationCommandPacketContract,
};
pub use enums::*;
pub use events::{EventBatch, EventLedger, EventProcessingDecision, EventRecord};
pub use invalidation_engine::{
    apply_artifact_invalidation, apply_packet_constituent_change, ArtifactInvalidationDecision,
    ArtifactInvalidationOutcome, PacketInvalidationOutcome,
};
pub use models::{ArtifactRecord, OverrideRecord, PacketRecord, RemediationItem};
pub use proof::report::{GovernedFlowReport, GovernedFlowStep};
pub use proof::scenario::run_governed_flow_proof;
pub use remediation_flow::{
    plan_artifact_remediation, plan_packet_remediation, remediation_required_for_packet,
    RemediationPlan, RemediationTrigger,
};
pub use state_machine::{
    can_transition_artifact_lifecycle, can_transition_freshness,
    compute_default_artifact_admissibility, compute_default_packet_admissibility,
    validate_artifact_state, validate_packet_state,
};
