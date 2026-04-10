use std::path::Path;

use precomputed_context_core::schema_bundle::export_schemas;

fn main() {
    match export_schemas(Path::new(".")) {
        Ok(paths) => {
            for path in paths {
                println!("wrote {}", path.display());
            }
        }
        Err(err) => {
            eprintln!("schema export failed: {}", err);
            std::process::exit(1);
        }
    }
}
