import { ClusterColorGroup } from '../../../src/types/graph';

export interface ColorManagerCallbacks {
  onSaveGroups: (groups: ClusterColorGroup[]) => void;
  onClose: () => void;
}

export function renderColorManager(
  container: HTMLElement,
  currentGroups: ClusterColorGroup[],
  callbacks: ColorManagerCallbacks,
): void {
  const groups = [...currentGroups];

  const render = () => {
    container.innerHTML = `
      <div class="color-manager-overlay">
        <div class="color-manager-panel">
          <div class="color-manager-header">
            <h3>Chromatic Cluster Groups</h3>
            <button id="btn-close-colors" class="btn-icon">&times;</button>
          </div>
          <p class="color-manager-subtitle">
            Define base hues by hashtag (<code>tag:#name</code>) or path glob (<code>path:folder/**</code>).
            Opacities are strictly preserved by hop depth.
          </p>

          <div class="color-groups-list">
            ${groups
              .map(
                (g, idx) => `
              <div class="color-group-item" data-idx="${idx}">
                <input type="checkbox" class="chk-group-enabled" ${g.enabled ? 'checked' : ''} />
                <input type="color" class="inp-group-color" value="${g.color}" />
                <input type="text" class="inp-group-label" value="${g.label}" placeholder="Label" />
                <input type="text" class="inp-group-query" value="${g.query}" placeholder="tag:#... or path:..." />
                <button class="btn-icon btn-remove-group" title="Delete rule">&times;</button>
              </div>
            `,
              )
              .join('')}
          </div>

          <div class="color-manager-actions">
            <button id="btn-add-group" class="btn-secondary">+ Add Rule</button>
            <div class="actions-right">
              <button id="btn-cancel-colors" class="btn-secondary">Cancel</button>
              <button id="btn-save-colors" class="btn-primary">Apply & Save</button>
            </div>
          </div>
        </div>
      </div>
    `;

    // Hook listeners
    container.querySelector('#btn-close-colors')?.addEventListener('click', callbacks.onClose);
    container.querySelector('#btn-cancel-colors')?.addEventListener('click', callbacks.onClose);

    container.querySelector('#btn-add-group')?.addEventListener('click', () => {
      groups.push({
        query: 'tag:#new_cluster',
        color: '#8B5CF6',
        label: 'New Cluster',
        enabled: true,
      });
      render();
    });

    container.querySelectorAll('.color-group-item').forEach((item) => {
      const idx = Number(item.getAttribute('data-idx'));
      const chk = item.querySelector('.chk-group-enabled') as HTMLInputElement;
      const colorInp = item.querySelector('.inp-group-color') as HTMLInputElement;
      const labelInp = item.querySelector('.inp-group-label') as HTMLInputElement;
      const queryInp = item.querySelector('.inp-group-query') as HTMLInputElement;
      const removeBtn = item.querySelector('.btn-remove-group');

      chk?.addEventListener('change', () => {
        groups[idx].enabled = chk.checked;
      });
      colorInp?.addEventListener('input', () => {
        groups[idx].color = colorInp.value;
      });
      labelInp?.addEventListener('change', () => {
        groups[idx].label = labelInp.value;
      });
      queryInp?.addEventListener('change', () => {
        groups[idx].query = queryInp.value;
      });
      removeBtn?.addEventListener('click', () => {
        groups.splice(idx, 1);
        render();
      });
    });

    container.querySelector('#btn-save-colors')?.addEventListener('click', () => {
      callbacks.onSaveGroups(groups);
      callbacks.onClose();
    });
  };

  render();
}
