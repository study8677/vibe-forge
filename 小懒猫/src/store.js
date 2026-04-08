import { CATEGORY_META, LAYER_META, createSeedState } from "./data/defaults.js";
import {
  buildAssistantReply,
  createMemoryDraft,
  promoteCapturedMemories
} from "./engine.js";
import { simulateConnectorSync } from "./connectors.js";
import { buildMemoryIndex, searchMemoryIndex } from "./indexing.js";

const STORAGE_KEY = "wanshilu-state-v1";

function canUseStorage() {
  return typeof window !== "undefined" && typeof window.localStorage !== "undefined";
}

function loadState() {
  if (!canUseStorage()) {
    return createSeedState();
  }

  try {
    const saved = window.localStorage.getItem(STORAGE_KEY);
    if (!saved) {
      return createSeedState();
    }

    return {
      ...createSeedState(),
      ...JSON.parse(saved)
    };
  } catch {
    return createSeedState();
  }
}

function persistState(state) {
  if (!canUseStorage()) {
    return;
  }

  window.localStorage.setItem(STORAGE_KEY, JSON.stringify(state));
}

function toMillis(value) {
  return value ? new Date(value).getTime() : 0;
}

function buildTopicSummary(memories, topicSpaces) {
  return topicSpaces.map((topic) => {
    const matchingMemories = memories.filter((memory) => memory.category === topic.category);
    return {
      ...topic,
      memoryCount: matchingMemories.length,
      pendingCount: matchingMemories.filter((memory) => memory.status === "pending").length,
      latestMemory:
        matchingMemories.sort((left, right) => toMillis(right.updatedAt) - toMillis(left.updatedAt))[0] ??
        null
    };
  });
}

function buildDashboard(memories, connectors, inbox) {
  const pendingCount = memories.filter((memory) => memory.status === "pending").length;
  const organizedCount = memories.filter((memory) => memory.status === "organized").length;
  const archiveCount = memories.filter((memory) => memory.layer === "archive").length;
  const connectedCount = connectors.filter((connector) => connector.status === "connected").length;

  return {
    pendingCount,
    organizedCount,
    archiveCount,
    connectedCount,
    inboxCount: inbox.filter((item) => item.status === "new").length
  };
}

function buildMemoryView(state) {
  const index = buildMemoryIndex(state.memories);
  const memories = searchMemoryIndex({
    index,
    memories: state.memories,
    query: state.ui.searchQuery,
    selectedLayer: state.ui.selectedLayer,
    selectedCategory: state.ui.selectedCategory
  });

  return {
    memoryIndex: index,
    visibleMemories: memories,
    recentMemories: [...state.memories].sort(
      (left, right) => toMillis(right.updatedAt) - toMillis(left.updatedAt)
    ),
    topicSummary: buildTopicSummary(state.memories, state.topicSpaces),
    dashboard: buildDashboard(state.memories, state.connectors, state.inbox)
  };
}

function updateConversation(conversations, conversationId, updater) {
  return conversations.map((conversation) => {
    if (conversation.id !== conversationId) {
      return conversation;
    }

    return updater(conversation);
  });
}

function makeMessage(role, content, memoryIds = []) {
  return {
    id: `msg-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 7)}`,
    role,
    content,
    createdAt: new Date().toISOString(),
    memoryIds
  };
}

function captureInboxToMemory(item) {
  return {
    id: `mem-inbox-${item.id}`,
    title: item.title,
    summary: item.snippet,
    content: `${item.title}：${item.snippet}`,
    layer: item.topicHint === "general" ? "capture" : "topic",
    category: item.topicHint ?? "general",
    tags: ["外部消息", item.connectorId],
    source: "connector",
    importance: 3,
    status: item.topicHint === "general" ? "pending" : "organized",
    createdAt: item.createdAt,
    updatedAt: item.createdAt,
    lastUsedAt: item.createdAt,
    entityRefs: []
  };
}

export function createAppStore() {
  let state = loadState();
  const listeners = new Set();

  function emit() {
    persistState(state);
    const snapshot = api.getSnapshot();
    listeners.forEach((listener) => listener(snapshot));
  }

  function patch(updater) {
    state = updater(state);
    emit();
  }

  const api = {
    subscribe(listener) {
      listeners.add(listener);
      listener(api.getSnapshot());

      return () => listeners.delete(listener);
    },
    getSnapshot() {
      const derived = buildMemoryView(state);

      return {
        ...state,
        ...derived,
        layerMeta: LAYER_META,
        categoryMeta: CATEGORY_META,
        activeConversation:
          state.conversations.find((conversation) => conversation.id === state.activeConversationId) ??
          state.conversations[0]
      };
    },
    sendChat(input) {
      const content = String(input).trim();
      if (!content) {
        return;
      }

      patch((current) => {
        const draft = createMemoryDraft(content);
        const index = buildMemoryIndex(current.memories);
        const relatedMemories = searchMemoryIndex({
          index,
          memories: current.memories,
          query: content
        }).slice(0, 3);

        let memories = draft ? [draft, ...current.memories] : [...current.memories];
        let assistantReply = buildAssistantReply({ input: content, relatedMemories });

        if (/整理|归档|收纳/.test(content)) {
          memories = promoteCapturedMemories(memories);
          assistantReply = buildAssistantReply({ input: content, relatedMemories });
        }

        const messagesToAppend = [
          makeMessage("user", content, draft ? [draft.id] : []),
          makeMessage("assistant", assistantReply)
        ];

        return {
          ...current,
          memories,
          conversations: updateConversation(
            current.conversations,
            current.activeConversationId,
            (conversation) => ({
              ...conversation,
              updatedAt: new Date().toISOString(),
              messages: [...conversation.messages, ...messagesToAppend]
            })
          )
        };
      });
    },
    setSearchQuery(query) {
      patch((current) => ({
        ...current,
        ui: {
          ...current.ui,
          searchQuery: query
        }
      }));
    },
    setLayerFilter(layer) {
      patch((current) => ({
        ...current,
        ui: {
          ...current.ui,
          selectedLayer: current.ui.selectedLayer === layer ? "all" : layer
        }
      }));
    },
    setCategoryFilter(category) {
      patch((current) => ({
        ...current,
        ui: {
          ...current.ui,
          selectedCategory: current.ui.selectedCategory === category ? "all" : category
        }
      }));
    },
    organizePending() {
      patch((current) => ({
        ...current,
        memories: promoteCapturedMemories(current.memories)
      }));
    },
    syncConnector(connectorId) {
      patch((current) => {
        const result = simulateConnectorSync(connectorId);
        const inboxItems = [...result.inboxItems, ...current.inbox];
        const connectorIds = result.inboxItems.map((item) => item.id);
        const importedMemories = result.inboxItems.map(captureInboxToMemory);

        return {
          ...current,
          connectors: current.connectors.map((connector) =>
            connector.id === connectorId
              ? {
                  ...connector,
                  ...result.connectorUpdate,
                  inboxCount: connector.inboxCount + result.inboxItems.length
                }
              : connector
          ),
          inbox: inboxItems,
          memories: [...importedMemories, ...current.memories],
          conversations: updateConversation(
            current.conversations,
            current.activeConversationId,
            (conversation) => ({
              ...conversation,
              updatedAt: new Date().toISOString(),
              messages: [
                ...conversation.messages,
                makeMessage(
                  "system",
                  `已从 ${connectorId} 同步 ${connectorIds.length} 条新消息，并生成可整理的记忆条目。`
                )
              ]
            })
          )
        };
      });
    },
    clearStorage() {
      state = createSeedState();
      emit();
    }
  };

  return api;
}
