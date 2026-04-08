import test from 'node:test';
import assert from 'node:assert/strict';

import {
  createInitialState,
  selectStoryline,
  applyPreference,
  getActiveStrategy,
  advanceScene,
  enterScene,
} from '../app/state.js';
import { getSceneActions } from '../app/content.js';

test('createInitialState starts from welcome with balanced defaults', () => {
  const state = createInitialState();

  assert.equal(state.scene, 'welcome');
  assert.equal(state.selectedStoryline, 'empty-after-work');
  assert.deepEqual(state.preferences, {
    closeness: 'balanced',
    support: 'chat',
    warmth: 'measured',
  });
});

test('selectStoryline resets scene and interaction progress', () => {
  const state = {
    ...createInitialState(),
    scene: 'chat',
    chatStep: 3,
    breatheCycles: 2,
    selectedStoryline: 'holding-back',
    selectedMood: 'guarded',
  };

  const next = selectStoryline(state, 'insomnia-loop');

  assert.equal(next.scene, 'tonight');
  assert.equal(next.chatStep, 0);
  assert.equal(next.breatheCycles, 0);
  assert.equal(next.selectedStoryline, 'insomnia-loop');
  assert.equal(next.selectedMood, 'tense');
});

test('applyPreference stores calibration choices without dropping the rest of state', () => {
  const state = createInitialState();
  const next = applyPreference(state, {
    closeness: 'quiet',
    support: 'breathe',
    warmth: 'soft',
  });

  assert.equal(next.scene, 'welcome');
  assert.deepEqual(next.preferences, {
    closeness: 'quiet',
    support: 'breathe',
    warmth: 'soft',
  });
});

test('tense mood maps to a somatic-first strategy', () => {
  const strategy = getActiveStrategy({
    selectedMood: 'tense',
    preferences: { support: 'chat' },
  });

  assert.equal(strategy.primaryAction, 'breathe');
  assert.equal(strategy.theme, 'ember-tense');
  assert.match(strategy.prompt, /慢一点|呼吸|放松/);
});

test('advanceScene moves from breathe into goodnight', () => {
  const next = advanceScene(
    {
      ...createInitialState(),
      scene: 'breathe',
    },
    'continue',
  );

  assert.equal(next.scene, 'goodnight');
});

test('enterScene clears timers and progress for repeatable breathe mode', () => {
  const next = enterScene(
    {
      ...createInitialState(),
      scene: 'chat',
      breatheCycles: 4,
      ambientTimerActive: true,
    },
    'breathe',
  );

  assert.equal(next.scene, 'breathe');
  assert.equal(next.breatheCycles, 0);
  assert.equal(next.ambientTimerActive, false);
});

test('guarded storyline tonight scene offers low-pressure actions', () => {
  const actions = getSceneActions('holding-back', 'tonight');

  assert.deepEqual(
    actions.map((action) => action.label),
    ['陪我待一会', '只说一句', '换个呼吸节奏'],
  );
});
