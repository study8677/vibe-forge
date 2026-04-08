import { defaultConfig } from './default-config.js';

function slugify(value, prefix) {
  const normalized = String(value ?? '')
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9\u4e00-\u9fa5]+/g, '-')
    .replace(/^-+|-+$/g, '');

  return `${prefix}-${normalized || 'untitled'}`;
}

function uniqueId(existingIds, baseId) {
  if (!existingIds.has(baseId)) {
    return baseId;
  }

  let count = 2;

  while (existingIds.has(`${baseId}-${count}`)) {
    count += 1;
  }

  return `${baseId}-${count}`;
}

function clone(value) {
  return JSON.parse(JSON.stringify(value));
}

function normalizeGroup(group, index) {
  const name = group?.name?.trim() || `Group ${index + 1}`;

  return {
    id: group?.id || slugify(name, 'group'),
    name,
    description: group?.description?.trim() || '',
    collapsed: Boolean(group?.collapsed)
  };
}

function normalizeItem(item, index, fallbackGroupId) {
  const title = item?.title?.trim() || `Service ${index + 1}`;

  return {
    id: item?.id || slugify(title, 'item'),
    groupId: item?.groupId || fallbackGroupId,
    title,
    url: item?.url?.trim() || '#',
    description: item?.description?.trim() || '',
    icon: item?.icon?.trim() || title.slice(0, 2).toUpperCase(),
    tags: Array.isArray(item?.tags) ? item.tags.filter(Boolean) : [],
    tone: item?.tone || 'neutral'
  };
}

function normalizeWidget(widget, index) {
  return {
    id: widget?.id || `widget-${index + 1}`,
    type: widget?.type || 'note',
    title: widget?.title?.trim() || `Widget ${index + 1}`,
    content: widget?.content?.trim() || ''
  };
}

export function normalizeConfig(input = {}) {
  const base = clone(defaultConfig);
  const groups = (input.groups?.length ? input.groups : base.groups).map(normalizeGroup);
  const groupIds = new Set(groups.map((group) => group.id));
  const fallbackGroupId = groups[0]?.id ?? 'group-general';
  const items = (input.items?.length ? input.items : base.items)
    .map((item, index) => normalizeItem(item, index, fallbackGroupId))
    .map((item) => ({
      ...item,
      groupId: groupIds.has(item.groupId) ? item.groupId : fallbackGroupId
    }));
  const widgets = (input.widgets?.length ? input.widgets : base.widgets).map(normalizeWidget);

  return {
    version: 1,
    app: {
      ...base.app,
      ...input.app,
      title: input.app?.title?.trim() || base.app.title,
      subtitle: input.app?.subtitle?.trim() || base.app.subtitle,
      searchEngine: input.app?.searchEngine?.trim() || base.app.searchEngine,
      accent: input.app?.accent?.trim() || base.app.accent,
      density: input.app?.density || base.app.density,
      theme: input.app?.theme || base.app.theme,
      background: input.app?.background || base.app.background
    },
    groups,
    items,
    widgets
  };
}

export function moveItemBetweenGroups(config, movement) {
  const next = {
    ...config,
    groups: [...config.groups],
    items: config.items.map((item) => ({ ...item }))
  };
  const item = next.items.find((entry) => entry.id === movement.itemId);

  if (!item) {
    return next;
  }

  const orderedByGroup = new Map(
    next.groups.map((group) => [
      group.id,
      next.items.filter((entry) => entry.groupId === group.id && entry.id !== movement.itemId)
    ])
  );

  const targetGroupItems = orderedByGroup.get(movement.targetGroupId) ?? [];
  const insertAt = Math.max(0, Math.min(movement.targetIndex, targetGroupItems.length));
  item.groupId = movement.targetGroupId;
  targetGroupItems.splice(insertAt, 0, item);
  orderedByGroup.set(movement.targetGroupId, targetGroupItems);

  next.items = next.groups.flatMap((group) => orderedByGroup.get(group.id) ?? []);
  return next;
}

export function reorderGroups(groups, orderedIds) {
  const lookup = new Map(groups.map((group) => [group.id, group]));
  const next = [];

  for (const id of orderedIds) {
    const group = lookup.get(id);
    if (group) {
      next.push(group);
      lookup.delete(id);
    }
  }

  for (const group of groups) {
    if (lookup.has(group.id)) {
      next.push(group);
    }
  }

  return next;
}

export function toggleGroupCollapsed(config, groupId) {
  return {
    ...config,
    groups: config.groups.map((group) =>
      group.id === groupId ? { ...group, collapsed: !group.collapsed } : group
    )
  };
}

export function upsertGroup(config, payload) {
  const next = clone(config);
  const ids = new Set(next.groups.map((group) => group.id));
  const group = normalizeGroup(payload, next.groups.length);
  const existingIndex = next.groups.findIndex((entry) => entry.id === payload.id);

  if (existingIndex >= 0) {
    next.groups[existingIndex] = {
      ...next.groups[existingIndex],
      ...group
    };

    return next;
  }

  next.groups.push({
    ...group,
    id: uniqueId(ids, group.id)
  });
  return next;
}

export function deleteGroup(config, groupId) {
  if (config.groups.length <= 1) {
    return config;
  }

  const remainingGroups = config.groups.filter((group) => group.id !== groupId);
  const fallbackGroupId = remainingGroups[0].id;

  return {
    ...config,
    groups: remainingGroups,
    items: config.items.map((item) =>
      item.groupId === groupId ? { ...item, groupId: fallbackGroupId } : item
    )
  };
}

export function upsertItem(config, payload) {
  const next = clone(config);
  const ids = new Set(next.items.map((item) => item.id));
  const item = normalizeItem(payload, next.items.length, next.groups[0]?.id);
  const existingIndex = next.items.findIndex((entry) => entry.id === payload.id);

  if (existingIndex >= 0) {
    next.items[existingIndex] = {
      ...next.items[existingIndex],
      ...item
    };

    return next;
  }

  next.items.push({
    ...item,
    id: uniqueId(ids, item.id)
  });
  return next;
}

export function deleteItem(config, itemId) {
  return {
    ...config,
    items: config.items.filter((item) => item.id !== itemId)
  };
}

export function upsertWidget(config, payload) {
  const next = clone(config);
  const existingIndex = next.widgets.findIndex((widget) => widget.id === payload.id);
  const widget = normalizeWidget(payload, next.widgets.length);

  if (existingIndex >= 0) {
    next.widgets[existingIndex] = {
      ...next.widgets[existingIndex],
      ...widget
    };

    return next;
  }

  next.widgets.push(widget);
  return next;
}

export function deleteWidget(config, widgetId) {
  return {
    ...config,
    widgets: config.widgets.filter((widget) => widget.id !== widgetId)
  };
}
