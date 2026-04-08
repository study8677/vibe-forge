import { DEFAULT_PREFERENCES, STORYLINES } from './content.js';

const SCENE_ORDER = [
  'welcome',
  'calibration',
  'tonight',
  'chat',
  'breathe',
  'goodnight',
];

export function createInitialState() {
  return {
    scene: 'welcome',
    selectedStoryline: 'empty-after-work',
    selectedMood: STORYLINES['empty-after-work'].mood,
    preferences: { ...DEFAULT_PREFERENCES },
    calibrationComplete: false,
    chatStep: 0,
    breatheCycles: 0,
    ambientTimerActive: false,
    lastAction: 'stay',
  };
}

export function selectStoryline(state, storylineId) {
  const storyline = STORYLINES[storylineId] ?? STORYLINES['empty-after-work'];

  return {
    ...state,
    scene: 'tonight',
    selectedStoryline: storylineId,
    selectedMood: storyline.mood,
    chatStep: 0,
    breatheCycles: 0,
    ambientTimerActive: false,
    lastAction: 'stay',
  };
}

export function applyPreference(state, preferences) {
  return {
    ...state,
    preferences: {
      ...state.preferences,
      ...preferences,
    },
  };
}

export function getActiveStrategy(state) {
  const warmth = state.preferences?.warmth ?? 'measured';
  const closeness = state.preferences?.closeness ?? 'balanced';

  const warmthSuffix =
    warmth === 'soft'
      ? '今晚你不用把自己装得没事。'
      : warmth === 'restrained'
        ? '如果你不想解释，我们就先不解释。'
        : '我会靠近一点，但不会压着你。';

  if (state.selectedMood === 'tense') {
    return {
      primaryAction: 'breathe',
      theme: 'ember-tense',
      status: '脑子还在转',
      prompt: '先别把自己逼得更紧，跟我慢一点呼吸。',
      support:
        closeness === 'quiet'
          ? '我先把声音收低，陪你把速度降下来。'
          : `你不用把念头一个个解决，我们先把身体带回来。${warmthSuffix}`,
    };
  }

  if (state.selectedMood === 'guarded') {
    return {
      primaryAction: closeness === 'active' ? 'short' : 'stay',
      theme: 'velvet-guarded',
      status: '像在把话压回去',
      prompt: '你像在把一句话压回去。',
      support: `我不会催你开口，也不会误会你的沉默。${warmthSuffix}`,
    };
  }

  if (state.selectedMood === 'steady') {
    return {
      primaryAction: 'chat',
      theme: 'moon-steady',
      status: '情绪还算平稳',
      prompt: '今晚的空气还算安稳，可以慢一点说。',
      support: warmthSuffix,
    };
  }

  return {
    primaryAction: state.preferences?.support === 'breathe' ? 'breathe' : 'stay',
    theme: 'indigo-empty',
    status: '有点被掏空',
    prompt: '今晚像把力气落在门外了。',
    support: `你不用立刻把自己拼完整。${warmthSuffix}`,
  };
}

export function advanceScene(state, intent = 'continue') {
  if (intent === 'chat') {
    return enterScene(state, 'chat');
  }

  if (intent === 'breathe') {
    return enterScene(state, 'breathe');
  }

  if (intent === 'goodnight') {
    return enterScene(state, 'goodnight');
  }

  if (intent === 'restart') {
    return enterScene(state, 'tonight');
  }

  if (state.scene === 'breathe') {
    return enterScene(state, 'goodnight');
  }

  const currentIndex = SCENE_ORDER.indexOf(state.scene);
  const nextScene =
    SCENE_ORDER[Math.min(currentIndex + 1, SCENE_ORDER.length - 1)];
  return enterScene(state, nextScene);
}

export function enterScene(state, scene) {
  const next = {
    ...state,
    scene,
    ambientTimerActive: false,
  };

  if (scene === 'chat' || scene === 'tonight') {
    next.chatStep = 0;
  }

  if (scene === 'breathe' || scene === 'tonight') {
    next.breatheCycles = 0;
  }

  return next;
}

export function setAmbientTimer(state, ambientTimerActive) {
  return {
    ...state,
    ambientTimerActive,
  };
}

export function incrementChatStep(state, totalMessages) {
  const maxStep = Math.max(totalMessages - 2, 0);
  return {
    ...state,
    chatStep: Math.min(state.chatStep + 1, maxStep),
  };
}

export function incrementBreatheCycle(state) {
  return {
    ...state,
    breatheCycles: state.breatheCycles + 1,
  };
}
