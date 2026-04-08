import test from 'node:test';
import assert from 'node:assert/strict';

import { filterItemsByQuery } from '../src/core/search.js';

const items = [
  {
    id: 'item-jellyfin',
    title: 'Jellyfin',
    description: '媒体中心',
    tags: ['media', 'video']
  },
  {
    id: 'item-immich',
    title: 'Immich',
    description: '照片管理',
    tags: ['photo', 'backup']
  }
];

test('filterItemsByQuery matches title and tags', () => {
  const result = filterItemsByQuery(items, 'video');

  assert.deepEqual(result.map((item) => item.id), ['item-jellyfin']);
});

test('filterItemsByQuery returns all items for empty query', () => {
  const result = filterItemsByQuery(items, '   ');

  assert.equal(result.length, 2);
});
