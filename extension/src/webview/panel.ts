/**
 * Webview panel hosting 4 views (Health, Structure, Evolution, Settings)
 * and Structure 2D canvas with dynamic CSP nonces.
 * Implements FR-042, FR-046, and NFR-008.
 */

import * as vscode from 'vscode';
import { GraphPanelManager } from './panelManager';
import { generateNonce, buildCspPolicy } from './cspHelper';

export { GraphPanelManager, generateNonce, buildCspPolicy };

export type ManagerView = 'health' | 'structure' | 'evolution' | 'settings';

export interface ManagerPanelOptions {
  activeView: ManagerView;
  extensionUri: vscode.Uri;
}
