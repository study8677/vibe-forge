import { Terminal } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import { WebLinksAddon } from '@xterm/addon-web-links';

const api = window.electronAPI;

// ============================================================
// State
// ============================================================
let connections = [];
const sessions = new Map(); // sessionId -> SessionState
let activeSessionId = null;
let activeView = 'terminal'; // 'terminal' | 'files'

// ============================================================
// DOM helpers
// ============================================================
const $ = (s) => document.querySelector(s);
const $$ = (s) => document.querySelectorAll(s);

function escapeHtml(str) {
  if (!str) return '';
  const d = document.createElement('div');
  d.textContent = str;
  return d.innerHTML;
}

function formatSize(bytes) {
  if (bytes == null || bytes === 0) return '0 B';
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(1024));
  return (bytes / Math.pow(1024, i)).toFixed(i > 0 ? 1 : 0) + ' ' + units[i];
}

function formatDate(ts) {
  if (!ts) return '-';
  const d = new Date(ts);
  const pad = n => String(n).padStart(2, '0');
  return `${d.getFullYear()}-${pad(d.getMonth()+1)}-${pad(d.getDate())} ${pad(d.getHours())}:${pad(d.getMinutes())}`;
}

function showToast(message, type = 'info') {
  const container = $('#toast-container');
  const el = document.createElement('div');
  el.className = `toast ${type}`;
  el.textContent = message;
  container.appendChild(el);
  setTimeout(() => {
    el.style.opacity = '0';
    el.style.transition = 'opacity 0.3s';
    setTimeout(() => el.remove(), 300);
  }, 3000);
}

// ============================================================
// Terminal theme (Catppuccin Mocha)
// ============================================================
const TERM_THEME = {
  background: '#1e1e2e',
  foreground: '#cdd6f4',
  cursor: '#f5e0dc',
  cursorAccent: '#1e1e2e',
  selectionBackground: '#45475a',
  selectionForeground: '#cdd6f4',
  black:   '#45475a', red:     '#f38ba8', green:   '#a6e3a1', yellow:  '#f9e2af',
  blue:    '#89b4fa', magenta: '#f5c2e7', cyan:    '#94e2d5', white:   '#bac2de',
  brightBlack: '#585b70', brightRed:     '#f38ba8', brightGreen: '#a6e3a1', brightYellow: '#f9e2af',
  brightBlue:  '#89b4fa', brightMagenta: '#f5c2e7', brightCyan:  '#94e2d5', brightWhite:  '#a6adc8',
};

// ============================================================
// Initialization
// ============================================================
document.addEventListener('DOMContentLoaded', async () => {
  await loadConnections();
  bindGlobalEvents();
  setupSSHListeners();
});

// ============================================================
// Connection Management
// ============================================================
async function loadConnections() {
  connections = await api.listConnections();
  renderSidebar();
}

function renderSidebar() {
  const list = $('#connection-list');

  // Group by group field
  const groups = {};
  for (const c of connections) {
    const g = c.group || '默认';
    (groups[g] ??= []).push(c);
  }

  if (connections.length === 0) {
    list.innerHTML = '<div class="no-connections-hint">暂无连接<br>点击 + 创建新连接</div>';
    return;
  }

  let html = '';
  for (const [name, conns] of Object.entries(groups)) {
    html += `<div class="connection-group">
      <div class="group-header"><span class="group-arrow">&#9662;</span> ${escapeHtml(name)}</div>`;

    for (const c of conns) {
      const connected = [...sessions.values()].some(s => s.connectionId === c.id);
      html += `
        <div class="connection-item${connected ? ' connected' : ''}" data-id="${c.id}">
          <span class="status-dot"></span>
          <span class="conn-name">${escapeHtml(c.name)}</span>
          <span class="conn-host">${escapeHtml(c.host)}</span>
          <div class="conn-actions">
            <button class="edit-btn" title="编辑">&#9998;</button>
            <button class="delete-btn danger" title="删除">&times;</button>
          </div>
        </div>`;
    }
    html += '</div>';
  }

  list.innerHTML = html;

  // Bind events
  list.querySelectorAll('.connection-item').forEach(el => {
    el.addEventListener('dblclick', () => connectById(el.dataset.id));

    el.querySelector('.edit-btn')?.addEventListener('click', e => {
      e.stopPropagation();
      const c = connections.find(x => x.id === el.dataset.id);
      if (c) showConnectionDialog(c);
    });

    el.querySelector('.delete-btn')?.addEventListener('click', async e => {
      e.stopPropagation();
      const c = connections.find(x => x.id === el.dataset.id);
      if (!c || !confirm(`确定删除连接 "${c.name}" 吗？`)) return;
      await api.deleteConnection(c.id);
      await loadConnections();
      showToast('已删除', 'info');
    });
  });

  // Group toggle
  list.querySelectorAll('.group-header').forEach(h => {
    h.addEventListener('click', () => {
      const items = h.parentElement.querySelectorAll('.connection-item');
      const arrow = h.querySelector('.group-arrow');
      const collapsed = arrow.innerHTML === '&#9656;' || arrow.textContent === '▸';
      arrow.innerHTML = collapsed ? '&#9662;' : '&#9656;';
      items.forEach(i => i.style.display = collapsed ? '' : 'none');
    });
  });
}

// ============================================================
// Connection Dialog
// ============================================================
function showConnectionDialog(conn = null) {
  const isEdit = !!conn;

  const overlay = document.createElement('div');
  overlay.className = 'modal-overlay';
  overlay.innerHTML = `
    <div class="modal-content">
      <h2>${isEdit ? '编辑连接' : '新建连接'}</h2>
      <form id="connection-form" autocomplete="off">
        <div class="form-group">
          <label>名称</label>
          <input type="text" id="f-name" value="${escapeHtml(conn?.name || '')}" required placeholder="My Server">
        </div>
        <div class="form-group">
          <label>主机</label>
          <input type="text" id="f-host" value="${escapeHtml(conn?.host || '')}" required placeholder="192.168.1.1 / example.com">
        </div>
        <div class="form-group">
          <label>端口</label>
          <input type="number" id="f-port" value="${conn?.port || 22}" min="1" max="65535">
        </div>
        <div class="form-group">
          <label>用户名</label>
          <input type="text" id="f-user" value="${escapeHtml(conn?.username || '')}" required placeholder="root">
        </div>
        <div class="form-group">
          <label>认证方式</label>
          <select id="f-auth">
            <option value="password"${conn?.authType !== 'key' ? ' selected' : ''}>密码</option>
            <option value="key"${conn?.authType === 'key' ? ' selected' : ''}>密钥</option>
          </select>
        </div>
        <div class="form-group" id="g-password"${conn?.authType === 'key' ? ' style="display:none"' : ''}>
          <label>密码</label>
          <input type="password" id="f-password" placeholder="${isEdit ? '留空则保持不变' : ''}">
        </div>
        <div class="form-group" id="g-key"${conn?.authType !== 'key' ? ' style="display:none"' : ''}>
          <label>私钥路径</label>
          <div class="input-with-button">
            <input type="text" id="f-key" value="${escapeHtml(conn?.privateKey || '')}" placeholder="~/.ssh/id_rsa">
            <button type="button" id="f-browse">浏览</button>
          </div>
        </div>
        <div class="form-group" id="g-passphrase"${conn?.authType !== 'key' ? ' style="display:none"' : ''}>
          <label>密钥密码（可选）</label>
          <input type="password" id="f-passphrase" placeholder="${isEdit ? '留空则保持不变' : ''}">
        </div>
        <div class="form-group">
          <label>分组</label>
          <input type="text" id="f-group" value="${escapeHtml(conn?.group || '')}" placeholder="默认">
        </div>
        <div class="form-actions">
          <button type="button" class="btn btn-secondary" id="f-cancel">取消</button>
          <button type="submit" class="btn btn-primary">${isEdit ? '保存' : '创建'}</button>
        </div>
      </form>
    </div>`;

  document.body.appendChild(overlay);
  overlay.querySelector('#f-name').focus();

  // Auth type toggle
  overlay.querySelector('#f-auth').addEventListener('change', e => {
    const isKey = e.target.value === 'key';
    overlay.querySelector('#g-password').style.display = isKey ? 'none' : '';
    overlay.querySelector('#g-key').style.display = isKey ? '' : 'none';
    overlay.querySelector('#g-passphrase').style.display = isKey ? '' : 'none';
  });

  // Browse key file
  overlay.querySelector('#f-browse').addEventListener('click', async () => {
    const r = await api.openFileDialog({ title: '选择私钥', properties: ['openFile'] });
    if (r.filePaths?.length) overlay.querySelector('#f-key').value = r.filePaths[0];
  });

  // Close
  const close = () => overlay.remove();
  overlay.querySelector('#f-cancel').addEventListener('click', close);
  overlay.addEventListener('click', e => { if (e.target === overlay) close(); });

  // Submit
  overlay.querySelector('#connection-form').addEventListener('submit', async e => {
    e.preventDefault();
    const data = {
      id: conn?.id || crypto.randomUUID(),
      name:       overlay.querySelector('#f-name').value.trim(),
      host:       overlay.querySelector('#f-host').value.trim(),
      port:       parseInt(overlay.querySelector('#f-port').value) || 22,
      username:   overlay.querySelector('#f-user').value.trim(),
      authType:   overlay.querySelector('#f-auth').value,
      password:   overlay.querySelector('#f-password').value,
      privateKey: overlay.querySelector('#f-key').value.trim(),
      passphrase: overlay.querySelector('#f-passphrase').value,
      group:      overlay.querySelector('#f-group').value.trim() || '默认',
    };
    try {
      await api.saveConnection(data);
      close();
      await loadConnections();
      showToast('连接已保存', 'success');
    } catch (err) {
      showToast('保存失败: ' + err.message, 'error');
    }
  });
}

// ============================================================
// Session Management
// ============================================================
async function connectById(connectionId) {
  // If already connected, switch to it
  for (const [sid, s] of sessions) {
    if (s.connectionId === connectionId) { switchSession(sid); return; }
  }

  const conn = connections.find(c => c.id === connectionId);
  if (!conn) return;

  showToast(`正在连接 ${conn.name}...`, 'info');

  try {
    const sessionId = await api.sshConnect(connectionId);
    const shellId = await api.sshShell(sessionId);

    // Create terminal
    const terminal = new Terminal({
      fontFamily: '"Menlo", "Monaco", "Courier New", monospace',
      fontSize: 14,
      lineHeight: 1.2,
      cursorBlink: true,
      cursorStyle: 'bar',
      allowProposedApi: true,
      theme: TERM_THEME,
    });

    const fitAddon = new FitAddon();
    terminal.loadAddon(fitAddon);
    terminal.loadAddon(new WebLinksAddon());

    // Send input to SSH shell
    terminal.onData(data => api.sshInput(sessionId, shellId, data));

    // Propagate terminal resize to SSH
    terminal.onResize(({ cols, rows }) => api.sshResize(sessionId, shellId, cols, rows));

    sessions.set(sessionId, {
      connectionId,
      connectionName: conn.name,
      terminal,
      fitAddon,
      shellId,
      currentPath: '/',
      fileList: [],
    });

    switchSession(sessionId);

    // Fit after DOM render
    requestAnimationFrame(() => {
      fitAddon.fit();
    });

    renderSidebar();
    showToast(`已连接 ${conn.name}`, 'success');
  } catch (err) {
    showToast(`连接失败: ${err.message}`, 'error');
  }
}

function switchSession(sessionId) {
  // Detach current terminal
  if (activeSessionId && sessions.has(activeSessionId)) {
    const cur = sessions.get(activeSessionId);
    if (cur.terminal.element?.parentNode) {
      cur.terminal.element.parentNode.removeChild(cur.terminal.element);
    }
  }

  activeSessionId = sessionId;
  const session = sessions.get(sessionId);

  // Show session area
  $('#empty-state').style.display = 'none';
  $('#session-area').style.display = 'flex';

  // Mount terminal
  const container = $('#terminal-container');
  container.innerHTML = '';
  session.terminal.open(container);

  requestAnimationFrame(() => session.fitAddon.fit());

  renderTabs();
  switchView(activeView);
}

async function disconnectSession(sessionId) {
  const session = sessions.get(sessionId);
  if (!session) return;

  try { await api.sshDisconnect(sessionId); } catch {}
  session.terminal.dispose();
  sessions.delete(sessionId);

  if (activeSessionId === sessionId) {
    const remaining = [...sessions.keys()];
    if (remaining.length > 0) {
      switchSession(remaining[remaining.length - 1]);
    } else {
      activeSessionId = null;
      $('#empty-state').style.display = '';
      $('#session-area').style.display = 'none';
    }
  }

  renderTabs();
  renderSidebar();
}

function renderTabs() {
  const bar = $('#tab-bar');
  let html = '';
  for (const [sid, s] of sessions) {
    html += `<div class="session-tab${sid === activeSessionId ? ' active' : ''}" data-sid="${sid}">
      <span>${escapeHtml(s.connectionName)}</span>
      <button class="tab-close" title="断开">&times;</button>
    </div>`;
  }
  bar.innerHTML = html;

  bar.querySelectorAll('.session-tab').forEach(tab => {
    tab.addEventListener('click', e => {
      if (!e.target.classList.contains('tab-close')) switchSession(tab.dataset.sid);
    });
    tab.querySelector('.tab-close').addEventListener('click', e => {
      e.stopPropagation();
      disconnectSession(tab.dataset.sid);
    });
  });
}

// ============================================================
// View Switching
// ============================================================
function switchView(view) {
  activeView = view;
  $$('.sub-tab').forEach(t => t.classList.toggle('active', t.dataset.view === view));

  $('#terminal-view').style.display = view === 'terminal' ? '' : 'none';
  $('#file-view').style.display = view === 'files' ? 'flex' : 'none';

  if (view === 'terminal' && activeSessionId) {
    const s = sessions.get(activeSessionId);
    if (s) requestAnimationFrame(() => s.fitAddon.fit());
  }
  if (view === 'files' && activeSessionId) {
    const s = sessions.get(activeSessionId);
    if (s) loadDirectory(s.currentPath);
  }
}

// ============================================================
// File Manager
// ============================================================
async function loadDirectory(dirPath) {
  if (!activeSessionId) return;
  const session = sessions.get(activeSessionId);
  if (!session) return;

  try {
    const items = await api.sftpList(activeSessionId, dirPath);
    session.currentPath = dirPath;
    session.fileList = items;
    $('#current-path').value = dirPath;
    renderFileList(items);
  } catch (err) {
    showToast('读取目录失败: ' + err.message, 'error');
  }
}

function renderFileList(items) {
  const tbody = $('#file-table-body');

  if (items.length === 0) {
    tbody.innerHTML = '<tr><td colspan="5" class="empty-cell">空目录</td></tr>';
    return;
  }

  const session = sessions.get(activeSessionId);
  const basePath = session.currentPath;

  let html = '';
  for (const item of items) {
    const icon = item.isDirectory ? '&#128193;' : '&#128196;';
    const cls = item.isDirectory ? 'file-name is-directory' : 'file-name';
    const fullPath = basePath.endsWith('/') ? basePath + item.name : basePath + '/' + item.name;

    html += `<tr data-path="${escapeHtml(fullPath)}" data-name="${escapeHtml(item.name)}" data-dir="${item.isDirectory}">
      <td><span class="file-icon">${icon}</span><span class="${cls}">${escapeHtml(item.name)}</span></td>
      <td class="file-size">${item.isDirectory ? '-' : formatSize(item.size)}</td>
      <td class="file-date">${formatDate(item.modTime)}</td>
      <td class="file-perms">${item.permissions}</td>
      <td class="file-actions">
        ${!item.isDirectory ? '<button class="dl-btn" title="下载">&#8615;</button>' : ''}
        <button class="ren-btn" title="重命名">&#9998;</button>
        <button class="del-btn danger" title="删除">&times;</button>
      </td>
    </tr>`;
  }
  tbody.innerHTML = html;

  // Bind row events
  tbody.querySelectorAll('tr').forEach(row => {
    const fp = row.dataset.path;
    const name = row.dataset.name;
    const isDir = row.dataset.dir === 'true';

    if (isDir) {
      row.addEventListener('dblclick', () => loadDirectory(fp));
    }

    row.querySelector('.dl-btn')?.addEventListener('click', async e => {
      e.stopPropagation();
      try {
        const r = await api.sftpDownload(activeSessionId, fp);
        if (r) showToast(`已下载 ${name}`, 'success');
      } catch (err) { showToast('下载失败: ' + err.message, 'error'); }
    });

    row.querySelector('.ren-btn')?.addEventListener('click', async e => {
      e.stopPropagation();
      const newName = prompt('新名称:', name);
      if (!newName || newName === name) return;
      const newPath = basePath.endsWith('/') ? basePath + newName : basePath + '/' + newName;
      try {
        await api.sftpRename(activeSessionId, fp, newPath);
        await loadDirectory(session.currentPath);
        showToast('已重命名', 'success');
      } catch (err) { showToast('重命名失败: ' + err.message, 'error'); }
    });

    row.querySelector('.del-btn')?.addEventListener('click', async e => {
      e.stopPropagation();
      if (!confirm(`确定删除 "${name}" 吗？`)) return;
      try {
        await api.sftpDelete(activeSessionId, fp, isDir);
        await loadDirectory(session.currentPath);
        showToast('已删除', 'info');
      } catch (err) { showToast('删除失败: ' + err.message, 'error'); }
    });
  });
}

// ============================================================
// SSH Event Listeners
// ============================================================
function setupSSHListeners() {
  api.onSshOutput((sessionId, _shellId, data) => {
    sessions.get(sessionId)?.terminal.write(data);
  });

  api.onSshClosed(sessionId => {
    if (sessions.has(sessionId)) {
      showToast('连接已断开', 'warning');
      disconnectSession(sessionId);
    }
  });

  api.onSshShellClosed((_sessionId, _shellId) => {
    showToast('Shell 已关闭', 'warning');
  });

  api.onSftpProgress((sessionId, progress) => {
    if (sessionId !== activeSessionId) return;
    const statusEl = $('#transfer-status');
    const fillEl = $('#progress-fill');
    const textEl = $('#progress-text');

    statusEl.style.display = 'flex';
    fillEl.style.width = progress.percent + '%';
    const verb = progress.type === 'upload' ? '上传' : '下载';
    textEl.textContent = `${verb} ${progress.file} — ${progress.percent}% (${formatSize(progress.transferred)} / ${formatSize(progress.total)})`;

    if (progress.done) {
      setTimeout(() => { statusEl.style.display = 'none'; }, 2500);
    }
  });
}

// ============================================================
// Global Event Bindings
// ============================================================
function bindGlobalEvents() {
  // Sidebar buttons
  $('#btn-add').addEventListener('click', () => showConnectionDialog());

  $('#btn-import').addEventListener('click', async () => {
    try {
      const r = await api.importConnections();
      if (r?.error) showToast('导入失败: ' + r.error, 'error');
      else if (r) { await loadConnections(); showToast(`成功导入 ${r.imported} 个连接`, 'success'); }
    } catch (err) { showToast('导入失败: ' + err.message, 'error'); }
  });

  $('#btn-export').addEventListener('click', async () => {
    try {
      const r = await api.exportConnections();
      if (r) showToast('连接已导出', 'success');
    } catch (err) { showToast('导出失败: ' + err.message, 'error'); }
  });

  // Sub-tab switching
  $$('.sub-tab').forEach(t => t.addEventListener('click', () => switchView(t.dataset.view)));

  // File manager controls
  $('#btn-parent').addEventListener('click', () => {
    const s = sessions.get(activeSessionId);
    if (!s) return;
    const parts = s.currentPath.split('/').filter(Boolean);
    parts.pop();
    loadDirectory('/' + parts.join('/') || '/');
  });

  const goToPath = () => loadDirectory($('#current-path').value.trim() || '/');
  $('#btn-go').addEventListener('click', goToPath);
  $('#current-path').addEventListener('keydown', e => { if (e.key === 'Enter') goToPath(); });

  $('#btn-refresh').addEventListener('click', () => {
    const s = sessions.get(activeSessionId);
    if (s) loadDirectory(s.currentPath);
  });

  $('#btn-upload').addEventListener('click', async () => {
    if (!activeSessionId) return;
    const s = sessions.get(activeSessionId);
    try {
      const r = await api.sftpUpload(activeSessionId, s.currentPath);
      if (r) { await loadDirectory(s.currentPath); showToast(`已上传 ${r.length} 个文件`, 'success'); }
    } catch (err) { showToast('上传失败: ' + err.message, 'error'); }
  });

  $('#btn-mkdir').addEventListener('click', async () => {
    if (!activeSessionId) return;
    const s = sessions.get(activeSessionId);
    const name = prompt('文件夹名称:');
    if (!name) return;
    const p = s.currentPath.endsWith('/') ? s.currentPath + name : s.currentPath + '/' + name;
    try {
      await api.sftpMkdir(activeSessionId, p);
      await loadDirectory(s.currentPath);
      showToast('已创建文件夹', 'success');
    } catch (err) { showToast('创建失败: ' + err.message, 'error'); }
  });

  // Window resize -> refit terminal
  window.addEventListener('resize', () => {
    if (activeSessionId && activeView === 'terminal') {
      const s = sessions.get(activeSessionId);
      if (s) s.fitAddon.fit();
    }
  });
}
