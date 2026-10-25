#!/usr/bin/env node
// index.js - Entry point for npm package

const { spawnSync } = require('child_process');
const path = require('path');
const fs = require('fs');

const binDir = path.join(__dirname, 'bin');
const platformBin = process.platform === 'win32' ? 'soma.exe' : 'soma';
const somaPath = path.join(binDir, platformBin);

// Platform-specific binary names
const PLATFORM_BINS = {
  'linux-x64': 'soma-x86_64-unknown-linux-gnu',
  'linux-arm64': 'soma-aarch64-unknown-linux-gnu',
  'darwin-x64': 'soma-x86_64-apple-darwin',
  'darwin-arm64': 'soma-aarch64-apple-darwin',
  'win32-x64': 'soma-x86_64-pc-windows-msvc.exe',
};

function getPlatformKey() {
  const platform = process.platform;
  const arch = process.arch;
  return `${platform}-${arch}`;
}

function findBinary() {
  const platformKey = getPlatformKey();
  const platformBinary = PLATFORM_BINS[platformKey];
  
  // Try platform-specific binary first
  if (platformBinary) {
    const platformPath = path.join(binDir, platformBinary);
    if (fs.existsSync(platformPath)) {
      return platformPath;
    }
  }
  
  // Fallback to generic soma binary
  if (fs.existsSync(somaPath)) {
    return somaPath;
  }
  
  // Try to find any soma binary in bin dir
  try {
    const files = fs.readdirSync(binDir);
    const somaFile = files.find(f => f.startsWith('soma'));
    if (somaFile) {
      return path.join(binDir, somaFile);
    }
  } catch {}
  
  return null;
}

function main() {
  const binary = findBinary();
  
  if (!binary) {
    console.error('Soma binary not found!');
    console.error('Expected at:', binDir);
    console.error('');
    console.error('Try reinstalling:');
    console.error('  npm install -g soma-ai');
    console.error('Or run postinstall manually:');
    console.error('  npm run postinstall');
    process.exit(1);
  }

  // Make executable on Unix
  if (process.platform !== 'win32') {
    try { fs.chmodSync(binary, 0o755); } catch {}
  }

  // Spawn the binary with all arguments
  const args = process.argv.slice(2);
  const child = spawnSync(binary, args, {
    stdio: 'inherit',
    shell: false,
    windowsVerbatimArguments: true,
  });

  if (child.error) {
    console.error(`Failed to start soma: ${child.error.message}`);
    process.exit(1);
  }

  process.exit(child.status ?? 0);
}

main();
