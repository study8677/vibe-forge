import {
  deleteGroup,
  deleteItem,
  deleteWidget,
  moveItemBetweenGroups,
  normalizeConfig,
  reorderGroups,
  toggleGroupCollapsed,
  upsertGroup,
  upsertItem,
  upsertWidget
} from './core/config.js';
import { defaultConfig } from './core/default-config.js';
import { loadConfig, saveConfig } from './core/storage.js';
import { renderEditor, renderGroups, renderHero, renderSettings } from './ui/render.js';

const state = {
  config: loadConfig(),
  query: '',
  editMode: false,
  settingsOpen: false,
  editor: {
    type: null,
    payload: null
  }
};

const dragState = {
  type: null,
  id: null,
  groupId: null
};

const refs = {
  brandTitle: document.querySelector('#brandTitle'),
  brandSubtitle: document.querySelector('#brandSubtitle'),
  heroSection: document.querySelector('#heroSection'),
  groupsSection: document.querySelector('#groupsSection'),
  settingsDrawer: document.querySelector('#settingsDrawer'),
  settingsContent: document.querySelector('#settingsContent'),
  editorDialog: document.querySelector('#editorDialog'),
  searchInput: document.querySelector('#searchInput'),
  searchForm: document.querySelector('#searchForm'),
  editModeButton: document.querySelector('#editModeButton'),
  toastStack: document.querySelector('#toastStack')
};

function persistConfig() {
  state.config = normalizeConfig(state.config);
  saveConfig(state.config);
}

function syncChrome() {
  refs.brandTitle.textContent = state.config.app.title;
  refs.brandSubtitle.textContent = state.config.app.subtitle;
  refs.editModeButton.textContent = state.editMode ? 'Disable edit mode' : 'Enable edit mode';
  document.body.dataset.settingsOpen = String(state.settingsOpen);
  document.body.dataset.editMode = String(state.editMode);
  document.body.dataset.theme = state.config.app.theme;
  document.body.dataset.background = state.config.app.background;
  document.body.dataset.density = state.config.app.density;
  document.documentElement.style.setProperty('--accent', state.config.app.accent);
}

function renderDynamic() {
  syncChrome();
  refs.heroSection.innerHTML = renderHero(state);
  refs.groupsSection.innerHTML = renderGroups(state);
  refs.settingsContent.innerHTML = renderSettings(state.config);
}

function closeDialog() {
  state.editor = {
    type: null,
    payload: null
  };
  refs.editorDialog.close();
  refs.editorDialog.innerHTML = '';
}

function openEditor(type, payload) {
  state.editor = {
    type,
    payload
  };
  refs.editorDialog.innerHTML = renderEditor(type, payload, state.config);
  refs.editorDialog.showModal();
}

function showToast(message) {
  const node = document.createElement('div');
  node.className = 'toast';
  node.textContent = message;
  refs.toastStack.append(node);

  window.setTimeout(() => {
    node.classList.add('is-leaving');
  }, 1800);

  window.setTimeout(() => {
    node.remove();
  }, 2400);
}

function updateAppSetting(name, value) {
  state.config = {
    ...state.config,
    app: {
      ...state.config.app,
      [name]: value
    }
  };
  persistConfig();
  renderDynamic();
}

function pointerAfterMidline(event, element) {
  const { top, height } = element.getBoundingClientRect();
  return event.clientY > top + height / 2;
}

function reorderIds(order, draggedId, targetId, placeAfter) {
  const next = order.filter((id) => id !== draggedId);
  const targetIndex = next.indexOf(targetId);
  const insertIndex = placeAfter ? targetIndex + 1 : targetIndex;
  next.splice(insertIndex, 0, draggedId);
  return next;
}

function findGroup(groupId) {
  return state.config.groups.find((group) => group.id === groupId);
}

function findItem(itemId) {
  return state.config.items.find((item) => item.id === itemId);
}

function findWidget(widgetId) {
  return state.config.widgets.find((widget) => widget.id === widgetId);
}

function handleAction(action, button) {
  if (action === 'toggle-settings') {
    state.settingsOpen = !state.settingsOpen;
    syncChrome();
    return;
  }

  if (action === 'toggle-edit') {
    state.editMode = !state.editMode;
    renderDynamic();
    return;
  }

  if (action === 'add-group') {
    openEditor('group', { name: '', description: '' });
    return;
  }

  if (action === 'edit-group') {
    const group = findGroup(button.dataset.groupId);
    if (group) {
      openEditor('group', group);
    }
    return;
  }

  if (action === 'delete-group') {
    if (!window.confirm('Delete this group? Existing services will move to the first group.')) {
      return;
    }
    state.config = deleteGroup(state.config, button.dataset.groupId);
    persistConfig();
    renderDynamic();
    showToast('Group deleted');
    return;
  }

  if (action === 'add-item') {
    openEditor('item', {
      title: '',
      url: '',
      description: '',
      groupId: button.dataset.groupId || state.config.groups[0]?.id,
      icon: '',
      tags: [],
      tone: 'neutral'
    });
    return;
  }

  if (action === 'edit-item') {
    const item = findItem(button.dataset.itemId);
    if (item) {
      openEditor('item', item);
    }
    return;
  }

  if (action === 'delete-item') {
    if (!window.confirm('Delete this service link?')) {
      return;
    }
    state.config = deleteItem(state.config, button.dataset.itemId);
    persistConfig();
    renderDynamic();
    showToast('Service deleted');
    return;
  }

  if (action === 'add-widget') {
    openEditor('widget', {
      type: 'note',
      title: 'New widget',
      content: ''
    });
    return;
  }

  if (action === 'edit-widget') {
    const widget = findWidget(button.dataset.widgetId);
    if (widget) {
      openEditor('widget', widget);
    }
    return;
  }

  if (action === 'delete-widget') {
    if (!window.confirm('Delete this widget?')) {
      return;
    }
    state.config = deleteWidget(state.config, button.dataset.widgetId);
    persistConfig();
    renderDynamic();
    showToast('Widget deleted');
    return;
  }

  if (action === 'toggle-group-collapse') {
    state.config = toggleGroupCollapsed(state.config, button.dataset.groupId);
    persistConfig();
    renderDynamic();
    return;
  }

  if (action === 'export-config') {
    const transport = document.querySelector('#configTransport');
    transport.value = JSON.stringify(state.config, null, 2);
    showToast('Config exported to the text area');
    return;
  }

  if (action === 'copy-config') {
    const transport = document.querySelector('#configTransport');
    transport.value = JSON.stringify(state.config, null, 2);
    navigator.clipboard?.writeText(transport.value).then(
      () => showToast('Config copied'),
      () => showToast('Clipboard unavailable, copied to text area instead')
    );
    return;
  }

  if (action === 'import-config') {
    const transport = document.querySelector('#configTransport');
    try {
      state.config = normalizeConfig(JSON.parse(transport.value));
      persistConfig();
      renderDynamic();
      showToast('Config imported');
    } catch {
      showToast('Invalid JSON');
    }
    return;
  }

  if (action === 'restore-defaults') {
    if (!window.confirm('Restore the default NAS layout?')) {
      return;
    }
    state.config = normalizeConfig(defaultConfig);
    persistConfig();
    renderDynamic();
    showToast('Default layout restored');
    return;
  }

  if (action === 'close-dialog') {
    closeDialog();
  }
}

function handleEditorSubmit(form) {
  const formData = new FormData(form);
  const entity = form.dataset.entity;

  if (entity === 'group') {
    state.config = upsertGroup(state.config, {
      id: formData.get('id') || undefined,
      name: formData.get('name'),
      description: formData.get('description')
    });
    persistConfig();
    renderDynamic();
    closeDialog();
    showToast('Group saved');
    return;
  }

  if (entity === 'widget') {
    state.config = upsertWidget(state.config, {
      id: formData.get('id') || undefined,
      type: formData.get('type'),
      title: formData.get('title'),
      content: formData.get('content')
    });
    persistConfig();
    renderDynamic();
    closeDialog();
    showToast('Widget saved');
    return;
  }

  state.config = upsertItem(state.config, {
    id: formData.get('id') || undefined,
    title: formData.get('title'),
    url: formData.get('url'),
    description: formData.get('description'),
    groupId: formData.get('groupId'),
    icon: formData.get('icon'),
    tags: String(formData.get('tags') || '')
      .split(',')
      .map((tag) => tag.trim())
      .filter(Boolean),
    tone: formData.get('tone')
  });
  persistConfig();
  renderDynamic();
  closeDialog();
  showToast('Service saved');
}

function handleDragStart(event) {
  if (!state.editMode || state.query.trim()) {
    return;
  }

  const itemCard = event.target.closest('[data-item-id]');

  if (itemCard) {
    dragState.type = 'item';
    dragState.id = itemCard.dataset.itemId;
    dragState.groupId = itemCard.closest('[data-group-id]')?.dataset.groupId || null;
    event.dataTransfer.effectAllowed = 'move';
    itemCard.classList.add('is-dragging');
    return;
  }

  const groupPanel = event.target.closest('[data-group-id]');

  if (groupPanel) {
    dragState.type = 'group';
    dragState.id = groupPanel.dataset.groupId;
    dragState.groupId = null;
    event.dataTransfer.effectAllowed = 'move';
    groupPanel.classList.add('is-dragging');
  }
}

function handleDrop(event) {
  if (!state.editMode || state.query.trim() || !dragState.type) {
    return;
  }

  event.preventDefault();

  if (dragState.type === 'group') {
    const targetGroup = event.target.closest('[data-group-id]');
    if (!targetGroup || targetGroup.dataset.groupId === dragState.id) {
      return;
    }

    const order = state.config.groups.map((group) => group.id);
    const nextOrder = reorderIds(
      order,
      dragState.id,
      targetGroup.dataset.groupId,
      pointerAfterMidline(event, targetGroup)
    );
    state.config = {
      ...state.config,
      groups: reorderGroups(state.config.groups, nextOrder)
    };
    persistConfig();
    renderDynamic();
    return;
  }

  const targetGroupId = event.target.closest('[data-drop-group-id]')?.dataset.dropGroupId;
  if (!targetGroupId) {
    return;
  }

  const targetCard = event.target.closest('[data-item-id]');
  const targetGroupItems = state.config.items
    .filter((item) => item.groupId === targetGroupId && item.id !== dragState.id)
    .map((item) => item.id);
  let targetIndex = targetGroupItems.length;

  if (targetCard) {
    const targetId = targetCard.dataset.itemId;
    const referenceIndex = targetGroupItems.indexOf(targetId);
    targetIndex = pointerAfterMidline(event, targetCard) ? referenceIndex + 1 : referenceIndex;
  }

  state.config = moveItemBetweenGroups(state.config, {
    itemId: dragState.id,
    targetGroupId,
    targetIndex
  });
  persistConfig();
  renderDynamic();
}

function handleDragEnd() {
  dragState.type = null;
  dragState.id = null;
  dragState.groupId = null;
  document
    .querySelectorAll('.is-dragging')
    .forEach((element) => element.classList.remove('is-dragging'));
}

document.addEventListener('click', (event) => {
  const button = event.target.closest('[data-action]');

  if (button) {
    handleAction(button.dataset.action, button);
  }
});

document.addEventListener('submit', (event) => {
  if (event.target.matches('#searchForm')) {
    event.preventDefault();
    const query = refs.searchInput.value.trim();
    if (!query) {
      return;
    }
    const searchUrl = state.config.app.searchEngine.replace('%s', encodeURIComponent(query));
    window.open(searchUrl, '_blank', 'noreferrer');
    return;
  }

  if (event.target.matches('.editor-form')) {
    event.preventDefault();
    handleEditorSubmit(event.target);
  }
});

document.addEventListener('input', (event) => {
  if (event.target.matches('#searchInput')) {
    state.query = event.target.value;
    refs.heroSection.innerHTML = renderHero(state);
    refs.groupsSection.innerHTML = renderGroups(state);
    return;
  }

  if (event.target.closest('#appSettingsForm')) {
    updateAppSetting(event.target.name, event.target.value);
  }
});

document.addEventListener('change', (event) => {
  if (event.target.closest('#appSettingsForm')) {
    updateAppSetting(event.target.name, event.target.value);
  }
});

document.addEventListener('keydown', (event) => {
  const tagName = document.activeElement?.tagName;
  const isEditingField = tagName === 'INPUT' || tagName === 'TEXTAREA' || tagName === 'SELECT';

  if (event.key === '/' && !isEditingField) {
    event.preventDefault();
    refs.searchInput.focus();
    refs.searchInput.select();
  }

  if (event.key.toLowerCase() === 'e' && !isEditingField) {
    event.preventDefault();
    state.editMode = !state.editMode;
    renderDynamic();
  }

  if (event.key.toLowerCase() === 's' && !isEditingField) {
    event.preventDefault();
    state.settingsOpen = !state.settingsOpen;
    syncChrome();
  }

  if (event.key === 'Escape' && refs.editorDialog.open) {
    closeDialog();
  }
});

document.addEventListener('dragstart', handleDragStart);
document.addEventListener('dragover', (event) => {
  if (state.editMode && !state.query.trim() && dragState.type) {
    event.preventDefault();
  }
});
document.addEventListener('drop', handleDrop);
document.addEventListener('dragend', handleDragEnd);

window.setInterval(() => {
  const now = new Date();
  const time = now.toLocaleTimeString('zh-CN', {
    hour: '2-digit',
    minute: '2-digit'
  });
  const date = now.toLocaleDateString('zh-CN', {
    weekday: 'short',
    month: 'short',
    day: 'numeric'
  });

  document.querySelectorAll('[data-clock-time]').forEach((node) => {
    node.textContent = time;
  });
  document.querySelectorAll('[data-clock-date]').forEach((node) => {
    node.textContent = date;
  });
}, 1000);

renderDynamic();
