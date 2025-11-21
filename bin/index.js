#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');

const binaryPath = path.join(__dirname, '..', 'target', 'release', "envgo");
const args = process.argv.slice(2);

spawn(binaryPath, args, {
  stdio: 'inherit',
  env: process.env
});
