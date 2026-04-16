use crate::import_gate::{default_gated_import_receipt_path, load_rehydrate_gate_receipt};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

pub const PROMOTION_APPROVAL_SCHEMA_VERSION: &str = "proof.promotion-approval.v1";
pub const PROMOTION_RECEIPT_SCHEMA_VERSION: &str = "proof.promotion-receipt.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PromotionApproval {
    pub schema_version: String,
    pub operator_id: String,
    pub action: String,
    pub gate_receipt_sha256: String,
    pub gated_import_receipt_sha256: String,
    pub approved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PromotionReceipt {
    pub schema_version: String,
    pub operator_id: String,
    pub action: String,
    pub gate_receipt_sha256: String,
    pub gated_import_receipt_sha256: String,
    pub approved: bool,
    pub decision: String,
}

pub fn default_operator_id() -> &'static str {
    "charlie"
}

pub fn build_promotion_approval(
    gate_receipt_path: &Path,
    gated_import_receipt_path: &Path,
) -> Result<PromotionApproval, Box<dyn Error>> {
    Ok(PromotionApproval {
        schema_version: PROMOTION_APPROVAL_SCHEMA_VERSION.to_string(),
        operator_id: default_operator_id().to_string(),
        action: "promote_authorized_import_receipt".to_string(),
        gate_receipt_sha256: sha256_hex_file(gate_receipt_path)?,
        gated_import_receipt_sha256: sha256_hex_file(gated_import_receipt_path)?,
        approved: true,
    })
}

pub fn validate_promotion_approval(
    gate_receipt_path: &Path,
    gated_import_receipt_path: &Path,
    approval_path: &Path,
) -> Result<PromotionReceipt, Box<dyn Error>> {
    if !gate_receipt_path.exists() {
        return Err(format!("gate receipt missing: {}", gate_receipt_path.display()).into());
    }

    if !gated_import_receipt_path.exists() {
        return Err(format!(
            "gated import receipt missing: {}",
            gated_import_receipt_path.display()
        )
        .into());
    }

    if !approval_path.exists() {
        return Err(format!("promotion approval missing: {}", approval_path.display()).into());
    }

    let approval = load_promotion_approval(approval_path)?;
    if !approval.approved {
        return Err("promotion approval is not approved".into());
    }

    let gate_receipt = load_rehydrate_gate_receipt(gate_receipt_path)?;
    if !gate_receipt.authorized {
        return Err("rehydrate gate receipt is not authorized".into());
    }

    let expected_gate_receipt_sha256 = sha256_hex_file(gate_receipt_path)?;
    if approval.gate_receipt_sha256 != expected_gate_receipt_sha256 {
        return Err(format!(
            "promotion approval gate hash mismatch: expected={} actual={}",
            expected_gate_receipt_sha256, approval.gate_receipt_sha256
        )
        .into());
    }

    let expected_gated_import_receipt_sha256 = sha256_hex_file(gated_import_receipt_path)?;
    if approval.gated_import_receipt_sha256 != expected_gated_import_receipt_sha256 {
        return Err(format!(
            "promotion approval gated import receipt hash mismatch: expected={} actual={}",
            expected_gated_import_receipt_sha256, approval.gated_import_receipt_sha256
        )
        .into());
    }

    Ok(PromotionReceipt {
        schema_version: PROMOTION_RECEIPT_SCHEMA_VERSION.to_string(),
        operator_id: approval.operator_id,
        action: approval.action,
        gate_receipt_sha256: expected_gate_receipt_sha256,
        gated_import_receipt_sha256: expected_gated_import_receipt_sha256,
        approved: true,
        decision: "approved_promotion".to_string(),
    })
}

pub fn default_promotion_workspace_current() -> PathBuf {
    PathBuf::from("target/proof_artifacts/slice21_promotion/current")
}

pub fn default_promotion_approval_path() -> PathBuf {
    PathBuf::from("target/proof_artifacts/slice21_promotion/operator_approval.json")
}

pub fn default_promotion_receipt_path(workspace_current_dir: &Path) -> PathBuf {
    workspace_current_dir.join("promotion_receipt.json")
}

pub fn default_promoted_import_receipt_path(workspace_current_dir: &Path) -> PathBuf {
    workspace_current_dir.join("import_receipt.json")
}

pub fn publish_promoted_import_receipt(
    source_gated_import_receipt_path: &Path,
    workspace_current_dir: &Path,
    promotion_receipt: &PromotionReceipt,
) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(workspace_current_dir)?;
    let destination = default_promoted_import_receipt_path(workspace_current_dir);
    fs::copy(source_gated_import_receipt_path, &destination)?;
    write_promotion_receipt(
        &default_promotion_receipt_path(workspace_current_dir),
        promotion_receipt,
    )?;
    Ok(())
}

pub fn write_promotion_approval(
    path: &Path,
    approval: &PromotionApproval,
) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, serde_json::to_vec_pretty(approval)?)?;
    Ok(())
}

pub fn load_promotion_approval(path: &Path) -> Result<PromotionApproval, Box<dyn Error>> {
    let bytes = fs::read(path)?;
    let approval: PromotionApproval = serde_json::from_slice(&bytes)?;
    Ok(approval)
}

pub fn write_promotion_receipt(
    path: &Path,
    receipt: &PromotionReceipt,
) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, serde_json::to_vec_pretty(receipt)?)?;
    Ok(())
}

pub fn load_promotion_receipt(path: &Path) -> Result<PromotionReceipt, Box<dyn Error>> {
    let bytes = fs::read(path)?;
    let receipt: PromotionReceipt = serde_json::from_slice(&bytes)?;
    Ok(receipt)
}

pub fn default_slice20_gate_receipt_path() -> PathBuf {
    PathBuf::from("target/proof_artifacts/slice20_rehydrate_gate/current/gate_receipt.json")
}

pub fn default_slice20_gated_import_receipt_path() -> PathBuf {
    PathBuf::from("target/proof_artifacts/slice20_rehydrate_gate/current/import_receipt.json")
}

pub fn default_slice20_gated_workspace_current() -> PathBuf {
    PathBuf::from("target/proof_artifacts/slice20_rehydrate_gate/current")
}

pub fn default_slice20_gated_import_receipt_path_from_workspace() -> PathBuf {
    default_gated_import_receipt_path(&default_slice20_gated_workspace_current())
}

pub fn sha256_hex_file(path: &Path) -> Result<String, Box<dyn Error>> {
    let bytes = fs::read(path)?;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let digest = hasher.finalize();
    Ok(to_hex(&digest))
}

fn to_hex(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        use std::fmt::Write as _;
        let _ = write!(&mut output, "{:02x}", byte);
    }
    output
}
