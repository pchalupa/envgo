#!/usr/bin/env node

import { spawn } from 'child_process';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const path = join(__dirname, '..', 'target', 'release', "envgo");
const args = process.argv.slice(2);

spawn(path, args, {
  stdio: 'inherit',
  env: process.env
});
