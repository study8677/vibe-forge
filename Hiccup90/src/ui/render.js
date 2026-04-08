import { filterItemsByQuery } from '../core/search.js';

const themeOptions = [
  { value: 'graphite', label: 'Graphite' },
  { value: 'mist', label: 'Mist' }
];

const backgroundOptions = [
  { value: 'aurora', label: 'Aurora' },
  { value: 'grid', label: 'Grid' },
  { value: 'plain', label: 'Plain' }
];

const densityOptions = [
  { value: 'comfortable', label: 'Comfortable' },
  { value: 'compact', label: 'Compact' }
];

const toneOptions = [
  { value: 'neutral', label: 'Neutral' },
  { value: 'cyan', label: 'Cyan' },
  { value: 'green', label: 'Green' },
  { value: 'amber', label: 'Amber' },
  { value: 'violet', label: 'Violet' },
  { value: 'rose', label: 'Rose' }
];

const widgetTypeOptions = [
  { value: 'clock', label: 'Clock' },
  { value: 'stats', label: 'Stats' },
  { value: 'note', label: 'Note' }
];

function escapeHtml(value) {
  return String(value ?? '')
    .replaceAll('&', '&amp;')
    .replaceAll('<', '&lt;')
    .replaceAll('>', '&gt;')
    .replaceAll('"', '&quot;')
    .replaceAll("'", '&#39;');
}

function renderOptions(options, selectedValue) {
  return options
    .map(
      (option) =>
        `<option value="${escapeHtml(option.value)}"${
          option.value === selectedValue ? ' selected' : ''
        }>${escapeHtml(option.label)}</option>`
    )
    .join('');
}

function getHostLabel(url) {
  try {
    return new URL(url).host;
  } catch {
    return 'custom link';
  }
}

function renderIcon(icon, title) {
  if (/^https?:\/\//.test(icon)) {
    return `<img class="service-icon-image" src="${escapeHtml(icon)}" alt="" />`;
  }

  return `<span class="service-icon-fallback">${escapeHtml(icon || title.slice(0, 2))}</span>`;
}

function groupItems(config, query) {
  const filteredItems = filterItemsByQuery(config.items, query);
  const lookup = new Map(config.groups.map((group) => [group.id, []]));

  for (const item of filteredItems) {
    const collection = lookup.get(item.groupId);
    if (collection) {
      collection.push(item);
    }
  }

  return lookup;
}

function renderWidget(widget, config, visibleCount) {
  if (widget.type === 'clock') {
    return `
      <article class="widget-card" data-widget-id="${escapeHtml(widget.id)}">
        <div class="widget-meta">
          <p>${escapeHtml(widget.title)}</p>
          <span>Local time</span>
        </div>
        <strong class="widget-clock-time" data-clock-time>--:--</strong>
        <p class="widget-clock-date" data-clock-date>Loading…</p>
      </article>
    `;
  }

  if (widget.type === 'stats') {
    return `
      <article class="widget-card" data-widget-id="${escapeHtml(widget.id)}">
        <div class="widget-meta">
          <p>${escapeHtml(widget.title)}</p>
          <span>Live summary</span>
        </div>
        <div class="stats-grid">
          <div>
            <strong>${config.items.length}</strong>
            <span>services</span>
          </div>
          <div>
            <strong>${config.groups.length}</strong>
            <span>groups</span>
          </div>
          <div>
            <strong>${visibleCount}</strong>
            <span>visible</span>
          </div>
        </div>
      </article>
    `;
  }

  return `
    <article class="widget-card" data-widget-id="${escapeHtml(widget.id)}">
      <div class="widget-meta">
        <p>${escapeHtml(widget.title)}</p>
        <span>Sticky note</span>
      </div>
      <p class="widget-note-content">${escapeHtml(widget.content || 'Add a short ops note.')}</p>
      <button class="text-button edit-only" type="button" data-action="edit-widget" data-widget-id="${escapeHtml(
        widget.id
      )}">
        Edit widget
      </button>
    </article>
  `;
}

export function renderHero(state) {
  const visibleItems = filterItemsByQuery(state.config.items, state.query);
  const modeLabel = state.editMode ? 'Edit mode enabled' : 'Browse mode';
  const queryLabel = state.query.trim()
    ? `Showing ${visibleItems.length} result${visibleItems.length === 1 ? '' : 's'}`
    : `${state.config.items.length} services ready`;

  return `
    <div class="hero-copy">
      <p class="hero-kicker">Self-hosted control plane</p>
      <h3>${escapeHtml(state.config.app.title)}</h3>
      <p class="hero-summary">${escapeHtml(state.config.app.subtitle)}</p>
      <div class="hero-chips">
        <span>${queryLabel}</span>
        <span>${state.config.groups.length} groups</span>
        <span>${modeLabel}</span>
      </div>
    </div>
    <div class="widget-grid">
      ${state.config.widgets.map((widget) => renderWidget(widget, state.config, visibleItems.length)).join('')}
    </div>
  `;
}

function renderServiceCard(item, editMode, query) {
  const draggable = editMode && !query.trim() ? 'true' : 'false';

  return `
    <article
      class="service-card tone-${escapeHtml(item.tone)}"
      data-item-id="${escapeHtml(item.id)}"
      draggable="${draggable}"
    >
      <a
        class="service-hitbox"
        href="${escapeHtml(item.url)}"
        target="_blank"
        rel="noreferrer"
        aria-label="Open ${escapeHtml(item.title)}"
      ></a>
      <div class="service-card-main">
        <div class="service-icon">${renderIcon(item.icon, item.title)}</div>
        <div class="service-copy">
          <p>${escapeHtml(item.title)}</p>
          <span>${escapeHtml(item.description || getHostLabel(item.url))}</span>
        </div>
        <span class="service-host">${escapeHtml(getHostLabel(item.url))}</span>
      </div>
      <div class="service-footer">
        <div class="service-tags">
          ${(item.tags || []).map((tag) => `<span>${escapeHtml(tag)}</span>`).join('')}
        </div>
        <div class="service-actions edit-only">
          <button class="icon-button" type="button" data-action="edit-item" data-item-id="${escapeHtml(item.id)}">
            Edit
          </button>
          <button class="icon-button danger" type="button" data-action="delete-item" data-item-id="${escapeHtml(
            item.id
          )}">
            Delete
          </button>
        </div>
      </div>
    </article>
  `;
}

export function renderGroups(state) {
  const collections = groupItems(state.config, state.query);
  const hasQuery = state.query.trim().length > 0;
  const panels = state.config.groups
    .map((group) => {
      const items = collections.get(group.id) ?? [];

      if (!state.editMode && hasQuery && items.length === 0) {
        return '';
      }

      return `
        <section
          class="group-panel${group.collapsed ? ' is-collapsed' : ''}"
          data-group-id="${escapeHtml(group.id)}"
          draggable="${state.editMode && !hasQuery ? 'true' : 'false'}"
        >
          <div class="group-header">
            <div class="group-copy">
              <p>${escapeHtml(group.name)}</p>
              <span>${escapeHtml(group.description || 'No description')}</span>
            </div>
            <div class="group-meta">
              <span class="group-count">${items.length}</span>
              <button
                class="icon-button"
                type="button"
                data-action="toggle-group-collapse"
                data-group-id="${escapeHtml(group.id)}"
              >
                ${group.collapsed ? 'Expand' : 'Collapse'}
              </button>
              <div class="edit-only group-edit-actions">
                <button class="icon-button" type="button" data-action="add-item" data-group-id="${escapeHtml(
                  group.id
                )}">
                  Add service
                </button>
                <button class="icon-button" type="button" data-action="edit-group" data-group-id="${escapeHtml(
                  group.id
                )}">
                  Edit
                </button>
                <button class="icon-button danger" type="button" data-action="delete-group" data-group-id="${escapeHtml(
                  group.id
                )}">
                  Delete
                </button>
              </div>
            </div>
          </div>
          <div class="group-body" data-drop-group-id="${escapeHtml(group.id)}">
            <div class="service-grid">
              ${
                items.length > 0
                  ? items.map((item) => renderServiceCard(item, state.editMode, state.query)).join('')
                  : `<div class="empty-state">${
                      state.editMode
                        ? 'Drop services here or create a new one.'
                        : 'No services match the current view.'
                    }</div>`
              }
            </div>
          </div>
        </section>
      `;
    })
    .join('');

  return panels || `<div class="empty-state">No results. Clear the search or create a new service.</div>`;
}

export function renderSettings(config) {
  const itemGroupLookup = new Map(config.groups.map((group) => [group.id, group.name]));

  return `
    <section class="settings-block">
      <div class="settings-block-header">
        <div>
          <p>Application</p>
          <span>Theme, density and search behavior.</span>
        </div>
      </div>
      <form class="settings-form" id="appSettingsForm">
        <label>
          <span>Title</span>
          <input type="text" name="title" value="${escapeHtml(config.app.title)}" />
        </label>
        <label>
          <span>Subtitle</span>
          <input type="text" name="subtitle" value="${escapeHtml(config.app.subtitle)}" />
        </label>
        <label>
          <span>Search engine URL</span>
          <input
            type="text"
            name="searchEngine"
            value="${escapeHtml(config.app.searchEngine)}"
            placeholder="https://www.google.com/search?q=%s"
          />
        </label>
        <label>
          <span>Accent color</span>
          <input type="color" name="accent" value="${escapeHtml(config.app.accent)}" />
        </label>
        <label>
          <span>Theme</span>
          <select name="theme">${renderOptions(themeOptions, config.app.theme)}</select>
        </label>
        <label>
          <span>Background</span>
          <select name="background">${renderOptions(backgroundOptions, config.app.background)}</select>
        </label>
        <label>
          <span>Density</span>
          <select name="density">${renderOptions(densityOptions, config.app.density)}</select>
        </label>
      </form>
    </section>

    <section class="settings-block">
      <div class="settings-block-header">
        <div>
          <p>Groups</p>
          <span>Organize services by workflow.</span>
        </div>
        <button class="text-button" type="button" data-action="add-group">Add group</button>
      </div>
      <div class="settings-list">
        ${config.groups
          .map(
            (group) => `
              <article class="settings-row">
                <div>
                  <p>${escapeHtml(group.name)}</p>
                  <span>${escapeHtml(group.description || 'No description')}</span>
                </div>
                <div class="settings-row-actions">
                  <button class="icon-button" type="button" data-action="edit-group" data-group-id="${escapeHtml(
                    group.id
                  )}">
                    Edit
                  </button>
                  <button class="icon-button danger" type="button" data-action="delete-group" data-group-id="${escapeHtml(
                    group.id
                  )}">
                    Delete
                  </button>
                </div>
              </article>
            `
          )
          .join('')}
      </div>
    </section>

    <section class="settings-block">
      <div class="settings-block-header">
        <div>
          <p>Services</p>
          <span>Manage quick links for your stack.</span>
        </div>
        <button class="text-button" type="button" data-action="add-item">Add service</button>
      </div>
      <div class="settings-list">
        ${config.items
          .map(
            (item) => `
              <article class="settings-row">
                <div>
                  <p>${escapeHtml(item.title)}</p>
                  <span>${escapeHtml(itemGroupLookup.get(item.groupId) || 'Unknown group')} · ${escapeHtml(
                    getHostLabel(item.url)
                  )}</span>
                </div>
                <div class="settings-row-actions">
                  <button class="icon-button" type="button" data-action="edit-item" data-item-id="${escapeHtml(
                    item.id
                  )}">
                    Edit
                  </button>
                  <button class="icon-button danger" type="button" data-action="delete-item" data-item-id="${escapeHtml(
                    item.id
                  )}">
                    Delete
                  </button>
                </div>
              </article>
            `
          )
          .join('')}
      </div>
    </section>

    <section class="settings-block">
      <div class="settings-block-header">
        <div>
          <p>Widgets</p>
          <span>Summary widgets above the navigation grid.</span>
        </div>
        <button class="text-button" type="button" data-action="add-widget">Add widget</button>
      </div>
      <div class="settings-list">
        ${config.widgets
          .map(
            (widget) => `
              <article class="settings-row">
                <div>
                  <p>${escapeHtml(widget.title)}</p>
                  <span>${escapeHtml(widget.type)}</span>
                </div>
                <div class="settings-row-actions">
                  <button class="icon-button" type="button" data-action="edit-widget" data-widget-id="${escapeHtml(
                    widget.id
                  )}">
                    Edit
                  </button>
                  <button class="icon-button danger" type="button" data-action="delete-widget" data-widget-id="${escapeHtml(
                    widget.id
                  )}">
                    Delete
                  </button>
                </div>
              </article>
            `
          )
          .join('')}
      </div>
    </section>

    <section class="settings-block">
      <div class="settings-block-header">
        <div>
          <p>Data tools</p>
          <span>Export, import or restore configuration.</span>
        </div>
      </div>
      <div class="data-tools">
        <textarea id="configTransport" rows="12" placeholder="Exported configuration will appear here. Paste JSON here to import."></textarea>
        <div class="data-tools-actions">
          <button class="ghost-button" type="button" data-action="export-config">Export</button>
          <button class="ghost-button" type="button" data-action="copy-config">Copy</button>
          <button class="ghost-button" type="button" data-action="import-config">Import</button>
          <button class="ghost-button danger" type="button" data-action="restore-defaults">Reset</button>
        </div>
      </div>
    </section>
  `;
}

export function renderEditor(type, payload, config) {
  if (type === 'group') {
    return `
      <form class="editor-form" method="dialog" data-entity="group">
        <div class="editor-header">
          <div>
            <p>Group</p>
            <h2>${payload.id ? 'Edit group' : 'New group'}</h2>
          </div>
          <button class="icon-button" type="button" data-action="close-dialog">×</button>
        </div>
        <input type="hidden" name="id" value="${escapeHtml(payload.id || '')}" />
        <label>
          <span>Name</span>
          <input type="text" name="name" required value="${escapeHtml(payload.name || '')}" />
        </label>
        <label>
          <span>Description</span>
          <textarea name="description" rows="4">${escapeHtml(payload.description || '')}</textarea>
        </label>
        <div class="editor-actions">
          <button class="ghost-button" type="button" data-action="close-dialog">Cancel</button>
          <button class="primary-button" type="submit">Save group</button>
        </div>
      </form>
    `;
  }

  if (type === 'widget') {
    return `
      <form class="editor-form" method="dialog" data-entity="widget">
        <div class="editor-header">
          <div>
            <p>Widget</p>
            <h2>${payload.id ? 'Edit widget' : 'New widget'}</h2>
          </div>
          <button class="icon-button" type="button" data-action="close-dialog">×</button>
        </div>
        <input type="hidden" name="id" value="${escapeHtml(payload.id || '')}" />
        <label>
          <span>Type</span>
          <select name="type">${renderOptions(widgetTypeOptions, payload.type || 'note')}</select>
        </label>
        <label>
          <span>Title</span>
          <input type="text" name="title" required value="${escapeHtml(payload.title || '')}" />
        </label>
        <label>
          <span>Content</span>
          <textarea name="content" rows="5">${escapeHtml(payload.content || '')}</textarea>
        </label>
        <div class="editor-actions">
          <button class="ghost-button" type="button" data-action="close-dialog">Cancel</button>
          <button class="primary-button" type="submit">Save widget</button>
        </div>
      </form>
    `;
  }

  return `
    <form class="editor-form" method="dialog" data-entity="item">
      <div class="editor-header">
        <div>
          <p>Service</p>
          <h2>${payload.id ? 'Edit service' : 'New service'}</h2>
        </div>
        <button class="icon-button" type="button" data-action="close-dialog">×</button>
      </div>
      <input type="hidden" name="id" value="${escapeHtml(payload.id || '')}" />
      <label>
        <span>Title</span>
        <input type="text" name="title" required value="${escapeHtml(payload.title || '')}" />
      </label>
      <label>
        <span>URL</span>
        <input type="url" name="url" required value="${escapeHtml(payload.url || '')}" />
      </label>
      <label>
        <span>Description</span>
        <input type="text" name="description" value="${escapeHtml(payload.description || '')}" />
      </label>
      <label>
        <span>Group</span>
        <select name="groupId">
          ${config.groups
            .map(
              (group) => `
                <option value="${escapeHtml(group.id)}"${
                  group.id === payload.groupId ? ' selected' : ''
                }>
                  ${escapeHtml(group.name)}
                </option>
              `
            )
            .join('')}
        </select>
      </label>
      <label>
        <span>Icon or image URL</span>
        <input type="text" name="icon" value="${escapeHtml(payload.icon || '')}" placeholder="JF or https://..." />
      </label>
      <label>
        <span>Tags</span>
        <input type="text" name="tags" value="${escapeHtml((payload.tags || []).join(', '))}" placeholder="media, video" />
      </label>
      <label>
        <span>Tone</span>
        <select name="tone">${renderOptions(toneOptions, payload.tone || 'neutral')}</select>
      </label>
      <div class="editor-actions">
        <button class="ghost-button" type="button" data-action="close-dialog">Cancel</button>
        <button class="primary-button" type="submit">Save service</button>
      </div>
    </form>
  `;
}
