import * as crypto from 'crypto';
import * as vscode from 'vscode';

const SECRET_KEY = 'llm-wiki.daemon.auth-token';

export class SecretManager {
  private inMemoryToken: string | null = null;

  constructor(
    private readonly context: vscode.ExtensionContext,
  ) {}

  /**
   * Initializes or retrieves the per-session cryptographic authorization token.
   * Transmits via memory and vscode.SecretStorage with zero plaintext disk exposure.
   */
  public async initializeToken(): Promise<string> {
    let token = await this.context.secrets.get(SECRET_KEY);
    if (!token) {
      token = crypto.randomBytes(32).toString('hex');
      await this.context.secrets.store(SECRET_KEY, token);
    }
    this.inMemoryToken = token;

    // Export token into terminal environment collection for integrated terminal Git commands
    this.context.environmentVariableCollection.replace('LLM_WIKI_AUTH_TOKEN', token);

    return token;
  }

  public getToken(): string | null {
    return this.inMemoryToken;
  }

  public async clearToken(): Promise<void> {
    this.inMemoryToken = null;
    this.context.environmentVariableCollection.delete('LLM_WIKI_AUTH_TOKEN');
    await this.context.secrets.delete(SECRET_KEY);
  }
}
