# Night Companion Prototype Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a mobile-first, high-fidelity interactive prototype for the "夜气" deep-night companion app with believable emotion sensing, proactive companionship, and restrained ambiguity.

**Architecture:** Use a dependency-light static single-page app. Keep product content, state transitions, and rendering logic separate so the prototype remains easy to test and iterate. Drive all scene changes from a small finite state layer that maps storylines and emotion states to copy, actions, and visual mood.

**Tech Stack:** HTML, CSS, vanilla JavaScript, Node built-in test runner

---

## File Map

- Create: `package.json` — local scripts for serving and testing
- Create: `index.html` — app shell, scene regions, mobile frame
- Create: `app/styles.css` — visual system, layout, motion, responsive polish
- Create: `app/content.js` — storylines, emotion labels, restrained copy variants
- Create: `app/state.js` — state model, transition helpers, emotion strategy mapping
- Create: `app/app.js` — DOM rendering, interactions, scene switching, timers
- Create: `tests/state.test.js` — behavior tests for storyline resets and strategy mapping

### Task 1: Project Skeleton

**Files:**
- Create: `package.json`
- Create: `index.html`

- [ ] **Step 1: Write the failing test**

```js
import test from 'node:test';
import assert from 'node:assert/strict';
import { createInitialState } from '../app/state.js';

test('createInitialState starts on welcome scene', () => {
  const state = createInitialState();
  assert.equal(state.scene, 'welcome');
});
```

- [ ] **Step 2: Run test to verify it fails**

Run: `node --test tests/state.test.js`
Expected: FAIL with module not found for `app/state.js`

- [ ] **Step 3: Write minimal implementation**

```json
{
  "name": "night-companion-prototype",
  "version": "0.1.0",
  "private": true,
  "type": "module",
  "scripts": {
    "dev": "python3 -m http.server 4173",
    "test": "node --test"
  }
}
```

```html
<!doctype html>
<html lang="zh-CN">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0, viewport-fit=cover" />
    <title>夜气</title>
    <link rel="stylesheet" href="./app/styles.css" />
  </head>
  <body>
    <div id="app"></div>
    <script type="module" src="./app/app.js"></script>
  </body>
</html>
```

- [ ] **Step 4: Run test to verify it still fails for the right reason**

Run: `node --test tests/state.test.js`
Expected: FAIL with exported symbol missing from `app/state.js`

- [ ] **Step 5: Commit**

```bash
git add package.json index.html
git commit -m "Record the prototype shell and local scripts

Constraint: Empty workspace needs a runnable baseline before UI work
Confidence: high
Scope-risk: narrow
Directive: Keep the shell dependency-light until the prototype feel is validated
Tested: node --test tests/state.test.js (expected failure for missing state module)
Not-tested: Browser rendering
"
```

### Task 2: State Model and Storyline Logic

**Files:**
- Create: `app/state.js`
- Create: `app/content.js`
- Test: `tests/state.test.js`

- [ ] **Step 1: Write the failing tests**

```js
import test from 'node:test';
import assert from 'node:assert/strict';
import {
  createInitialState,
  selectStoryline,
  applyPreference,
  getActiveStrategy
} from '../app/state.js';

test('selectStoryline resets scene and chat state', () => {
  const state = {
    ...createInitialState(),
    scene: 'chat',
    chatStep: 3,
    selectedStoryline: 'empty-after-work',
    selectedMood: 'low'
  };

  const next = selectStoryline(state, 'insomnia-loop');

  assert.equal(next.scene, 'tonight');
  assert.equal(next.chatStep, 0);
  assert.equal(next.selectedStoryline, 'insomnia-loop');
});

test('applyPreference stores calibration choices', () => {
  const state = createInitialState();
  const next = applyPreference(state, {
    closeness: 'quiet',
    support: 'breathe',
    warmth: 'measured'
  });

  assert.deepEqual(next.preferences, {
    closeness: 'quiet',
    support: 'breathe',
    warmth: 'measured'
  });
});

test('getActiveStrategy maps tense mood to somatic support first', () => {
  const strategy = getActiveStrategy({
    selectedMood: 'tense',
    preferences: { support: 'chat' }
  });

  assert.equal(strategy.primaryAction, 'breathe');
  assert.match(strategy.prompt, /慢一点|呼吸|放松/);
});
```

- [ ] **Step 2: Run test to verify it fails**

Run: `node --test tests/state.test.js`
Expected: FAIL because functions are not exported yet

- [ ] **Step 3: Write minimal implementation**

```js
export const STORYLINES = {
  'empty-after-work': { mood: 'empty' },
  'holding-back': { mood: 'guarded' },
  'insomnia-loop': { mood: 'tense' }
};

export function createInitialState() {
  return {
    scene: 'welcome',
    selectedStoryline: 'empty-after-work',
    selectedMood: 'empty',
    chatStep: 0,
    preferences: {
      closeness: 'balanced',
      support: 'chat',
      warmth: 'measured'
    }
  };
}

export function selectStoryline(state, storylineId) {
  const mood = STORYLINES[storylineId]?.mood ?? 'empty';
  return {
    ...state,
    scene: 'tonight',
    selectedStoryline: storylineId,
    selectedMood: mood,
    chatStep: 0
  };
}

export function applyPreference(state, preferences) {
  return {
    ...state,
    preferences: { ...preferences }
  };
}

export function getActiveStrategy(state) {
  if (state.selectedMood === 'tense') {
    return {
      primaryAction: 'breathe',
      prompt: '先别把自己逼得更紧，跟我慢一点呼吸。'
    };
  }

  return {
    primaryAction: 'stay',
    prompt: '你不用立刻解释，我先陪你待一会。'
  };
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `node --test tests/state.test.js`
Expected: PASS with 3 passing tests

- [ ] **Step 5: Commit**

```bash
git add app/state.js app/content.js tests/state.test.js
git commit -m "Lock the prototype behavior around mood strategy and storyline resets

Constraint: Emotion sensing is simulated locally in this prototype
Rejected: Hardcode all copy in the renderer | would make storyline switching brittle
Confidence: high
Scope-risk: narrow
Directive: Add new moods through the state map first, not ad-hoc DOM conditionals
Tested: node --test tests/state.test.js
Not-tested: Browser UI
"
```

### Task 3: High-Fidelity Visual System

**Files:**
- Create: `app/styles.css`

- [ ] **Step 1: Write the failing test**

Use an assertion in `tests/state.test.js` that the tense strategy exposes a `theme` token:

```js
assert.equal(strategy.theme, 'ember-tense');
```

- [ ] **Step 2: Run test to verify it fails**

Run: `node --test tests/state.test.js`
Expected: FAIL because `theme` is undefined

- [ ] **Step 3: Write minimal implementation**

Extend `getActiveStrategy`:

```js
return {
  primaryAction: 'breathe',
  prompt: '先别把自己逼得更紧，跟我慢一点呼吸。',
  theme: 'ember-tense'
};
```

Create `app/styles.css` with:

```css
:root {
  --bg-0: #07111f;
  --bg-1: #0d1f34;
  --mist: rgba(208, 219, 237, 0.22);
  --warm: #e6b77d;
  --text-main: #f6f1ea;
  --text-soft: rgba(246, 241, 234, 0.7);
  --sans: "Inter", "SF Pro Display", "PingFang SC", sans-serif;
  --serif: "Iowan Old Style", "Songti SC", "Noto Serif SC", serif;
}

body {
  margin: 0;
  min-height: 100vh;
  background:
    radial-gradient(circle at 20% 20%, rgba(230, 183, 125, 0.16), transparent 32%),
    radial-gradient(circle at 80% 30%, rgba(143, 176, 214, 0.18), transparent 26%),
    linear-gradient(180deg, var(--bg-1), var(--bg-0));
  color: var(--text-main);
  font-family: var(--sans);
}

.app-shell {
  min-height: 100vh;
  display: grid;
  place-items: center;
  padding: 24px;
}

.phone-frame {
  width: min(100%, 390px);
  min-height: 844px;
  border-radius: 32px;
  overflow: hidden;
  background: rgba(7, 17, 31, 0.58);
  backdrop-filter: blur(28px);
  box-shadow: 0 30px 80px rgba(0, 0, 0, 0.45);
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `node --test tests/state.test.js`
Expected: PASS with updated strategy expectations

- [ ] **Step 5: Commit**

```bash
git add app/styles.css app/state.js tests/state.test.js
git commit -m "Give the prototype a stable night-air visual system

Constraint: The first impression must feel atmospheric without adding UI dependencies
Rejected: Card-heavy dashboard layout | breaks the calm, poster-like first viewport
Confidence: medium
Scope-risk: moderate
Directive: Keep visual richness in gradients, blur, spacing, and typography rather than extra widgets
Tested: node --test tests/state.test.js
Not-tested: Cross-browser rendering
"
```

### Task 4: Scene Rendering and Interaction Flow

**Files:**
- Modify: `index.html`
- Create: `app/app.js`
- Modify: `app/content.js`

- [ ] **Step 1: Write the failing test**

Add a pure rendering helper test:

```js
import { getSceneActions } from '../app/content.js';

test('guarded storyline tonight scene offers low-pressure actions', () => {
  const actions = getSceneActions('holding-back', 'tonight');
  assert.deepEqual(actions, ['陪我待一会', '只说一句', '换个呼吸节奏']);
});
```

- [ ] **Step 2: Run test to verify it fails**

Run: `node --test tests/state.test.js`
Expected: FAIL because `getSceneActions` does not exist

- [ ] **Step 3: Write minimal implementation**

Add `app/content.js` exports for:

```js
export function getSceneActions(storylineId, scene) {
  const map = {
    'holding-back': {
      tonight: ['陪我待一会', '只说一句', '换个呼吸节奏']
    }
  };

  return map[storylineId]?.[scene] ?? ['陪我待一会', '说一句就好', '陪我慢一点'];
}
```

Create `app/app.js` with:

```js
import { createInitialState, selectStoryline, applyPreference, getActiveStrategy } from './state.js';
import { getSceneActions } from './content.js';

const app = document.querySelector('#app');
let state = createInitialState();

function render() {
  const strategy = getActiveStrategy(state);
  const actions = getSceneActions(state.selectedStoryline, state.scene);

  app.innerHTML = `
    <main class="app-shell" data-scene="${state.scene}">
      <section class="phone-frame">
        <div class="screen screen-${state.scene}">
          <header class="status-row">
            <span>01:12</span>
            <span>${state.selectedMood}</span>
          </header>
          <div class="scene-body">
            <h1>${strategy.prompt}</h1>
            <div class="action-list">
              ${actions.map((label) => `<button data-action="${label}">${label}</button>`).join('')}
            </div>
          </div>
        </div>
      </section>
    </main>
  `;
}

render();
```

- [ ] **Step 4: Run test to verify it passes**

Run: `node --test tests/state.test.js`
Expected: PASS with the action-map test green

- [ ] **Step 5: Commit**

```bash
git add index.html app/app.js app/content.js tests/state.test.js
git commit -m "Turn the mood system into a clickable night-companion flow

Constraint: Prototype must be fully clickable without backend services
Rejected: Multi-page routing setup | unnecessary overhead for a contained narrative prototype
Confidence: medium
Scope-risk: moderate
Directive: Keep rendering functions scene-based and data-driven to protect copy iteration
Tested: node --test tests/state.test.js
Not-tested: Full manual flow
"
```

### Task 5: Breathing Mode, Ending Flow, and Manual Verification

**Files:**
- Modify: `app/app.js`
- Modify: `app/styles.css`
- Test: `tests/state.test.js`

- [ ] **Step 1: Write the failing test**

Add a reducer-style test:

```js
import { advanceScene } from '../app/state.js';

test('advanceScene moves from breathe to goodnight', () => {
  const next = advanceScene({
    ...createInitialState(),
    scene: 'breathe'
  });

  assert.equal(next.scene, 'goodnight');
});
```

- [ ] **Step 2: Run test to verify it fails**

Run: `node --test tests/state.test.js`
Expected: FAIL because `advanceScene` is undefined

- [ ] **Step 3: Write minimal implementation**

Add to `app/state.js`:

```js
export function advanceScene(state) {
  const order = ['welcome', 'calibration', 'tonight', 'chat', 'breathe', 'goodnight'];
  const currentIndex = order.indexOf(state.scene);
  const nextScene = order[Math.min(currentIndex + 1, order.length - 1)];
  return { ...state, scene: nextScene };
}
```

Update `app/app.js` to wire:

- calibration submit
- tonight action buttons
- breathe action advancing to `goodnight`
- storyline switcher

Add CSS for:

- animated breathing ring
- goodnight summary card
- active storyline tabs

- [ ] **Step 4: Run verification**

Run:
- `node --test`
- `python3 -m http.server 4173`

Expected:
- Tests PASS
- Prototype loads at `http://localhost:4173`
- Manual click path works across all 5 scenes and 3 storylines

- [ ] **Step 5: Commit**

```bash
git add app/state.js app/app.js app/styles.css tests/state.test.js
git commit -m "Complete the full late-night companion prototype loop

Constraint: The prototype must prove the emotional pacing in one uninterrupted flow
Rejected: Stop at static mock screens | would not validate proactive companionship
Confidence: medium
Scope-risk: moderate
Directive: If new scenes are added, extend the explicit scene order and retest transitions
Tested: node --test; manual browser flow on localhost
Not-tested: Device-specific Safari quirks
"
```

## Self-Review

- Spec coverage: welcome, calibration, tonight, chat, breathe, and goodnight are all represented in tasks. Storyline switching, mood strategy, and restrained copy all have explicit implementation hooks.
- Placeholder scan: all tasks name exact files, commands, and code targets. Browser-polish work remains tied to specific files.
- Type consistency: state function names (`createInitialState`, `selectStoryline`, `applyPreference`, `getActiveStrategy`, `advanceScene`) are consistent across tasks.
