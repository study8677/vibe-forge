import {
  CALIBRATION_STEPS,
  STORYLINE_ORDER,
  getStorylineMeta,
  getSceneActions,
  getChatSequence,
  getClosingMessage,
} from './content.js';
import {
  createInitialState,
  selectStoryline,
  applyPreference,
  getActiveStrategy,
  advanceScene,
  enterScene,
  setAmbientTimer,
  incrementChatStep,
  incrementBreatheCycle,
} from './state.js';

const app = document.querySelector('#app');

let state = createInitialState();
let ambientTimerId = null;

function getClock() {
  return new Intl.DateTimeFormat('zh-CN', {
    hour: '2-digit',
    minute: '2-digit',
  }).format(new Date());
}

function clearAmbientNudge() {
  if (ambientTimerId) {
    window.clearTimeout(ambientTimerId);
    ambientTimerId = null;
  }
  state = setAmbientTimer(state, false);
}

function scheduleAmbientNudge() {
  if (state.scene !== 'tonight') {
    return;
  }

  const nudge = app.querySelector('[data-role="ambient-nudge"]');
  if (!nudge) {
    return;
  }

  state = setAmbientTimer(state, true);
  ambientTimerId = window.setTimeout(() => {
    nudge.dataset.live = 'true';
    state = setAmbientTimer(state, false);
    ambientTimerId = null;
  }, 1800);
}

function setState(nextState) {
  state = nextState;
  render();
}

function renderStorylineTabs() {
  return `
    <div class="storyline-tabs">
      ${STORYLINE_ORDER.map((storylineId) => {
        const meta = getStorylineMeta(storylineId);
        const activeClass =
          storylineId === state.selectedStoryline ? 'is-active' : '';

        return `
          <button
            class="storyline-tab ${activeClass}"
            data-storyline="${storylineId}"
            type="button"
          >
            <span>${meta.shortLabel}</span>
            <small>${meta.label}</small>
          </button>
        `;
      }).join('')}
    </div>
  `;
}

function renderActions(actions, actionKind) {
  return `
    <div class="action-list">
      ${actions
        .map(
          (action) => `
            <button class="action-card" type="button" data-kind="${actionKind}" data-action="${action.id}">
              <span class="action-label">${action.label}</span>
              <span class="action-hint">${action.hint}</span>
            </button>
          `,
        )
        .join('')}
    </div>
  `;
}

function renderCalibration() {
  return `
    <section class="screen calibration-screen">
      <header class="status-row">
        <span>今晚先校准一下距离</span>
        <span>${getClock()}</span>
      </header>
      <div class="scene-body">
        <p class="scene-kicker">CALIBRATION</p>
        <h1 class="scene-title serif">先告诉我，今晚想被怎样对待。</h1>
        <p class="scene-copy">
          只要把边界说轻一点，我就知道该在什么距离开口。
        </p>
        <div class="question-list">
          ${CALIBRATION_STEPS.map(
            (question) => `
              <section class="question-block">
                <p class="question-title">${question.title}</p>
                <div class="option-grid">
                  ${question.options
                    .map((option) => {
                      const selected =
                        state.preferences[question.key] === option.value
                          ? 'is-selected'
                          : '';

                      return `
                        <button
                          class="segmented-option ${selected}"
                          type="button"
                          data-pref-key="${question.key}"
                          data-pref-value="${option.value}"
                        >
                          <span>${option.label}</span>
                          <small>${option.hint}</small>
                        </button>
                      `;
                    })
                    .join('')}
                </div>
              </section>
            `,
          ).join('')}
        </div>
        <button class="primary-button" type="button" data-ui="finish-calibration">
          今晚就这样靠近我
        </button>
      </div>
    </section>
  `;
}

function renderTonight() {
  const meta = getStorylineMeta(state.selectedStoryline);
  const strategy = getActiveStrategy(state);

  return `
    <section class="screen tonight-screen">
      <header class="status-row">
        <span>${meta.timeLabel}</span>
        <span>${strategy.status}</span>
      </header>
      <div class="scene-body">
        ${renderStorylineTabs()}
        <p class="scene-kicker">TONIGHT AIR</p>
        <h1 class="scene-title serif">${meta.headline}</h1>
        <p class="scene-copy">${meta.support}</p>
        <div class="insight-panel">
          <span class="insight-label">察觉到的空气</span>
          <p>${meta.atmosphere}</p>
        </div>
        ${renderActions(getSceneActions(state.selectedStoryline, 'tonight'), 'tonight')}
        <p class="ambient-nudge" data-role="ambient-nudge">${meta.nudge}</p>
      </div>
    </section>
  `;
}

function renderChat() {
  const meta = getStorylineMeta(state.selectedStoryline);
  const sequence = getChatSequence(state.selectedStoryline, state.lastAction);
  const visibleCount = Math.min(sequence.length, state.chatStep + 2);
  const visibleMessages = sequence.slice(0, visibleCount);

  return `
    <section class="screen chat-screen">
      <header class="status-row">
        <button class="inline-link" type="button" data-ui="back-tonight">今晚</button>
        <span>${meta.stateLabel}</span>
      </header>
      <div class="scene-body chat-layout">
        <p class="scene-kicker">LOW-PRESSURE CHAT</p>
        <h1 class="scene-title serif">${meta.label}</h1>
        <div class="message-stack">
          ${visibleMessages
            .map(
              (message, index) => `
                <article class="message message-${message.speaker}" style="animation-delay: ${index * 80}ms">
                  <p>${message.text}</p>
                </article>
              `,
            )
            .join('')}
        </div>
        ${renderActions(
          getSceneActions(state.selectedStoryline, 'chat', state.chatStep),
          'chat',
        )}
      </div>
    </section>
  `;
}

function renderBreathe() {
  const progress = Math.min(((state.breatheCycles + 1) / 3) * 100, 100);
  const isFinalCycle = state.breatheCycles >= 2;

  return `
    <section class="screen breathe-screen">
      <header class="status-row">
        <button class="inline-link" type="button" data-ui="back-tonight">今晚</button>
        <span>先把呼吸接回来</span>
      </header>
      <div class="scene-body breathe-layout">
        <p class="scene-kicker">BREATHING INTERVENTION</p>
        <div class="breathe-ring" style="--progress:${progress}%;">
          <div class="breathe-core">
            <span>${Math.min(state.breatheCycles + 1, 3)}/3</span>
            <small>吸 4 秒 · 停 2 秒 · 呼 6 秒</small>
          </div>
        </div>
        <h1 class="scene-title serif">三轮就好，不用逼自己马上睡着。</h1>
        <p class="scene-copy">
          现在先别跟念头争。只要让身体先慢一点，夜晚就会自己往下落。
        </p>
        ${renderActions(getSceneActions(state.selectedStoryline, 'breathe'), 'breathe')}
        <p class="soft-caption">
          ${isFinalCycle ? '够了。现在去把夜晚收一下。' : '每次只跟一轮，不用一口气做完。'}
        </p>
      </div>
    </section>
  `;
}

function renderGoodnight() {
  const meta = getStorylineMeta(state.selectedStoryline);
  const closing = getClosingMessage(state.selectedStoryline, state.preferences);

  return `
    <section class="screen goodnight-screen">
      <header class="status-row">
        <span>GOODNIGHT</span>
        <span>${getClock()}</span>
      </header>
      <div class="scene-body">
        <p class="scene-kicker">TONIGHT CLOSED SOFTLY</p>
        <h1 class="scene-title serif">${closing.title}</h1>
        <p class="scene-copy">${closing.body}</p>
        <div class="summary-panel">
          <div>
            <span>今晚空气</span>
            <strong>${meta.stateLabel}</strong>
          </div>
          <div>
            <span>你选的距离</span>
            <strong>${state.preferences.closeness === 'quiet' ? '安静陪着' : state.preferences.closeness === 'active' ? '主动一点' : '刚刚好'}</strong>
          </div>
          <div>
            <span>收口方式</span>
            <strong>${state.lastAction === 'breathe' ? '呼吸落地' : '轻量陪伴'}</strong>
          </div>
        </div>
        <div class="footer-actions">
          <button class="primary-button" type="button" data-ui="restart-tonight">
            再看一条夜晚
          </button>
          <button class="ghost-button" type="button" data-ui="back-tonight">
            回到今晚首页
          </button>
        </div>
      </div>
    </section>
  `;
}

function renderWelcome() {
  return `
    <section class="screen welcome-screen">
      <header class="status-row">
        <span>YEQI</span>
        <span>${getClock()}</span>
      </header>
      <div class="scene-body welcome-layout">
        <p class="scene-kicker">MIDNIGHT COMPANION</p>
        <h1 class="brand-mark serif">夜气</h1>
        <p class="scene-title serif">
          它不会一直说话，但会在你快要塌下去之前先靠近一点。
        </p>
        <p class="scene-copy">
          会读空气，识别情绪，主动陪伴，但始终停在不越界的距离。
        </p>
        <div class="brand-manifest">
          <span>先察觉</span>
          <span>再靠近</span>
          <span>最后安静收口</span>
        </div>
        <button class="primary-button" type="button" data-ui="start">
          今夜开始
        </button>
      </div>
    </section>
  `;
}

function renderScene() {
  if (state.scene === 'calibration') {
    return renderCalibration();
  }

  if (state.scene === 'tonight') {
    return renderTonight();
  }

  if (state.scene === 'chat') {
    return renderChat();
  }

  if (state.scene === 'breathe') {
    return renderBreathe();
  }

  if (state.scene === 'goodnight') {
    return renderGoodnight();
  }

  return renderWelcome();
}

function render() {
  clearAmbientNudge();

  const strategy = getActiveStrategy(state);
  const meta = getStorylineMeta(state.selectedStoryline);

  app.innerHTML = `
    <main class="app-shell theme-${strategy.theme}">
      <div class="ambient-orb orb-one"></div>
      <div class="ambient-orb orb-two"></div>
      <section class="phone-frame">
        <div class="phone-noise"></div>
        <div class="screen-wrap">
          ${renderScene()}
        </div>
        <footer class="phone-footer">
          <span>${meta.shortLabel}</span>
          <span>${strategy.status}</span>
        </footer>
      </section>
    </main>
  `;

  scheduleAmbientNudge();
}

function handleUiAction(action) {
  if (action === 'start') {
    setState(advanceScene(state));
    return;
  }

  if (action === 'finish-calibration') {
    setState(
      enterScene(
        {
          ...state,
          calibrationComplete: true,
        },
        'tonight',
      ),
    );
    return;
  }

  if (action === 'back-tonight') {
    setState(enterScene(state, 'tonight'));
    return;
  }

  if (action === 'restart-tonight') {
    setState(selectStoryline(state, state.selectedStoryline));
  }
}

function handleTonightAction(action) {
  const nextBase = {
    ...state,
    lastAction: action,
  };

  if (action === 'breathe') {
    setState(advanceScene(nextBase, 'breathe'));
    return;
  }

  setState(advanceScene(nextBase, 'chat'));
}

function handleChatAction(action) {
  if (action === 'breathe') {
    setState(advanceScene({ ...state, lastAction: action }, 'breathe'));
    return;
  }

  if (action === 'goodnight') {
    setState(advanceScene(state, 'goodnight'));
    return;
  }

  const sequence = getChatSequence(state.selectedStoryline, state.lastAction);
  const next = incrementChatStep(state, sequence.length);

  if (next.chatStep >= Math.max(sequence.length - 2, 0) && action === 'continue') {
    setState(next);
    return;
  }

  setState({
    ...next,
    lastAction: action === 'stay' ? 'stay' : state.lastAction,
  });
}

function handleBreatheAction(action) {
  if (action === 'goodnight') {
    setState(advanceScene(state, 'goodnight'));
    return;
  }

  const next = incrementBreatheCycle(state);
  if (next.breatheCycles >= 3) {
    setState(advanceScene(next, 'continue'));
    return;
  }

  setState(next);
}

app.addEventListener('click', (event) => {
  const prefButton = event.target.closest('[data-pref-key]');
  if (prefButton) {
    const prefKey = prefButton.getAttribute('data-pref-key');
    const prefValue = prefButton.getAttribute('data-pref-value');
    setState(applyPreference(state, { [prefKey]: prefValue }));
    return;
  }

  const storylineButton = event.target.closest('[data-storyline]');
  if (storylineButton) {
    const storylineId = storylineButton.getAttribute('data-storyline');
    setState(selectStoryline(state, storylineId));
    return;
  }

  const uiButton = event.target.closest('[data-ui]');
  if (uiButton) {
    handleUiAction(uiButton.getAttribute('data-ui'));
    return;
  }

  const actionButton = event.target.closest('[data-action]');
  if (!actionButton) {
    return;
  }

  const action = actionButton.getAttribute('data-action');
  const kind = actionButton.getAttribute('data-kind');

  if (kind === 'tonight') {
    handleTonightAction(action);
    return;
  }

  if (kind === 'chat') {
    handleChatAction(action);
    return;
  }

  if (kind === 'breathe') {
    handleBreatheAction(action);
  }
});

render();
