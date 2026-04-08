const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

// Create dist directory
fs.mkdirSync(path.join(__dirname, 'dist'), { recursive: true });

// Copy xterm CSS
fs.copyFileSync(
  path.join(__dirname, 'node_modules', '@xterm', 'xterm', 'css', 'xterm.css'),
  path.join(__dirname, 'dist', 'xterm.css')
);

// Bundle renderer JS
execSync(
  'npx esbuild src/renderer/app.js --bundle --outfile=dist/renderer.js --platform=browser --target=chrome120 --format=iife',
  { cwd: __dirname, stdio: 'inherit' }
);

console.log('Build complete!');
