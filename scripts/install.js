#!/usr/bin/env node
// install.js - Downloads the appropriate binary for the current platform

const https = require('https');
const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');
const os = require('os');

const VERSION = process.env.npm_package_version || '0.1.0';
const REPO = 'yourusername/soma-ai';
const BASE_URL = `https://github.com/${REPO}/releases/download/v${VERSION}`;

const PLATFORM_MAP = {
  'linux-x64': { asset: 'soma-x86_64-unknown-linux-gnu.tar.gz', binary: 'soma' },
  'linux-arm64': { asset: 'soma-aarch64-unknown-linux-gnu.tar.gz', binary: 'soma' },
  'darwin-x64': { asset: 'soma-x86_64-apple-darwin.tar.gz', binary: 'soma' },
  'darwin-arm64': { asset: 'soma-aarch64-apple-darwin.tar.gz', binary: 'soma' },
  'win32-x64': { asset: 'soma-x86_64-pc-windows-msvc.zip', binary: 'soma.exe' },
};

function getPlatformKey() {
  const platform = os.platform();
  const arch = os.arch();
  return `${platform}-${arch}`;
}

function downloadFile(url, dest) {
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(dest);
    https.get(url, (response) => {
      if (response.statusCode === 302 || response.statusCode === 301) {
        // Follow redirect
        downloadFile(response.headers.location, dest).then(resolve).catch(reject);
        return;
      }
      if (response.statusCode !== 200) {
        reject(new Error(`Failed to download: ${response.statusCode}`));
        return;
      }
      response.pipe(file);
      file.on('finish', () => file.close(resolve));
    }).on('error', (err) => {
      fs.unlink(dest, () => {});
      reject(err);
    });
  });
}

function extractArchive(archivePath, destDir) {
  const ext = path.extname(archivePath);
  if (ext === '.gz' || archivePath.endsWith('.tar.gz')) {
    execSync(`tar -xzf "${archivePath}" -C "${destDir}"`, { stdio: 'inherit' });
  } else if (ext === '.zip') {
    execSync(`unzip -o "${archivePath}" -d "${destDir}"`, { stdio: 'inherit' });
  } else {
    throw new Error(`Unknown archive format: ${archivePath}`);
  }
}

async function main() {
  const platformKey = getPlatformKey();
  const platformInfo = PLATFORM_MAP[platformKey];

  if (!platformInfo) {
    console.error(`Unsupported platform: ${platformKey}`);
    console.error('Supported platforms:', Object.keys(PLATFORM_MAP).join(', '));
    process.exit(1);
  }

  console.log(`Installing soma v${VERSION} for ${platformKey}...`);

  const assetUrl = `${BASE_URL}/${platformInfo.asset}`;
  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'soma-install-'));
  const archivePath = path.join(tempDir, platformInfo.asset);
  const binDir = path.join(__dirname, '..', 'bin');

  try {
    // Download
    console.log(`Downloading from ${assetUrl}...`);
    await downloadFile(assetUrl, archivePath);
    console.log('Download complete.');

    // Extract
    console.log('Extracting...');
    extractArchive(archivePath, tempDir);

    // Find binary
    const extractedBinary = path.join(tempDir, platformInfo.binary);
    if (!fs.existsSync(extractedBinary)) {
      // Try finding it recursively
      const files = fs.readdirSync(tempDir, { recursive: true });
      const found = files.find(f => f.endsWith(platformInfo.binary));
      if (found) {
        fs.copyFileSync(path.join(tempDir, found), path.join(binDir, platformInfo.binary));
      } else {
        throw new Error(`Binary not found in archive: ${platformInfo.binary}`);
      }
    } else {
      // Ensure bin directory exists
      fs.mkdirSync(binDir, { recursive: true });
      fs.copyFileSync(extractedBinary, path.join(binDir, platformInfo.binary));
    }

    // Make executable on Unix
    if (platformInfo.binary === 'soma') {
      fs.chmodSync(path.join(binDir, 'soma'), 0o755);
    }

    console.log(`✓ soma installed to ${binDir}`);
    console.log(`Run: npx soma --help`);

  } catch (error) {
    console.error('Installation failed:', error.message);
    process.exit(1);
  } finally {
    // Cleanup
    try { fs.rmSync(tempDir, { recursive: true, force: true }); } catch {}
  }
}

main();