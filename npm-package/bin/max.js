#!/usr/bin/env node
// bin/max.js — MAX CLI entry point (npm wrapper)
// Created by Ememzyvisuals (Emmanuel Ariyo)

'use strict';

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');
const os = require('os');

function getBinaryPath() {
  const platform = os.platform(); // linux, darwin, win32
  const arch = os.arch();         // x64, arm64

  let binaryName;
  if (platform === 'win32') {
    binaryName = 'max.exe';
  } else {
    binaryName = 'max';
  }

  // Look for binary installed by postinstall script
  const localBin = path.join(__dirname, '..', '.bin', binaryName);
  if (fs.existsSync(localBin)) {
    return localBin;
  }

  // Fallback: look in PATH
  return binaryName;
}

const bin = getBinaryPath();
const args = process.argv.slice(2);

const child = spawn(bin, args, {
  stdio: 'inherit',
  env: process.env,
});

child.on('error', (err) => {
  if (err.code === 'ENOENT') {
    console.error('\n  [MAX] Binary not found. Try reinstalling:\n');
    console.error('    npm install -g @ememzyvisuals/max\n');
    console.error('  Or install manually from GitHub Releases:');
    console.error('    https://github.com/ememzyvisuals/max/releases\n');
  } else {
    console.error('  [MAX] Error:', err.message);
  }
  process.exit(1);
});

child.on('close', (code) => {
  process.exit(code || 0);
});
