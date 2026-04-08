const { app, BrowserWindow, ipcMain, dialog, safeStorage } = require('electron');
const path = require('path');
const fs = require('fs');
const { Client } = require('ssh2');

// ============================================================
// Connection Store (simple JSON file, no extra dependencies)
// ============================================================
class Store {
  constructor(name) {
    this.filePath = path.join(app.getPath('userData'), `${name}.json`);
    this.data = this._read();
  }
  _read() {
    try { return JSON.parse(fs.readFileSync(this.filePath, 'utf8')); }
    catch { return { connections: [] }; }
  }
  _write() { fs.writeFileSync(this.filePath, JSON.stringify(this.data, null, 2)); }
  get(key) { return this.data[key]; }
  set(key, value) { this.data[key] = value; this._write(); }
}

let store;
const sessions = new Map(); // sessionId -> { conn, config, shells, sftp }
let mainWindow = null;

// ============================================================
// Window
// ============================================================
function createWindow() {
  mainWindow = new BrowserWindow({
    width: 1200,
    height: 800,
    minWidth: 800,
    minHeight: 600,
    backgroundColor: '#1e1e2e',
    webPreferences: {
      preload: path.join(__dirname, 'preload.js'),
      contextIsolation: true,
      nodeIntegration: false,
    },
  });
  mainWindow.loadFile('src/index.html');
}

app.whenReady().then(() => {
  store = new Store('connections');
  createWindow();
});

app.on('window-all-closed', () => {
  for (const [, session] of sessions) {
    try { session.conn.end(); } catch {}
  }
  sessions.clear();
  app.quit();
});

// ============================================================
// Password encryption helpers (uses OS keychain via safeStorage)
// ============================================================
function encryptPassword(password) {
  if (!password) return '';
  try {
    if (safeStorage.isEncryptionAvailable()) {
      return safeStorage.encryptString(password).toString('base64');
    }
  } catch {}
  return password;
}

function decryptPassword(encrypted) {
  if (!encrypted) return '';
  try {
    if (safeStorage.isEncryptionAvailable()) {
      return safeStorage.decryptString(Buffer.from(encrypted, 'base64'));
    }
  } catch {}
  return encrypted;
}

// ============================================================
// IPC — Connection Management
// ============================================================
ipcMain.handle('connections:list', () => {
  return store.get('connections').map(c => ({
    ...c,
    password: c.password ? '***' : '',
    passphrase: c.passphrase ? '***' : '',
  }));
});

ipcMain.handle('connections:save', (_event, connection) => {
  const connections = store.get('connections');
  const idx = connections.findIndex(c => c.id === connection.id);

  // Encrypt new passwords; keep existing if unchanged
  if (connection.password && connection.password !== '***') {
    connection.password = encryptPassword(connection.password);
  } else if (idx >= 0) {
    connection.password = connections[idx].password;
  }
  if (connection.passphrase && connection.passphrase !== '***') {
    connection.passphrase = encryptPassword(connection.passphrase);
  } else if (idx >= 0) {
    connection.passphrase = connections[idx].passphrase;
  }

  if (idx >= 0) connections[idx] = connection;
  else connections.push(connection);

  store.set('connections', connections);
  return true;
});

ipcMain.handle('connections:delete', (_event, id) => {
  store.set('connections', store.get('connections').filter(c => c.id !== id));
  return true;
});

ipcMain.handle('connections:export', async () => {
  const { filePath } = await dialog.showSaveDialog(mainWindow, {
    title: '导出连接',
    defaultPath: 'otron-connections.json',
    filters: [{ name: 'JSON', extensions: ['json'] }],
  });
  if (!filePath) return null;

  const connections = store.get('connections').map(c => ({
    ...c, password: '', passphrase: '',
  }));
  fs.writeFileSync(filePath, JSON.stringify({ version: 1, connections }, null, 2));
  return filePath;
});

ipcMain.handle('connections:import', async () => {
  const { filePaths } = await dialog.showOpenDialog(mainWindow, {
    title: '导入连接',
    filters: [{ name: 'JSON', extensions: ['json'] }],
    properties: ['openFile'],
  });
  if (!filePaths?.length) return null;

  try {
    const raw = JSON.parse(fs.readFileSync(filePaths[0], 'utf8'));
    const incoming = Array.isArray(raw) ? raw : raw.connections;
    if (!Array.isArray(incoming)) throw new Error('无效格式');

    const connections = store.get('connections');
    const existingIds = new Set(connections.map(c => c.id));
    let imported = 0;

    for (const conn of incoming) {
      if (!conn.id || !conn.host) continue;
      if (existingIds.has(conn.id)) {
        const i = connections.findIndex(c => c.id === conn.id);
        connections[i] = { ...connections[i], ...conn };
      } else {
        connections.push(conn);
      }
      imported++;
    }

    store.set('connections', connections);
    return { imported };
  } catch (e) {
    return { error: e.message };
  }
});

// ============================================================
// IPC — SSH
// ============================================================
ipcMain.handle('ssh:connect', async (_event, connectionId) => {
  const config = store.get('connections').find(c => c.id === connectionId);
  if (!config) throw new Error('连接不存在');

  const sessionId = `s_${Date.now()}_${Math.random().toString(36).slice(2, 9)}`;

  return new Promise((resolve, reject) => {
    const conn = new Client();

    conn.on('ready', () => {
      sessions.set(sessionId, { conn, config, shells: new Map(), sftp: null });
      resolve(sessionId);
    });

    conn.on('error', err => reject(new Error(`SSH连接失败: ${err.message}`)));

    conn.on('close', () => {
      if (sessions.has(sessionId)) {
        sessions.delete(sessionId);
        mainWindow?.webContents.send('ssh:closed', sessionId);
      }
    });

    const opts = {
      host: config.host,
      port: config.port || 22,
      username: config.username,
      readyTimeout: 15000,
    };

    if (config.authType === 'key' && config.privateKey) {
      try {
        opts.privateKey = fs.readFileSync(config.privateKey);
        if (config.passphrase) opts.passphrase = decryptPassword(config.passphrase);
      } catch (e) {
        return reject(new Error(`无法读取私钥: ${e.message}`));
      }
    } else {
      opts.password = decryptPassword(config.password);
    }

    conn.connect(opts);
  });
});

ipcMain.handle('ssh:shell', async (_event, sessionId) => {
  const session = sessions.get(sessionId);
  if (!session) throw new Error('Session not found');

  return new Promise((resolve, reject) => {
    session.conn.shell({ term: 'xterm-256color', cols: 80, rows: 24 }, (err, stream) => {
      if (err) return reject(err);

      const shellId = `sh_${Date.now()}`;
      session.shells.set(shellId, stream);

      stream.on('data', data => {
        mainWindow?.webContents.send('ssh:output', sessionId, shellId, data.toString('utf8'));
      });
      stream.stderr.on('data', data => {
        mainWindow?.webContents.send('ssh:output', sessionId, shellId, data.toString('utf8'));
      });
      stream.on('close', () => {
        session.shells.delete(shellId);
        mainWindow?.webContents.send('ssh:shell-closed', sessionId, shellId);
      });

      resolve(shellId);
    });
  });
});

ipcMain.on('ssh:input', (_event, sessionId, shellId, data) => {
  sessions.get(sessionId)?.shells.get(shellId)?.write(data);
});

ipcMain.on('ssh:resize', (_event, sessionId, shellId, cols, rows) => {
  sessions.get(sessionId)?.shells.get(shellId)?.setWindow(rows, cols, 0, 0);
});

ipcMain.handle('ssh:disconnect', (_event, sessionId) => {
  const session = sessions.get(sessionId);
  if (session) { session.conn.end(); sessions.delete(sessionId); }
  return true;
});

// ============================================================
// IPC — SFTP
// ============================================================
async function getSftp(sessionId) {
  const session = sessions.get(sessionId);
  if (!session) throw new Error('Session not found');
  if (!session.sftp) {
    session.sftp = await new Promise((resolve, reject) => {
      session.conn.sftp((err, sftp) => err ? reject(err) : resolve(sftp));
    });
  }
  return session.sftp;
}

ipcMain.handle('sftp:list', async (_event, sessionId, remotePath) => {
  const sftp = await getSftp(sessionId);
  return new Promise((resolve, reject) => {
    sftp.readdir(remotePath, (err, list) => {
      if (err) return reject(new Error(`读取目录失败: ${err.message}`));
      resolve(
        list.map(item => ({
          name: item.filename,
          size: item.attrs.size,
          modTime: item.attrs.mtime * 1000,
          isDirectory: (item.attrs.mode & 0o40000) !== 0,
          permissions: (item.attrs.mode & 0o777).toString(8).padStart(3, '0'),
          owner: item.attrs.uid,
        })).sort((a, b) => {
          if (a.isDirectory !== b.isDirectory) return a.isDirectory ? -1 : 1;
          return a.name.localeCompare(b.name);
        })
      );
    });
  });
});

ipcMain.handle('sftp:download', async (_event, sessionId, remotePath) => {
  const sftp = await getSftp(sessionId);
  const fileName = path.basename(remotePath);

  const { filePath } = await dialog.showSaveDialog(mainWindow, {
    title: '下载文件', defaultPath: fileName,
  });
  if (!filePath) return null;

  return new Promise((resolve, reject) => {
    sftp.stat(remotePath, (statErr, stats) => {
      const totalSize = statErr ? 0 : stats.size;
      let transferred = 0;

      const rs = sftp.createReadStream(remotePath);
      const ws = fs.createWriteStream(filePath);

      rs.on('data', chunk => {
        transferred += chunk.length;
        if (totalSize > 0) {
          mainWindow?.webContents.send('sftp:progress', sessionId, {
            type: 'download', file: fileName, transferred, total: totalSize,
            percent: Math.round((transferred / totalSize) * 100),
          });
        }
      });

      rs.on('error', e => reject(new Error(`下载失败: ${e.message}`)));
      ws.on('error', e => reject(new Error(`写入失败: ${e.message}`)));
      ws.on('finish', () => {
        mainWindow?.webContents.send('sftp:progress', sessionId, {
          type: 'download', file: fileName, transferred: totalSize,
          total: totalSize, percent: 100, done: true,
        });
        resolve(filePath);
      });

      rs.pipe(ws);
    });
  });
});

ipcMain.handle('sftp:upload', async (_event, sessionId, remotePath) => {
  const sftp = await getSftp(sessionId);

  const { filePaths } = await dialog.showOpenDialog(mainWindow, {
    title: '上传文件',
    properties: ['openFile', 'multiSelections'],
  });
  if (!filePaths?.length) return null;

  const results = [];
  for (const localPath of filePaths) {
    const fileName = path.basename(localPath);
    const dest = remotePath.endsWith('/') ? remotePath + fileName : remotePath + '/' + fileName;

    await new Promise((resolve, reject) => {
      const stats = fs.statSync(localPath);
      const totalSize = stats.size;
      let transferred = 0;

      const rs = fs.createReadStream(localPath);
      const ws = sftp.createWriteStream(dest);

      rs.on('data', chunk => {
        transferred += chunk.length;
        mainWindow?.webContents.send('sftp:progress', sessionId, {
          type: 'upload', file: fileName, transferred, total: totalSize,
          percent: Math.round((transferred / totalSize) * 100),
        });
      });

      rs.on('error', e => reject(new Error(`读取失败: ${e.message}`)));
      ws.on('error', e => reject(new Error(`上传失败: ${e.message}`)));
      ws.on('close', () => {
        mainWindow?.webContents.send('sftp:progress', sessionId, {
          type: 'upload', file: fileName, transferred: totalSize,
          total: totalSize, percent: 100, done: true,
        });
        results.push(dest);
        resolve();
      });

      rs.pipe(ws);
    });
  }
  return results;
});

ipcMain.handle('sftp:mkdir', async (_event, sessionId, remotePath) => {
  const sftp = await getSftp(sessionId);
  return new Promise((resolve, reject) => {
    sftp.mkdir(remotePath, err => err ? reject(new Error(`创建目录失败: ${err.message}`)) : resolve(true));
  });
});

ipcMain.handle('sftp:delete', async (_event, sessionId, remotePath, isDirectory) => {
  const sftp = await getSftp(sessionId);
  return new Promise((resolve, reject) => {
    const cb = err => err ? reject(new Error(`删除失败: ${err.message}`)) : resolve(true);
    isDirectory ? sftp.rmdir(remotePath, cb) : sftp.unlink(remotePath, cb);
  });
});

ipcMain.handle('sftp:rename', async (_event, sessionId, oldPath, newPath) => {
  const sftp = await getSftp(sessionId);
  return new Promise((resolve, reject) => {
    sftp.rename(oldPath, newPath, err => err ? reject(new Error(`重命名失败: ${err.message}`)) : resolve(true));
  });
});

// ============================================================
// IPC — Dialog
// ============================================================
ipcMain.handle('dialog:open-file', async (_event, options) => {
  return dialog.showOpenDialog(mainWindow, options);
});
