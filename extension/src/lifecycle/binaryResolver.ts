import * as fs from 'fs';
import * as path from 'path';
import * as os from 'os';

export function resolveBinaryPath(extensionPath: string): string {
  const binaryName = os.platform() === 'win32' ? 'knowframe-core.exe' : 'knowframe-core';

  // 1. Check workspace dev target build (crates/core/target/debug or target/debug)
  const candidateDevPaths = [
    path.join(extensionPath, '..', 'target', 'release', binaryName),
    path.join(extensionPath, '..', 'target', 'debug', binaryName),
    path.join(extensionPath, '..', 'crates', 'core', 'target', 'release', binaryName),
    path.join(extensionPath, '..', 'crates', 'core', 'target', 'debug', binaryName),
  ];

  for (const p of candidateDevPaths) {
    if (fs.existsSync(p)) {
      return p;
    }
  }

  // 2. Check bundled platform-specific binary directory
  let platformKey = '';
  switch (os.platform()) {
    case 'linux':
      platformKey = os.arch() === 'arm64' ? 'linux-arm64' : 'linux-x64';
      break;
    case 'darwin':
      platformKey = os.arch() === 'arm64' ? 'darwin-arm64' : 'darwin-x64';
      break;
    case 'win32':
      platformKey = 'win32-x64';
      break;
    default:
      platformKey = `${os.platform()}-${os.arch()}`;
  }

  const bundledPath = path.join(extensionPath, 'bin', platformKey, binaryName);
  if (fs.existsSync(bundledPath)) {
    return bundledPath;
  }

  // 3. Fallback to binary directly inside extensionPath / bin
  const flatBinPath = path.join(extensionPath, 'bin', binaryName);
  if (fs.existsSync(flatBinPath)) {
    return flatBinPath;
  }

  // 4. Return binaryName for system PATH lookup
  return binaryName;
}
