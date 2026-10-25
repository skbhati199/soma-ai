#!/usr/bin/env node
// postinstall.js - Runs after npm install to set up shell integration

const { execSync } = require('child_process');
const path = require('path');
const fs = require('fs');

const binDir = path.join(__dirname, '..', 'bin');
const somaPath = path.join(binDir, process.platform === 'win32' ? 'soma.exe' : 'soma');

function runCommand(cmd, options = {}) {
  try {
    execSync(cmd, { stdio: 'inherit', shell: true, ...options });
    return true;
  } catch (error) {
    return false;
  }
}

function main() {
  console.log('Setting up Soma AI...');

  // Verify binary exists
  if (!fs.existsSync(somaPath)) {
    console.error('Binary not found at:', somaPath);
    console.error('Run `npm run install` first');
    process.exit(1);
  }

  // Make executable on Unix
  if (process.platform !== 'win32') {
    fs.chmodSync(somaPath, 0o755);
  }

  // Test binary
  console.log('Testing binary...');
  if (!runCommand(`"${somaPath}" --version`)) {
    console.error('Binary test failed');
    process.exit(1);
  }

  console.log('✓ Soma AI installed successfully!');
  console.log('');
  console.log('Next steps:');
  console.log('  1. Add to PATH (optional):');
  console.log(`     export PATH="\$PATH:${binDir}"`);
  console.log('  2. Install shell integration:');
  console.log(`     ${path.basename(somaPath)} install`);
  console.log('  3. Restart your shell');
  console.log('');
  console.log('Quick test:');
  console.log(`  ${path.basename(somaPath)} ask "list all files"`);
}

main();
