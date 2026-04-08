import test from 'node:test';
import assert from 'node:assert/strict';

import { normalizeConfig, moveItemBetweenGroups, reorderGroups } from '../src/core/config.js';

test('normalizeConfig merges defaults and ensures stable ids', () => {
  const normalized = normalizeConfig({
    app: {
      title: 'NAS Hub'
    },
    groups: [
      {
        name: 'Media'
      }
    ],
    items: [
      {
        title: 'Jellyfin',
        url: 'https://media.local',
        groupId: 'group-media'
      }
    ]
  });

  assert.equal(normalized.app.title, 'NAS Hub');
  assert.equal(normalized.app.subtitle.length > 0, true);
  assert.equal(normalized.groups[0].id, 'group-media');
  assert.equal(normalized.items[0].id, 'item-jellyfin');
  assert.deepEqual(normalized.items[0].tags, []);
});

test('moveItemBetweenGroups updates group ownership and ordering', () => {
  const next = moveItemBetweenGroups(
    {
      groups: [
        { id: 'group-a', name: 'A' },
        { id: 'group-b', name: 'B' }
      ],
      items: [
        { id: 'item-1', title: 'One', groupId: 'group-a' },
        { id: 'item-2', title: 'Two', groupId: 'group-a' },
        { id: 'item-3', title: 'Three', groupId: 'group-b' }
      ]
    },
    {
      itemId: 'item-2',
      targetGroupId: 'group-b',
      targetIndex: 1
    }
  );

  assert.deepEqual(
    next.items.map((item) => `${item.id}:${item.groupId}`),
    ['item-1:group-a', 'item-3:group-b', 'item-2:group-b']
  );
});

test('reorderGroups applies the requested group order', () => {
  const next = reorderGroups(
    [
      { id: 'group-a', name: 'A' },
      { id: 'group-b', name: 'B' },
      { id: 'group-c', name: 'C' }
    ],
    ['group-c', 'group-a', 'group-b']
  );

  assert.deepEqual(next.map((group) => group.id), ['group-c', 'group-a', 'group-b']);
});
