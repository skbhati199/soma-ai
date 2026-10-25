#!/usr/bin/env node
// test.js - Simple test to verify the package works

const { execSync } = require('child_process');
const path = require('path');

const binDir = path.join(__dirname, '..', 'bin');
const somaPath = path.join(binDir, process.platform === 'win32' ? 'soma.exe' : 'soma');

function run(cmd) {
  return execSync(cmd, { encoding: 'utf8', shell: true }).trim();
}

function main() {
  console.log('Running tests...\n');

  // Test 1: Version
  console.log('Test 1: Version');
  const version = run(`"${somaPath}" --version`);
  console.log(`  Version: ${version}`);
  console.log('  ✓ Pass\n');

  // Test 2: Help
  console.log('Test 2: Help');
  const help = run(`"${somaPath}" --help`);
  console.log(`  Help output: ${help.length} chars`);
  console.log('  ✓ Pass\n');

  // Test 3: Config show
  console.log('Test 3: Config show');
  const config = run(`"${somaPath}" config show`);
  console.log(`  Config: ${config.length} chars`);
  console.log('  ✓ Pass\n');

  // Test 4: Model list
  console.log('Test 4: Model list');
  const models = run(`"${somaPath}" model list`);
  console.log(`  Models: ${models}`);
  console.log('  ✓ Pass\n');

  console.log('All tests passed! ✓');
}

main();