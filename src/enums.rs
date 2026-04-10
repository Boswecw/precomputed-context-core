#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RepoArchetype {
    ServiceApiRepo,
    DesktopAppRepo,
    ProtocolDocumentationRepo,
    SharedContractLibraryRepo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceFamily {
    CodeRuntime,
    ContractSchema,
    TestVerification,
    RepoTruthDoc,
    ProtocolDoc,
    ArchitectureDecision,
    GeneratedOutput,
    AdvisoryNote,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArtifactClass {
    RepoNavigationMap,
    KeyFilePacket,
    ValidationCommandPacket,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PacketRole {
    RepoNavigationAssist,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuthorityLevel {
    Canonical,
    StrongDerived,
    WeakDerived,
    Provisional,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LifecycleState {
    Candidate,
    Approved,
    Superseded,
    Blocked,
    Retired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PacketLifecycleState {
    Candidate,
    Approved,
    Blocked,
    Retired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FreshnessState {
    Fresh,
    ReviewDue,
    Stale,
    Invalidated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CriticStatus {
    NotReviewed,
    Passed,
    PassedWithConcerns,
    Failed,
    RemediationRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AdmissibilityState {
    Admissible,
    AdmissibleWithWarning,
    Restricted,
    NotAdmissible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RemediationStatus {
    None,
    Open,
    Triaged,
    InRepair,
    Resolved,
    ClosedWithoutRepair,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SensitivityClassification {
    InternalGeneral,
    InternalSensitive,
    RestrictedRuntime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BlockingStatus {
    NonBlocking,
    WarningOnly,
    Blocking,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AffectedObjectType {
    Artifact,
    Packet,
    AuthorityRecord,
    SourceRef,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventType {
    SourceChanged,
    SourceMoved,
    SourceDeleted,
    SourceValidationChanged,
    AuthorityRecordChanged,
    ArtifactCandidateCreated,
    ArtifactValidationPassed,
    ArtifactValidationFailed,
    ArtifactApproved,
    ArtifactBlocked,
    ArtifactFreshnessChanged,
    ArtifactInvalidated,
    ArtifactSuperseded,
    RemediationCreated,
    RemediationTriaged,
    RemediationRepairStarted,
    RemediationResolved,
    RemediationClosedWithoutRepair,
    PacketCandidateCreated,
    PacketComposed,
    PacketApproved,
    PacketInvalidated,
    PacketRevalidated,
    PacketAdmissibilityChanged,
    OperatorOverrideCreated,
    OperatorOverrideExpired,
    OperatorOverrideRevoked,
    GovernanceRuleChanged,
}
