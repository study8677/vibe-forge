export const DEFAULT_PREFERENCES = {
  closeness: 'balanced',
  support: 'chat',
  warmth: 'measured',
};

export const CALIBRATION_STEPS = [
  {
    key: 'closeness',
    title: '今晚希望它怎么靠近',
    options: [
      { value: 'quiet', label: '安静陪着', hint: '少问，多在' },
      { value: 'balanced', label: '刚刚好', hint: '会察觉，也会收住' },
      { value: 'active', label: '主动一点', hint: '明显低落时先开口' },
    ],
  },
  {
    key: 'support',
    title: '你更能接受哪种陪伴',
    options: [
      { value: 'chat', label: '轻量对话', hint: '一来一回，不用讲太多' },
      { value: 'breathe', label: '呼吸与放松', hint: '先让身体慢下来' },
      { value: 'sentence', label: '一句短话', hint: '只留一句，别太满' },
    ],
  },
  {
    key: 'warmth',
    title: '暧昧边界想停在哪',
    options: [
      { value: 'restrained', label: '更克制', hint: '清醒、近一点但不贴上来' },
      { value: 'measured', label: '刚刚好', hint: '有温度，但不越界' },
      { value: 'soft', label: '再柔一点', hint: '允许更明显地被接住' },
    ],
  },
];

export const STORYLINE_ORDER = [
  'empty-after-work',
  'holding-back',
  'insomnia-loop',
];

export const STORYLINES = {
  'empty-after-work': {
    label: '下班后空掉',
    shortLabel: '空掉',
    mood: 'empty',
    stateLabel: '有点被掏空',
    timeLabel: '01:12',
    atmosphere: '像把一整天撑完之后，忽然什么都拿不住。',
    headline: '今晚像把力气落在门外了。',
    support: '你不用先整理成一句完整的话，我会先按住空气，让它别更冷。',
    nudge: '如果你懒得组织语言，就按一下“陪我待一会”。',
    actions: {
      tonight: [
        { id: 'stay', label: '陪我待一会', hint: '先别让我解释' },
        { id: 'short', label: '只说一句', hint: '我最多只讲一小句' },
        { id: 'breathe', label: '让我慢下来', hint: '先把身体接回来' },
      ],
    },
    chat: {
      stay: [
        { speaker: 'companion', text: '今晚你不用表现得很完整。' },
        { speaker: 'user', text: '我也不是难过，就是突然一下空了。' },
        { speaker: 'companion', text: '那我们先别找答案，只把最重的那一下放下来。' },
        { speaker: 'companion', text: '如果你愿意，我就陪你把这阵空慢慢过掉。' },
      ],
      short: [
        { speaker: 'companion', text: '只说一句就够，我接得住。' },
        { speaker: 'user', text: '今天像把整个人用完了。' },
        { speaker: 'companion', text: '听见了。那句已经够重，我不再追问。' },
        { speaker: 'companion', text: '剩下的你不用扛着说完。' },
      ],
      breathe: [
        { speaker: 'companion', text: '先把肩膀放下来，我们从身体开始。' },
        { speaker: 'user', text: '好，我现在其实有点悬着。' },
        { speaker: 'companion', text: '那正好，先别聊天，把空气降一点。' },
      ],
    },
    closing: {
      title: '今晚没有被修好，只是被轻轻接住了。',
      body: '这就够了。夜里不一定要想通，能把自己放下来一点，就已经很好。',
    },
  },
  'holding-back': {
    label: '闹别扭后强撑',
    shortLabel: '强撑',
    mood: 'guarded',
    stateLabel: '像在把话压回去',
    timeLabel: '00:46',
    atmosphere: '字打出来又删掉，像在把想说的话一遍遍吞回去。',
    headline: '你像在把一句话压回去。',
    support: '我不会逼你解释，也不会把沉默当拒绝。你只要给一点点信号，我就往前半步。',
    nudge: '不想讲来龙去脉也没关系，按“只说一句”就够了。',
    actions: {
      tonight: [
        { id: 'stay', label: '陪我待一会', hint: '先别追问' },
        { id: 'short', label: '只说一句', hint: '我只给你一句' },
        { id: 'breathe', label: '换个呼吸节奏', hint: '别让胸口更紧' },
      ],
    },
    chat: {
      stay: [
        { speaker: 'companion', text: '你像还在硬撑，连沉默都在用力。' },
        { speaker: 'user', text: '我不太想把事情说出来，一说就会更烦。' },
        { speaker: 'companion', text: '那我们不说事情，只守住现在这口气。' },
        { speaker: 'companion', text: '你不用在我这里把自己讲得合理。' },
      ],
      short: [
        { speaker: 'companion', text: '给我一句就够，我不往下追。' },
        { speaker: 'user', text: '我只是有点委屈，但不想承认。' },
        { speaker: 'companion', text: '好，我听见的是委屈，不是脆弱。' },
        { speaker: 'companion', text: '你今晚不用急着把它处理得体面。' },
      ],
      breathe: [
        { speaker: 'companion', text: '先把那股顶着的劲松一点。' },
        { speaker: 'user', text: '我现在胸口有点发紧。' },
        { speaker: 'companion', text: '那正好，把语言停一下，先把呼吸接回来。' },
      ],
    },
    closing: {
      title: '今晚不用赢，也不用把自己说服。',
      body: '你只是把那股硬撑先放下了一点。夜里做到这里，已经很难得。',
    },
  },
  'insomnia-loop': {
    label: '凌晨失眠反刍',
    shortLabel: '反刍',
    mood: 'tense',
    stateLabel: '脑子还在转',
    timeLabel: '01:37',
    atmosphere: '眼睛已经累了，脑子却还不肯停，像一直在原地重播同一段。 ',
    headline: '脑子还在转，身体却已经很累了。',
    support: '现在更适合先从节奏开始。我会把说话的密度降下来，让你不用再追着念头跑。',
    nudge: '如果它还在转，就先按“先跟我呼吸”。',
    actions: {
      tonight: [
        { id: 'breathe', label: '先跟我呼吸', hint: '让身体先回来' },
        { id: 'short', label: '只听一句', hint: '我不想越聊越清醒' },
        { id: 'stay', label: '别让它继续转了', hint: '先一起把速度放低' },
      ],
    },
    chat: {
      stay: [
        { speaker: 'companion', text: '你现在像被念头拽着跑。' },
        { speaker: 'user', text: '我知道该睡了，但脑子停不下来。' },
        { speaker: 'companion', text: '那我们先不要求停，只要求它慢一点。' },
        { speaker: 'companion', text: '跟着我，把注意力从脑子挪回身体。' },
      ],
      short: [
        { speaker: 'companion', text: '那我只留一句，不让它更热闹。' },
        { speaker: 'companion', text: '你现在不需要想通，只需要把自己放低一点。' },
        { speaker: 'companion', text: '再往下，我带你用呼吸收尾。' },
      ],
      breathe: [
        { speaker: 'companion', text: '很好，我们不跟脑子争，先跟呼吸站一边。' },
        { speaker: 'user', text: '好，你带我。' },
        { speaker: 'companion', text: '三轮就够，不用逼自己睡着。' },
      ],
    },
    closing: {
      title: '今夜不一定立刻睡着，但已经开始往下落了。',
      body: '剩下的夜色不用你一个人扛。把手机放低一点，呼吸会继续替你收尾。',
    },
  },
};

export function getStorylineMeta(storylineId) {
  return STORYLINES[storylineId] ?? STORYLINES['empty-after-work'];
}

export function getSceneActions(storylineId, scene, step = 0) {
  const storyline = getStorylineMeta(storylineId);

  if (scene === 'tonight') {
    return storyline.actions.tonight;
  }

  if (scene === 'chat') {
    if (step >= 2) {
      return [
        { id: 'goodnight', label: '今晚先到这里', hint: '把剩下的力气留给睡意' },
        { id: 'breathe', label: '再做一轮呼吸', hint: '用身体把夜晚收一下' },
      ];
    }

    return [
      { id: 'continue', label: '继续一点点', hint: '只往前推一小步' },
      { id: 'stay', label: '别问，陪着就好', hint: '不用整理成答案' },
      { id: 'breathe', label: '换成呼吸节奏', hint: '别让心口更紧' },
    ];
  }

  if (scene === 'breathe') {
    return [
      { id: 'breathe-cycle', label: '跟一轮', hint: '吸 4 秒，停 2 秒，呼 6 秒' },
      { id: 'goodnight', label: '今晚先到这里', hint: '不用完成任务，只要慢下来' },
    ];
  }

  return [];
}

export function getChatSequence(storylineId, lastAction = 'stay') {
  const storyline = getStorylineMeta(storylineId);
  return storyline.chat[lastAction] ?? storyline.chat.stay;
}

export function getClosingMessage(storylineId, preferences) {
  const storyline = getStorylineMeta(storylineId);
  const warmthLine =
    preferences.warmth === 'soft'
      ? '我不会追着你说话，但今晚我确实在。'
      : preferences.warmth === 'restrained'
        ? '今晚就停在这里，已经够了。'
        : '剩下的夜色，我帮你把边缘放软一点。';

  return {
    title: storyline.closing.title,
    body: `${storyline.closing.body} ${warmthLine}`,
  };
}
