//! Model Downloader

use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use sha2::{Digest, Sha256};
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

pub fn download_with_progress(url: &str, dest: &Path, expected_sha256: &str) -> Result<()> {
    let client = Client::new();

    // Get content length first
    let head = client.head(url).send()?;
    let total_size = head.content_length().unwrap_or(0);

    // Create progress bar
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .expect("valid template")
        .progress_chars("█▉▊▋▌▍▎▏  "));

    // Download with streaming
    let mut response = client.get(url).send()?;
    let mut file = File::create(dest)?;
    let mut hasher = Sha256::new();
    let mut downloaded: u64 = 0;
    let mut buffer = [0u8; 8192];

    loop {
        let bytes_read = response.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        file.write_all(&buffer[..bytes_read])?;
        hasher.update(&buffer[..bytes_read]);
        downloaded += bytes_read as u64;
        pb.set_position(downloaded);
    }

    pb.finish_with_message("Download complete");

    // Verify checksum
    let hash = hasher.finalize();
    let actual_sha256 = hex::encode(hash);

    if !expected_sha256.is_empty() && actual_sha256 != expected_sha256 {
        fs::remove_file(dest)?;
        anyhow::bail!(
            "Checksum mismatch for {}: expected {}, got {}",
            dest.display(),
            expected_sha256,
            actual_sha256
        );
    }

    tracing::info!("Verified checksum for {}", dest.display());
    Ok(())
}

pub fn verify_checksum(path: &Path, expected_sha256: &str) -> Result<bool> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let hash = hasher.finalize();
    let actual_sha256 = hex::encode(hash);
    Ok(actual_sha256 == expected_sha256)
}
