function renderStatCard(label, value, hint) {
  return `
    <article class="stat-card">
      <span class="stat-label">${label}</span>
      <strong class="stat-value">${value}</strong>
      <p class="stat-hint">${hint}</p>
    </article>
  `;
}

function renderMessage(message) {
  return `
    <article class="message ${message.role}">
      <div class="message-role">${message.role === "assistant" ? "管家" : message.role === "system" ? "系统" : "你"}</div>
      <div class="message-content">${message.content.replace(/\n/g, "<br />")}</div>
    </article>
  `;
}

function renderMemoryCard(memory, layerMeta, categoryMeta) {
  return `
    <article class="memory-card" data-accent="${categoryMeta[memory.category]?.accent ?? "sand"}">
      <div class="memory-meta">
        <span class="pill subtle">${layerMeta[memory.layer]?.label ?? memory.layer}</span>
        <span class="pill subtle">${categoryMeta[memory.category]?.label ?? memory.category}</span>
        <span class="memory-status">${memory.status === "pending" ? "待整理" : memory.status === "archived" ? "已归档" : "已整理"}</span>
      </div>
      <h3>${memory.title}</h3>
      <p>${memory.summary}</p>
      <footer>
        <div class="tag-row">
          ${memory.tags.slice(0, 4).map((tag) => `<span class="tag">${tag}</span>`).join("")}
        </div>
        <span class="memory-time">${new Date(memory.updatedAt).toLocaleString("zh-CN", {
          month: "numeric",
          day: "numeric",
          hour: "2-digit",
          minute: "2-digit"
        })}</span>
      </footer>
    </article>
  `;
}

function renderTopicCard(topic, categoryMeta) {
  return `
    <article class="topic-card" data-accent="${topic.accent}">
      <div class="topic-header">
        <span class="pill">${categoryMeta[topic.category]?.label ?? topic.category}</span>
        <strong>${topic.memoryCount}</strong>
      </div>
      <h3>${topic.name}</h3>
      <p>${topic.description}</p>
      <small>${topic.pendingCount ? `还有 ${topic.pendingCount} 条待整理` : "结构稳定，可继续沉淀"}</small>
    </article>
  `;
}

function renderConnectorCard(connector) {
  const statusLabel = {
    connected: "已连接",
    preview: "预留中",
    planned: "计划中"
  }[connector.status];

  return `
    <article class="connector-card" data-accent="${connector.accent}">
      <div class="connector-topline">
        <div>
          <h3>${connector.name}</h3>
          <p>${connector.description}</p>
        </div>
        <span class="status-dot ${connector.status}">${statusLabel}</span>
      </div>
      <div class="connector-meta">
        <span>${connector.family}</span>
        <span>${connector.lastSyncedAt ? `上次同步 ${new Date(connector.lastSyncedAt).toLocaleTimeString("zh-CN", {
          hour: "2-digit",
          minute: "2-digit"
        })}` : "尚未同步"}</span>
      </div>
      <button class="secondary-button" data-action="sync-connector" data-connector-id="${connector.id}">
        同步消息
      </button>
    </article>
  `;
}

function renderInboxItem(item, connectorMap) {
  return `
    <article class="inbox-item">
      <div class="inbox-item-top">
        <strong>${item.title}</strong>
        <span>${connectorMap.get(item.connectorId)?.name ?? item.connectorId}</span>
      </div>
      <p>${item.snippet}</p>
    </article>
  `;
}

function renderFilterButtons(entries, selected, key, label) {
  return entries
    .map(([value, meta]) => {
      const active = selected === value;
      return `<button class="filter-chip ${active ? "active" : ""}" data-action="${key}" data-value="${value}">${label ? meta[label] : meta.label}</button>`;
    })
    .join("");
}

function renderApp(snapshot) {
  const connectorMap = new Map(snapshot.connectors.map((connector) => [connector.id, connector]));

  return `
    <div class="shell">
      <header class="hero">
        <div class="hero-copy">
          <span class="hero-kicker">万事录 · 私人管家</span>
          <h1>把聊天、记忆和外部消息收进同一个自然光生活中枢。</h1>
          <p>当前原型专注于分层记忆、聊天整理、索引召回和主流聊天程序连接器抽象。</p>
        </div>
        <div class="hero-actions">
          <button class="primary-button" data-action="organize">整理待处理记忆</button>
          <button class="ghost-button" data-action="reset">恢复种子数据</button>
        </div>
      </header>

      <section class="stats-grid">
        ${renderStatCard("待整理", snapshot.dashboard.pendingCount, "速记层中的未整理片段")}
        ${renderStatCard("已整理", snapshot.dashboard.organizedCount, "可直接参与召回的结构化记忆")}
        ${renderStatCard("已归档", snapshot.dashboard.archiveCount, "阶段总结和长期档案")}
        ${renderStatCard("联通渠道", snapshot.dashboard.connectedCount, "已经进入同步状态的聊天程序")}
        ${renderStatCard("消息收件箱", snapshot.dashboard.inboxCount, "最近同步进来的外部消息")}
      </section>

      <div class="workspace">
        <aside class="column left-column">
          <section class="panel">
            <div class="panel-heading">
              <div>
                <span class="eyebrow">主题空间</span>
                <h2>生活与工作的长期脉络</h2>
              </div>
            </div>
            <div class="topic-grid">
              ${snapshot.topicSummary.map((topic) => renderTopicCard(topic, snapshot.categoryMeta)).join("")}
            </div>
          </section>

          <section class="panel">
            <div class="panel-heading">
              <div>
                <span class="eyebrow">外部联通</span>
                <h2>流行聊天程序接口层</h2>
              </div>
            </div>
            <div class="connector-list">
              ${snapshot.connectors.map(renderConnectorCard).join("")}
            </div>
          </section>
        </aside>

        <main class="column main-column">
          <section class="panel chat-panel">
            <div class="panel-heading">
              <div>
                <span class="eyebrow">管家对话</span>
                <h2>${snapshot.activeConversation.title}</h2>
              </div>
            </div>
            <div class="chat-log">
              ${snapshot.activeConversation.messages.map(renderMessage).join("")}
            </div>
            <form class="chat-form" data-role="chat-form">
              <textarea
                name="message"
                rows="4"
                placeholder="例如：记住我最近想恢复晨跑；帮我整理五一出行计划；回顾一下最近的工作节奏。"
              ></textarea>
              <div class="chat-form-footer">
                <p>聊天内容会同时触发记忆草稿沉淀与索引更新。</p>
                <button class="primary-button" type="submit">发送给管家</button>
              </div>
            </form>
          </section>
        </main>

        <aside class="column right-column">
          <section class="panel">
            <div class="panel-heading">
              <div>
                <span class="eyebrow">记忆库</span>
                <h2>分层分类与搜索</h2>
              </div>
            </div>
            <label class="search-box">
              <span>检索记忆</span>
              <input
                type="search"
                value="${snapshot.ui.searchQuery}"
                placeholder="输入偏好、主题、地点或关键词"
                data-role="memory-search"
              />
            </label>
            <div class="filter-group">
              <div class="filter-label">层级</div>
              <div class="filter-row">
                <button class="filter-chip ${snapshot.ui.selectedLayer === "all" ? "active" : ""}" data-action="set-layer" data-value="all">全部</button>
                ${renderFilterButtons(Object.entries(snapshot.layerMeta), snapshot.ui.selectedLayer, "set-layer")}
              </div>
            </div>
            <div class="filter-group">
              <div class="filter-label">分类</div>
              <div class="filter-row">
                <button class="filter-chip ${snapshot.ui.selectedCategory === "all" ? "active" : ""}" data-action="set-category" data-value="all">全部</button>
                ${renderFilterButtons(Object.entries(snapshot.categoryMeta), snapshot.ui.selectedCategory, "set-category")}
              </div>
            </div>
            <div class="memory-list">
              ${snapshot.visibleMemories.slice(0, 10).map((memory) => renderMemoryCard(memory, snapshot.layerMeta, snapshot.categoryMeta)).join("")}
            </div>
          </section>

          <section class="panel">
            <div class="panel-heading">
              <div>
                <span class="eyebrow">同步收件箱</span>
                <h2>等待你整理的外部消息</h2>
              </div>
            </div>
            <div class="inbox-list">
              ${snapshot.inbox.slice(0, 6).map((item) => renderInboxItem(item, connectorMap)).join("")}
            </div>
          </section>
        </aside>
      </div>
    </div>
  `;
}

export function createApp(root, store) {
  function redraw(snapshot) {
    root.innerHTML = renderApp(snapshot);
  }

  root.addEventListener("submit", (event) => {
    const form = event.target.closest('[data-role="chat-form"]');
    if (!form) {
      return;
    }

    event.preventDefault();
    const textarea = form.querySelector('textarea[name="message"]');
    store.sendChat(textarea.value);
    textarea.value = "";
  });

  root.addEventListener("input", (event) => {
    const search = event.target.closest('[data-role="memory-search"]');
    if (!search) {
      return;
    }

    store.setSearchQuery(search.value);
  });

  root.addEventListener("click", (event) => {
    const trigger = event.target.closest("[data-action]");
    if (!trigger) {
      return;
    }

    const action = trigger.dataset.action;
    const value = trigger.dataset.value;
    const connectorId = trigger.dataset.connectorId;

    if (action === "organize") {
      store.organizePending();
    }

    if (action === "reset") {
      store.clearStorage();
    }

    if (action === "set-layer") {
      store.setLayerFilter(value);
    }

    if (action === "set-category") {
      store.setCategoryFilter(value);
    }

    if (action === "sync-connector") {
      store.syncConnector(connectorId);
    }
  });

  return store.subscribe(redraw);
}
