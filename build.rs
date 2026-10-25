//! Build Script

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Version from Cargo.toml
    let version = env::var("CARGO_PKG_VERSION").unwrap_or("unknown");
    println!("cargo:rustc-env=SOMA_VERSION={}", version);

    // Git commit hash
    let git_hash = get_git_hash().unwrap_or("unknown");
    println!("cargo:rustc-env=SOMA_GIT_HASH={}", git_hash);

    // Build timestamp
    let timestamp = chrono::Utc::now()
        .format("%Y-%m-%d %H:%M:%S UTC")
        .to_string();
    println!("cargo:rustc-env=SOMA_BUILD_TIME={}", timestamp);

    // Model hash (if embedded)
    #[cfg(feature = "embedded-model")]
    {
        let model_hash = compute_model_hash();
        println!("cargo:rustc-env=SOMA_MODEL_HASH={}", model_hash);
    }

    // Rerun if these files change
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=.git/HEAD");
}

fn get_git_hash() -> Option<String> {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()?;
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

#[cfg(feature = "embedded-model")]
fn compute_model_hash() -> String {
    use sha2::{Digest, Sha256};
    let model_path = PathBuf::from("model.gguf");
    if model_path.exists() {
        let content = fs::read(model_path).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(&content);
        format!("{:x}", hasher.finalize())
    } else {
        "unknown".to_string()
    }
}
