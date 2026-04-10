use precomputed_context_core::*;

#[allow(dead_code)]
pub fn base_authority_record() -> AuthorityResolutionRecord {
    AuthorityResolutionRecord {
        schema_version: "1.0".into(),
        repo_id: "forge-command".into(),
        repo_name: "ForgeCommand".into(),
        repo_archetype: RepoArchetype::DesktopAppRepo,
        authority_order: vec![
            SourceFamily::ContractSchema,
            SourceFamily::CodeRuntime,
            SourceFamily::TestVerification,
            SourceFamily::RepoTruthDoc,
            SourceFamily::ProtocolDoc,
        ],
        approved_source_families: vec![
            SourceFamily::CodeRuntime,
            SourceFamily::ContractSchema,
            SourceFamily::TestVerification,
            SourceFamily::RepoTruthDoc,
            SourceFamily::ProtocolDoc,
        ],
        disallowed_source_families: vec![
            SourceFamily::GeneratedOutput,
            SourceFamily::AdvisoryNote,
        ],
        ambiguity_rules: vec![
            "code_vs_repo_truth_doc_conflict_blocks".into(),
            "protocol_vs_runtime_conflict_requires_review".into(),
        ],
        escalation_required_conditions: vec!["authority_conflict_on_required_truth".into()],
        approved_derivation_scope: vec![
            ArtifactClass::RepoNavigationMap,
            ArtifactClass::KeyFilePacket,
            ArtifactClass::ValidationCommandPacket,
        ],
        operator_review_required_conditions: vec!["authority_ambiguity".into()],
        notes_on_known_authority_gaps: Some("Initial proof-slice record.".into()),
        created_at: "2026-04-09T00:00:00-04:00".into(),
        last_reviewed_at: "2026-04-09T00:00:00-04:00".into(),
    }
}

pub fn base_artifact() -> ArtifactRecord {
    ArtifactRecord {
        schema_version: "1.0".into(),
        artifact_id: "art-001".into(),
        artifact_class: ArtifactClass::RepoNavigationMap,
        repo_id: "forge-command".into(),
        title: "ForgeCommand Repo Navigation".into(),
        operational_purpose: "Provide first-wave repo structure".into(),
        summary_block: "Bounded navigational summary".into(),
        source_refs: vec!["src-tauri/src".into(), "doc/fcSYSTEM.md".into()],
        source_ref_hashes: vec!["hash-1".into(), "hash-2".into()],
        authority_level: AuthorityLevel::Canonical,
        lifecycle_state: LifecycleState::Approved,
        freshness_state: FreshnessState::Fresh,
        critic_status: CriticStatus::Passed,
        admissibility_state: AdmissibilityState::Admissible,
        related_artifact_refs: vec![],
        supersedes_artifact_id: None,
        protocol_refs: vec!["BDS_BACKEND_ENGINEERING_PROTOCOL".into()],
        created_at: "2026-04-09T00:00:00-04:00".into(),
        last_validated_at: "2026-04-09T00:00:00-04:00".into(),
        producer_identity: "proof-slice-core".into(),
        sensitivity_classification: SensitivityClassification::InternalGeneral,
    }
}

pub fn base_packet() -> PacketRecord {
    PacketRecord {
        schema_version: "1.0".into(),
        packet_id: "pkt-001".into(),
        packet_role: PacketRole::RepoNavigationAssist,
        repo_id: "forge-command".into(),
        included_artifact_ids: vec!["art-001".into()],
        included_artifact_hashes: vec!["hash-art-001".into()],
        packet_constraints: vec!["bounded".into()],
        packet_budget_band: "small".into(),
        lane_compatibility: vec!["neuroforge".into(), "neuronforge".into()],
        lifecycle_state: PacketLifecycleState::Approved,
        admissibility_state: AdmissibilityState::Admissible,
        created_at: "2026-04-09T00:00:00-04:00".into(),
        last_evaluated_at: "2026-04-09T00:00:00-04:00".into(),
        required_constituents_present: true,
        reevaluation_required: false,
        sensitivity_classification: SensitivityClassification::InternalGeneral,
    }
}