import { readdir } from 'node:fs/promises';
import path from 'node:path';
import { spawn } from 'node:child_process';

const roots = ['src', 'tests', 'scripts'];
const files = [];

async function walk(directory) {
  const entries = await readdir(directory, { withFileTypes: true });

  for (const entry of entries) {
    const fullPath = path.join(directory, entry.name);

    if (entry.isDirectory()) {
      await walk(fullPath);
      continue;
    }

    if (/\.(js|mjs)$/.test(entry.name)) {
      files.push(fullPath);
    }
  }
}

for (const root of roots) {
  await walk(root);
}

if (files.length === 0) {
  console.error('No JS files found for syntax verification.');
  process.exit(1);
}

await new Promise((resolve, reject) => {
  const child = spawn(process.execPath, ['--check', ...files], {
    stdio: 'inherit'
  });

  child.on('exit', (code) => {
    if (code === 0) {
      resolve();
      return;
    }

    reject(new Error(`node --check exited with code ${code}`));
  });
});

console.log(`Syntax check passed for ${files.length} files.`);
