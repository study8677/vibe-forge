const CJK_CHAR_RE = /\p{Unified_Ideograph}/u;
const LATIN_WORD_RE = /[a-z0-9]+/g;

export function tokenize(text = "") {
  const raw = String(text).toLowerCase();
  const latinWords = raw.match(LATIN_WORD_RE) ?? [];
  const cjkChars = [...raw].filter((char) => CJK_CHAR_RE.test(char));
  const cjkBigrams = [];

  for (let index = 0; index < cjkChars.length - 1; index += 1) {
    cjkBigrams.push(`${cjkChars[index]}${cjkChars[index + 1]}`);
  }

  return [...new Set([...latinWords, ...cjkBigrams])];
}

function buildSearchText(memory) {
  return [
    memory.title,
    memory.summary,
    memory.content,
    ...(memory.tags ?? []),
    memory.category,
    memory.layer,
    memory.source
  ]
    .filter(Boolean)
    .join(" ");
}

function toMillis(value) {
  return value ? new Date(value).getTime() : 0;
}

export function buildMemoryIndex(memories = []) {
  const inverted = new Map();
  const documents = new Map();

  memories.forEach((memory) => {
    const tokens = tokenize(buildSearchText(memory));
    documents.set(memory.id, tokens);

    tokens.forEach((token) => {
      if (!inverted.has(token)) {
        inverted.set(token, new Set());
      }

      inverted.get(token).add(memory.id);
    });
  });

  return {
    documents,
    inverted
  };
}

function collectCandidateIds(index, queryTokens, memories) {
  if (!queryTokens.length) {
    return memories.map((memory) => memory.id);
  }

  const ids = new Set();
  queryTokens.forEach((token) => {
    const matches = index.inverted.get(token);
    if (!matches) {
      return;
    }

    matches.forEach((id) => ids.add(id));
  });

  return [...ids];
}

function scoreMemory(memory, documentTokens, queryTokens) {
  const tokenHits = queryTokens.filter((token) => documentTokens.includes(token)).length;
  const importance = Number(memory.importance ?? 0);
  const lastUsedWeight = toMillis(memory.lastUsedAt) / 1e12;
  const updatedWeight = toMillis(memory.updatedAt) / 1e12;

  return tokenHits * 100 + importance * 5 + lastUsedWeight + updatedWeight;
}

export function searchMemoryIndex({
  index,
  memories = [],
  query = "",
  selectedLayer = "all",
  selectedCategory = "all"
}) {
  const queryTokens = tokenize(query);
  const candidates = collectCandidateIds(index, queryTokens, memories)
    .map((id) => memories.find((memory) => memory.id === id))
    .filter(Boolean)
    .filter((memory) => selectedLayer === "all" || memory.layer === selectedLayer)
    .filter((memory) => selectedCategory === "all" || memory.category === selectedCategory)
    .map((memory) => ({
      memory,
      score: scoreMemory(memory, index.documents.get(memory.id) ?? [], queryTokens)
    }))
    .sort((left, right) => right.score - left.score)
    .map((entry) => entry.memory);

  if (queryTokens.length) {
    return candidates;
  }

  return candidates.sort(
    (left, right) => toMillis(right.updatedAt) - toMillis(left.updatedAt)
  );
}
