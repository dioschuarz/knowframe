export interface ActionsToolbarCallbacks {
  onSetHub: () => void;
  onRescanVault: () => void;
  onFixBrokenLinks: () => void;
  onToggleColorManager: () => void;
}

export function renderActionsToolbar(
  container: HTMLElement,
  callbacks: ActionsToolbarCallbacks,
): void {
  container.innerHTML = `
    <div class="actions-toolbar">
      <button id="btn-set-hub" class="btn-action" title="Designate selected note as Level 0 active prompt context">
        <span class="codicon codicon-target"></span> Set as Hub
      </button>
      <button id="btn-rescan" class="btn-action" title="Trigger full vault AST re-scan">
        <span class="codicon codicon-sync"></span> Rescan Vault
      </button>
      <button id="btn-fix-links" class="btn-action" title="Inspect broken links and dead section anchors">
        <span class="codicon codicon-warning"></span> Fix Broken Links
      </button>
      <button id="btn-toggle-colors" class="btn-action" title="Manage chromatic cluster rules">
        <span class="codicon codicon-color-mode"></span> Clusters
      </button>
    </div>
  `;

  container.querySelector('#btn-set-hub')?.addEventListener('click', callbacks.onSetHub);
  container.querySelector('#btn-rescan')?.addEventListener('click', callbacks.onRescanVault);
  container.querySelector('#btn-fix-links')?.addEventListener('click', callbacks.onFixBrokenLinks);
  container.querySelector('#btn-toggle-colors')?.addEventListener('click', callbacks.onToggleColorManager);
}
