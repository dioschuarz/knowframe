/**
 * In-memory credentials and token manager backed by vscode.SecretStorage.
 * Implements FR-045 and Constitution Invariant IV & Zero-Leakage Secret Storage Spec:
 * NEVER writes plaintext secrets or authentication tokens to disk, environment variables, or CLI flags.
 */

import * as vscode from 'vscode';

export class SecretManager {
  constructor(private readonly secretStorage: vscode.SecretStorage) {}

  public async getSecret(key: string): Promise<string | undefined> {
    return this.secretStorage.get(key);
  }

  public async storeSecret(key: string, value: string): Promise<void> {
    await this.secretStorage.store(key, value);
  }

  public async deleteSecret(key: string): Promise<void> {
    await this.secretStorage.delete(key);
  }
}
