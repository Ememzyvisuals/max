#!/usr/bin/env node
// scripts/install.js — MAX postinstall binary downloader
// Created by Ememzyvisuals (Emmanuel Ariyo)

'use strict';

const https = require('https');
const fs = require('fs');
const path = require('path');
const os = require('os');
const { execSync } = require('child_process');
const zlib = require('zlib');

const GITHUB_REPO = 'ememzyvisuals/max';
const PKG_VERSION = require('../package.json').version;
const BASE_URL = `https://github.com/${GITHUB_REPO}/releases/download/v${PKG_VERSION}`;
const BIN_DIR = path.join(__dirname, '..', '.bin');

// ── Platform detection ──────────────────────────────────────────────────────

function getTarget() {
  const platform = os.platform();
  const arch = os.arch();

  const platformMap = {
    linux: {
      x64: { artifact: 'max-linux-x86_64.tar.gz', binary: 'max-linux-x86_64' },
      arm64: { artifact: 'max-linux-arm64.tar.gz', binary: 'max-linux-arm64' },
    },
    darwin: {
      x64: { artifact: 'max-macos-x86_64.tar.gz', binary: 'max-macos-x86_64' },
      arm64: { artifact: 'max-macos-arm64.tar.gz', binary: 'max-macos-arm64' },
    },
    win32: {
      x64: { artifact: 'max-windows-x86_64.zip', binary: 'max-windows-x86_64.exe' },
    },
  };

  const plat = platformMap[platform];
  if (!plat) {
    throw new Error(`Unsupported platform: ${platform}`);
  }

  const target = plat[arch];
  if (!target) {
    throw new Error(`Unsupported architecture: ${arch} on ${platform}`);
  }

  return { platform, arch, ...target };
}

// ── Download helpers ────────────────────────────────────────────────────────

function download(url, dest) {
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(dest);

    function get(url) {
      https.get(url, (res) => {
        // Follow redirects (GitHub releases use 302)
        if (res.statusCode === 301 || res.statusCode === 302) {
          return get(res.headers.location);
        }
        if (res.statusCode !== 200) {
          return reject(new Error(`Download failed: HTTP ${res.statusCode} for ${url}`));
        }
        res.pipe(file);
        file.on('finish', () => file.close(resolve));
        file.on('error', (err) => {
          fs.unlink(dest, () => reject(err));
        });
      }).on('error', (err) => {
        fs.unlink(dest, () => reject(err));
      });
    }

    get(url);
  });
}

function extractTarGz(archivePath, destDir) {
  return new Promise((resolve, reject) => {
    const gunzip = zlib.createGunzip();
    const tar = require('child_process').spawn('tar', ['-xzf', archivePath, '-C', destDir]);
    tar.on('close', (code) => {
      if (code === 0) resolve();
      else reject(new Error(`tar exited with code ${code}`));
    });
    tar.on('error', reject);
  });
}

function extractZip(archivePath, destDir) {
  // Use PowerShell on Windows
  execSync(
    `powershell -Command "Expand-Archive -Path '${archivePath}' -DestinationPath '${destDir}' -Force"`,
    { stdio: 'inherit' }
  );
}

// ── Main install flow ────────────────────────────────────────────────────────

async function install() {
  console.log('\n  MAX — Installing binary...');
  console.log('  Created by Ememzyvisuals (Emmanuel Ariyo)');
  console.log('  https://github.com/ememzyvisuals/max\n');

  let target;
  try {
    target = getTarget();
  } catch (err) {
    console.error(`  [ERROR] ${err.message}`);
    console.error('  Please install manually: https://github.com/ememzyvisuals/max/releases');
    process.exit(1);
  }

  console.log(`  Platform: ${target.platform} (${target.arch})`);
  console.log(`  Artifact: ${target.artifact}`);
  console.log(`  Version:  v${PKG_VERSION}\n`);

  if (!fs.existsSync(BIN_DIR)) {
    fs.mkdirSync(BIN_DIR, { recursive: true });
  }

  const artifactUrl = `${BASE_URL}/${target.artifact}`;
  const archivePath = path.join(BIN_DIR, target.artifact);
  const finalBinName = os.platform() === 'win32' ? 'max.exe' : 'max';
  const finalBinPath = path.join(BIN_DIR, finalBinName);

  // Download
  process.stdout.write('  Downloading...');
  try {
    await download(artifactUrl, archivePath);
    process.stdout.write(' ✓\n');
  } catch (err) {
    process.stdout.write(' ✗\n');
    console.error(`  [ERROR] Download failed: ${err.message}`);
    console.error(`  URL: ${artifactUrl}`);
    console.error('  Install manually: https://github.com/ememzyvisuals/max/releases');
    process.exit(1);
  }

  // Extract
  process.stdout.write('  Extracting...');
  try {
    if (target.artifact.endsWith('.tar.gz')) {
      await extractTarGz(archivePath, BIN_DIR);
    } else {
      extractZip(archivePath, BIN_DIR);
    }
    process.stdout.write(' ✓\n');
  } catch (err) {
    process.stdout.write(' ✗\n');
    console.error(`  [ERROR] Extraction failed: ${err.message}`);
    process.exit(1);
  }

  // Rename extracted binary to 'max' / 'max.exe'
  const extractedBin = path.join(BIN_DIR, target.binary);
  if (fs.existsSync(extractedBin) && extractedBin !== finalBinPath) {
    fs.renameSync(extractedBin, finalBinPath);
  }

  // Set executable permissions on Unix
  if (os.platform() !== 'win32') {
    fs.chmodSync(finalBinPath, 0o755);
  }

  // Cleanup archive
  fs.unlinkSync(archivePath);

  // Verify installation
  process.stdout.write('  Verifying...');
  try {
    const version = execSync(`"${finalBinPath}" --version`, { encoding: 'utf8' }).trim();
    process.stdout.write(' ✓\n');
    console.log(`\n  MAX ${version} installed successfully!`);
    console.log('  Run: max --help\n');
  } catch (err) {
    process.stdout.write(' ✗\n');
    console.error('  [WARN] Could not verify binary. Try running: max --version');
  }
}

install().catch((err) => {
  console.error('  [FATAL]', err.message);
  process.exit(1);
});
