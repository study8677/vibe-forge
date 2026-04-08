# 万事录 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a zero-dependency private steward web app with layered persistent memory, a local chat interface, lightweight indexing, and popular messenger connector stubs.

**Architecture:** The app is a static SPA split into small ES modules. Persistent state lives in localStorage, indexing and organization run entirely on-device, and the UI exposes one coherent flow across chat, memory, topics, and connector inbox synchronization.

**Tech Stack:** HTML, CSS, vanilla JavaScript ES modules, localStorage, Node built-in test runner

---

### Task 1: Lock indexing behavior

**Files:**
- Create: `tests/indexing.test.mjs`
- Create: `src/indexing.js`

- [ ] Write tests for Chinese and English tokenization, inverted index construction, and ranked search.
- [ ] Run `npm test -- tests/indexing.test.mjs` and verify failure before implementation.
- [ ] Implement the smallest tokenization and search utilities needed to satisfy the tests.
- [ ] Re-run `npm test -- tests/indexing.test.mjs` and verify success.

### Task 2: Lock memory organization behavior

**Files:**
- Create: `tests/engine.test.mjs`
- Create: `src/engine.js`

- [ ] Write tests for memory draft extraction, category inference, layer promotion, and context-aware assistant reply generation.
- [ ] Run `npm test -- tests/engine.test.mjs` and verify failure before implementation.
- [ ] Implement the minimal organization engine and reply builder required by the tests.
- [ ] Re-run `npm test -- tests/engine.test.mjs` and verify success.

### Task 3: Add persistence and seeded product state

**Files:**
- Create: `src/store.js`
- Create: `src/data/defaults.js`

- [ ] Define the persisted state shape for conversation, memory, topics, connectors, and inbox items.
- [ ] Add localStorage read/write helpers with versioning and safe fallback to seeded defaults.
- [ ] Wire indexing rebuilds and dashboard summaries into derived selectors.

### Task 4: Build the application shell and natural-light UI

**Files:**
- Create: `index.html`
- Create: `styles.css`
- Create: `src/app.js`
- Create: `src/main.js`

- [ ] Render the dashboard, layered memory panel, topic spaces, connector status board, and chat view from one root app.
- [ ] Implement message sending, memory capture, search filtering, layer/category chips, and connector sync actions.
- [ ] Apply a calm natural-light visual system with card rhythm, warm surfaces, and responsive layout.

### Task 5: Add connector simulation

**Files:**
- Create: `src/connectors.js`
- Modify: `src/app.js`
- Modify: `src/store.js`

- [ ] Model popular chat connectors with consistent metadata.
- [ ] Simulate inbox sync events that create actionable imported items.
- [ ] Surface recent connector activity in both dashboard and memory workflow.

### Task 6: Final docs and verification

**Files:**
- Create: `README.md`

- [ ] Document how to run the app and what the prototype includes.
- [ ] Run `npm test` and verify all tests pass.
- [ ] Run a manual static-serve smoke test with `npm run start`.
