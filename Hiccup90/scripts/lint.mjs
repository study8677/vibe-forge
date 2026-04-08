import { readdir, readFile, stat } from 'node:fs/promises';
import path from 'node:path';

const root = process.cwd();
const allowedExtensions = new Set(['.js', '.mjs', '.html', '.css']);
const problems = [];

async function walk(directory) {
  const entries = await readdir(directory, { withFileTypes: true });

  for (const entry of entries) {
    if (entry.name === 'node_modules' || entry.name === '.git') {
      continue;
    }

    const fullPath = path.join(directory, entry.name);

    if (entry.isDirectory()) {
      await walk(fullPath);
      continue;
    }

    if (!allowedExtensions.has(path.extname(entry.name))) {
      continue;
    }

    const content = await readFile(fullPath, 'utf8');

    if (content.includes('\t')) {
      problems.push(`${path.relative(root, fullPath)} contains tabs`);
    }
  }
}

await stat(root);
await walk(root);

if (problems.length > 0) {
  console.error(problems.join('\n'));
  process.exit(1);
}
