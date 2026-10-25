//! Embedded Model Support (for air-gapped builds)

use anyhow::Result;
use std::path::Path;

/// Extract embedded model to filesystem
/// Used when building with `embedded-model` feature
#[cfg(feature = "embedded-model")]
pub fn extract_embedded_model(dest: &Path) -> Result<()> {
    const EMBEDDED_MODEL: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/model.gguf"));

    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)?;
    }

    std::fs::write(dest, EMBEDDED_MODEL)?;
    tracing::info!("Extracted embedded model to {}", dest.display());
    Ok(())
}

/// Check if embedded model is available
#[cfg(feature = "embedded-model")]
pub fn has_embedded_model() -> bool {
    true
}

#[cfg(not(feature = "embedded-model"))]
pub fn has_embedded_model() -> bool {
    false
}

#[cfg(not(feature = "embedded-model"))]
pub fn extract_embedded_model(_dest: &Path) -> Result<()> {
    anyhow::bail!("Embedded model feature not enabled. Build with --features embedded-model");
}