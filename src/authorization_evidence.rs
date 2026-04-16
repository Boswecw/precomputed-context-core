use crate::trust_envelope::sha256_hex_file;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

pub const AUTHORIZATION_EVIDENCE_SCHEMA_VERSION: &str = "proof.authorization-evidence-link.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuthorizationEvidenceLink {
    pub schema_version: String,
    pub policy_file_name: String,
    pub policy_sha256: String,
    pub trust_envelope_file_name: String,
    pub trust_envelope_sha256: String,
    pub authorization_receipt_file_name: String,
    pub authorization_receipt_sha256: String,
    pub import_receipt_present: bool,
    pub import_receipt_file_name: Option<String>,
    pub import_receipt_sha256: Option<String>,
}

pub fn default_authorization_evidence_link_path() -> PathBuf {
    PathBuf::from("target/proof_artifacts/slice19_policy/current/authorization_evidence_link.json")
}

pub fn default_import_receipt_path() -> PathBuf {
    PathBuf::from("target/proof_artifacts/slice16_import/current/import_receipt.json")
}

pub fn build_authorization_evidence_link(
    policy_path: &Path,
    trust_envelope_path: &Path,
    authorization_receipt_path: &Path,
    import_receipt_path: Option<&Path>,
) -> Result<AuthorizationEvidenceLink, Box<dyn Error>> {
    let import_receipt_path = import_receipt_path.filter(|value| value.exists());

    Ok(AuthorizationEvidenceLink {
        schema_version: AUTHORIZATION_EVIDENCE_SCHEMA_VERSION.to_string(),
        policy_file_name: file_name_string(policy_path)?,
        policy_sha256: sha256_hex_file(policy_path)?,
        trust_envelope_file_name: file_name_string(trust_envelope_path)?,
        trust_envelope_sha256: sha256_hex_file(trust_envelope_path)?,
        authorization_receipt_file_name: file_name_string(authorization_receipt_path)?,
        authorization_receipt_sha256: sha256_hex_file(authorization_receipt_path)?,
        import_receipt_present: import_receipt_path.is_some(),
        import_receipt_file_name: import_receipt_path.map(file_name_string).transpose()?,
        import_receipt_sha256: import_receipt_path.map(sha256_hex_file).transpose()?,
    })
}

pub fn write_authorization_evidence_link(
    path: &Path,
    evidence: &AuthorizationEvidenceLink,
) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let bytes = serde_json::to_vec_pretty(evidence)?;
    fs::write(path, bytes)?;
    Ok(())
}

pub fn load_authorization_evidence_link(
    path: &Path,
) -> Result<AuthorizationEvidenceLink, Box<dyn Error>> {
    let bytes = fs::read(path)?;
    let evidence: AuthorizationEvidenceLink = serde_json::from_slice(&bytes)?;
    Ok(evidence)
}

fn file_name_string(path: &Path) -> Result<String, Box<dyn Error>> {
    Ok(path
        .file_name()
        .ok_or("path has no file name")?
        .to_string_lossy()
        .to_string())
}
