use precomputed_context_core::{
    proof::{
        base_artifact_record, base_authority_record as proof_base_authority_record,
        base_packet_record,
    },
    ArtifactRecord, AuthorityResolutionRecord, PacketRecord,
};

#[allow(dead_code)]
pub fn base_authority_record() -> AuthorityResolutionRecord {
    proof_base_authority_record()
}

pub fn base_artifact() -> ArtifactRecord {
    base_artifact_record("art-001", "src-tauri/src")
}

pub fn base_packet() -> PacketRecord {
    base_packet_record("pkt-001", "art-001")
}