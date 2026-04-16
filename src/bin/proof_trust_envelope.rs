use precomputed_context_core::trust_envelope::write_default_trust_envelope_for_zip;
use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let zip_path = PathBuf::from("target/proof_artifacts/slice14_export.zip");
    let envelope_path = write_default_trust_envelope_for_zip(&zip_path)?;
    println!("{}", envelope_path.display());
    Ok(())
}
