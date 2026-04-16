use crate::authorization_evidence::load_authorization_evidence_link;
use crate::import_authorization::load_authorization_receipt;
use crate::import_policy::hash_import_authorization_policy_file;
use crate::trust_envelope::{
    load_signed_trust_envelope, sha256_hex_file, verify_signed_trust_envelope,
    verify_zip_against_envelope,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

pub const REHYDRATE_GATE_RECEIPT_SCHEMA_VERSION: &str = "proof.rehydrate-gate-receipt.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RehydrateGateReceipt {
    pub schema_version: String,
    pub policy_sha256: String,
    pub trust_envelope_sha256: String,
    pub authorization_receipt_sha256: String,
    pub evidence_link_sha256: String,
    pub import_receipt_sha256: String,
    pub package_file_name: String,
    pub package_sha256: String,
    pub authorized: bool,
    pub decision: String,
}

pub fn validate_rehydrate_gate(
    zip_path: &Path,
    policy_path: &Path,
    trust_envelope_path: &Path,
    authorization_receipt_path: &Path,
    evidence_link_path: &Path,
    import_receipt_path: &Path,
) -> Result<RehydrateGateReceipt, Box<dyn Error>> {
    if !authorization_receipt_path.exists() {
        return Err(format!(
            "authorization receipt missing: {}",
            authorization_receipt_path.display()
        )
        .into());
    }

    if !evidence_link_path.exists() {
        return Err(format!(
            "authorization evidence link missing: {}",
            evidence_link_path.display()
        )
        .into());
    }

    if !import_receipt_path.exists() {
        return Err(format!(
            "import receipt missing for gated publication: {}",
            import_receipt_path.display()
        )
        .into());
    }

    let policy_sha256 = hash_import_authorization_policy_file(policy_path)?;
    let trust_envelope_sha256 = sha256_hex_file(trust_envelope_path)?;
    let authorization_receipt_sha256 = sha256_hex_file(authorization_receipt_path)?;
    let evidence_link_sha256 = sha256_hex_file(evidence_link_path)?;
    let import_receipt_sha256 = sha256_hex_file(import_receipt_path)?;

    let authorization_receipt = load_authorization_receipt(authorization_receipt_path)?;
    if !authorization_receipt.authorized {
        return Err("authorization receipt is not authorized".into());
    }

    if authorization_receipt.policy_sha256 != policy_sha256 {
        return Err(format!(
            "authorization receipt policy hash mismatch: expected={} actual={}",
            policy_sha256, authorization_receipt.policy_sha256
        )
        .into());
    }

    if authorization_receipt.trust_envelope_sha256 != trust_envelope_sha256 {
        return Err(format!(
            "authorization receipt envelope hash mismatch: expected={} actual={}",
            trust_envelope_sha256, authorization_receipt.trust_envelope_sha256
        )
        .into());
    }

    let evidence = load_authorization_evidence_link(evidence_link_path)?;
    if evidence.policy_sha256 != policy_sha256 {
        return Err(format!(
            "authorization evidence policy hash mismatch: expected={} actual={}",
            policy_sha256, evidence.policy_sha256
        )
        .into());
    }

    if evidence.trust_envelope_sha256 != trust_envelope_sha256 {
        return Err(format!(
            "authorization evidence envelope hash mismatch: expected={} actual={}",
            trust_envelope_sha256, evidence.trust_envelope_sha256
        )
        .into());
    }

    if evidence.authorization_receipt_sha256 != authorization_receipt_sha256 {
        return Err(format!(
            "authorization evidence receipt hash mismatch: expected={} actual={}",
            authorization_receipt_sha256, evidence.authorization_receipt_sha256
        )
        .into());
    }

    if evidence.import_receipt_sha256.as_deref() != Some(import_receipt_sha256.as_str()) {
        return Err(format!(
            "authorization evidence import receipt hash mismatch: expected={} actual={:?}",
            import_receipt_sha256, evidence.import_receipt_sha256
        )
        .into());
    }

    let trust_envelope = load_signed_trust_envelope(trust_envelope_path)?;
    verify_signed_trust_envelope(&trust_envelope)?;
    verify_zip_against_envelope(zip_path, &trust_envelope)?;

    let actual_package_file_name = zip_path
        .file_name()
        .ok_or("zip path has no file name")?
        .to_string_lossy()
        .to_string();
    if authorization_receipt.package_file_name != actual_package_file_name {
        return Err(format!(
            "authorization receipt package file mismatch: expected={} actual={}",
            actual_package_file_name, authorization_receipt.package_file_name
        )
        .into());
    }

    if authorization_receipt.package_sha256 != trust_envelope.package_sha256 {
        return Err(format!(
            "authorization receipt package sha mismatch: expected={} actual={}",
            trust_envelope.package_sha256, authorization_receipt.package_sha256
        )
        .into());
    }

    Ok(RehydrateGateReceipt {
        schema_version: REHYDRATE_GATE_RECEIPT_SCHEMA_VERSION.to_string(),
        policy_sha256,
        trust_envelope_sha256,
        authorization_receipt_sha256,
        evidence_link_sha256,
        import_receipt_sha256,
        package_file_name: actual_package_file_name,
        package_sha256: trust_envelope.package_sha256,
        authorized: true,
        decision: "authorized_rehydrate_publication".to_string(),
    })
}

pub fn default_rehydrate_gate_workspace_current() -> PathBuf {
    PathBuf::from("target/proof_artifacts/slice20_rehydrate_gate/current")
}

pub fn default_rehydrate_gate_receipt_path(workspace_current_dir: &Path) -> PathBuf {
    workspace_current_dir.join("gate_receipt.json")
}

pub fn default_gated_import_receipt_path(workspace_current_dir: &Path) -> PathBuf {
    workspace_current_dir.join("import_receipt.json")
}

pub fn publish_authorized_import_receipt(
    source_import_receipt_path: &Path,
    workspace_current_dir: &Path,
    gate_receipt: &RehydrateGateReceipt,
) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(workspace_current_dir)?;
    let gated_import_receipt_path = default_gated_import_receipt_path(workspace_current_dir);
    fs::copy(source_import_receipt_path, &gated_import_receipt_path)?;
    write_rehydrate_gate_receipt(
        &default_rehydrate_gate_receipt_path(workspace_current_dir),
        gate_receipt,
    )?;
    Ok(())
}

pub fn write_rehydrate_gate_receipt(
    path: &Path,
    receipt: &RehydrateGateReceipt,
) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let bytes = serde_json::to_vec_pretty(receipt)?;
    fs::write(path, bytes)?;
    Ok(())
}

pub fn load_rehydrate_gate_receipt(path: &Path) -> Result<RehydrateGateReceipt, Box<dyn Error>> {
    let bytes = fs::read(path)?;
    let receipt: RehydrateGateReceipt = serde_json::from_slice(&bytes)?;
    Ok(receipt)
}
