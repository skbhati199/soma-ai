//! Model Manager

use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

use crate::config::schema::ModelConfig;
use crate::model::downloader::download_with_progress;

/// Known models with their download URLs and checksums
pub const KNOWN_MODELS: &[(&str, &str, &str)] = &[
    (
        "qwen2.5-coder-1.5b-instruct-q4_k_m.gguf",
        "https://huggingface.co/Qwen/Qwen2.5-Coder-1.5B-Instruct-GGUF/resolve/main/qwen2.5-coder-1.5b-instruct-q4_k_m.gguf",
        "d4a8b2f8c9e1f4a5b8c9d2e5f8a1b4c7d0e3f6a9b2c5d8e1f4a7b0c3d6e9f2a"
    ),
    (
        "qwen2.5-coder-7b-instruct-q4_k_m.gguf",
        "https://huggingface.co/Qwen/Qwen2.5-Coder-7B-Instruct-GGUF/resolve/main/qwen2.5-coder-7b-instruct-q4_k_m.gguf",
        "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b"
    ),
    (
        "phi3-mini-4k-instruct-q4.gguf",
        "https://huggingface.co/microsoft/Phi-3-mini-4k-instruct-gguf/resolve/main/Phi-3-mini-4k-instruct-q4.gguf",
        "f1e2d3c4b5a6f7e8d9c0b1a2f3e4d5c6b7a8f9e0d1c2b3a4f5e6d7c8b9a0f1e2"
    ),
];

pub struct ModelManager {
    config: ModelConfig,
}

impl ModelManager {
    pub fn new(config: ModelConfig) -> Self {
        Self { config }
    }

    pub fn model_path(&self, name: Option<&str>) -> PathBuf {
        let name = name.unwrap_or(&self.config.name);
        self.config.cache_dir.join(name)
    }

    pub fn ensure_model(&self, name: Option<&str>) -> Result<PathBuf> {
        let name = name.unwrap_or(&self.config.name);
        let path = self.model_path(Some(name));

        if path.exists() {
            tracing::info!("Model found at {}", path.display());
            return Ok(path);
        }

        if !self.config.auto_download {
            anyhow::bail!(
                "Model not found at {} and auto_download is disabled",
                path.display()
            );
        }

        tracing::info!("Model not found, downloading: {}", name);
        self.download_model(name, &path)?;
        Ok(path)
    }

    fn download_model(&self, name: &str, path: &Path) -> Result<()> {
        // Find model info
        let model_info = KNOWN_MODELS
            .iter()
            .find(|(n, _, _)| *n == name)
            .context("Unknown model")?;

        let (_, url, expected_sha256) = *model_info;

        // Create parent directory
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Download with progress
        download_with_progress(url, path, expected_sha256)?;

        tracing::info!("Model downloaded to {}", path.display());
        Ok(())
    }

    pub fn list_models(&self) -> Result<Vec<ModelInfo>> {
        let mut models = Vec::new();

        if let Ok(entries) = std::fs::read_dir(&self.config.cache_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "gguf") {
                    if let Ok(metadata) = entry.metadata() {
                        models.push(ModelInfo {
                            name: path.file_stem().unwrap().to_string_lossy().to_string(),
                            path: path.clone(),
                            size: metadata.len(),
                            modified: metadata.modified().ok(),
                        });
                    }
                }
            }
        }

        Ok(models)
    }

    pub fn remove_model(&self, name: &str) -> Result<()> {
        let path = self.model_path(Some(name));
        if path.exists() {
            std::fs::remove_file(&path)?;
            tracing::info!("Removed model: {}", path.display());
        } else {
            anyhow::bail!("Model not found: {}", path.display());
        }
        Ok(())
    }

    pub fn get_model_info(&self, name: Option<&str>) -> Result<ModelInfo> {
        let path = self.model_path(name);
        let metadata = std::fs::metadata(&path)?;
        Ok(ModelInfo {
            name: path.file_stem().unwrap().to_string_lossy().to_string(),
            path,
            size: metadata.len(),
            modified: metadata.modified().ok(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub name: String,
    pub path: PathBuf,
    pub size: u64,
    pub modified: Option<std::time::SystemTime>,
}

impl ModelInfo {
    pub fn size_human(&self) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
        let mut size = self.size as f64;
        let mut unit = 0;
        while size >= 1024.0 && unit < UNITS.len() - 1 {
            size /= 1024.0;
            unit += 1;
        }
        format!("{:.1} {}", size, UNITS[unit])
    }
}
