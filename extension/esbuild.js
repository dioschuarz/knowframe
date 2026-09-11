const esbuild = require('esbuild');
const fs = require('fs');
const path = require('path');

const production = process.argv.includes('--production');
const watch = process.argv.includes('--watch');

async function main() {
  const extensionCtx = await esbuild.context({
    entryPoints: ['src/extension.ts'],
    bundle: true,
    format: 'cjs',
    minify: production,
    sourcemap: !production,
    sourcesContent: false,
    platform: 'node',
    outfile: 'dist/extension.js',
    external: ['vscode'],
    logLevel: 'info',
  });

  const webviewCtx = await esbuild.context({
    entryPoints: ['webview-ui/src/main.ts'],
    bundle: true,
    format: 'iife',
    minify: production,
    sourcemap: !production,
    sourcesContent: false,
    platform: 'browser',
    outfile: 'dist/webview.js',
    logLevel: 'info',
  });

  if (watch) {
    await extensionCtx.watch();
    await webviewCtx.watch();
  } else {
    await extensionCtx.rebuild();
    await extensionCtx.dispose();
    await webviewCtx.rebuild();
    await webviewCtx.dispose();
  }

  // Copy CSS and HTML to dist
  fs.mkdirSync('dist', { recursive: true });
  fs.copyFileSync(
    path.join(__dirname, 'webview-ui/src/theme.css'),
    path.join(__dirname, 'dist/theme.css')
  );
  fs.copyFileSync(
    path.join(__dirname, 'webview-ui/index.html'),
    path.join(__dirname, 'dist/index.html')
  );
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});

