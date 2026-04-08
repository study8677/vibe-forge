const { contextBridge, ipcRenderer } = require('electron');

contextBridge.exposeInMainWorld('electronAPI', {
  // --- Connections ---
  listConnections: () => ipcRenderer.invoke('connections:list'),
  saveConnection: (conn) => ipcRenderer.invoke('connections:save', conn),
  deleteConnection: (id) => ipcRenderer.invoke('connections:delete', id),
  exportConnections: () => ipcRenderer.invoke('connections:export'),
  importConnections: () => ipcRenderer.invoke('connections:import'),

  // --- SSH ---
  sshConnect: (connId) => ipcRenderer.invoke('ssh:connect', connId),
  sshShell: (sessionId) => ipcRenderer.invoke('ssh:shell', sessionId),
  sshInput: (sessionId, shellId, data) => ipcRenderer.send('ssh:input', sessionId, shellId, data),
  sshResize: (sessionId, shellId, cols, rows) => ipcRenderer.send('ssh:resize', sessionId, shellId, cols, rows),
  sshDisconnect: (sessionId) => ipcRenderer.invoke('ssh:disconnect', sessionId),

  onSshOutput: (cb) => ipcRenderer.on('ssh:output', (_e, sid, shid, data) => cb(sid, shid, data)),
  onSshClosed: (cb) => ipcRenderer.on('ssh:closed', (_e, sid) => cb(sid)),
  onSshShellClosed: (cb) => ipcRenderer.on('ssh:shell-closed', (_e, sid, shid) => cb(sid, shid)),

  // --- SFTP ---
  sftpList: (sid, p) => ipcRenderer.invoke('sftp:list', sid, p),
  sftpDownload: (sid, p) => ipcRenderer.invoke('sftp:download', sid, p),
  sftpUpload: (sid, p) => ipcRenderer.invoke('sftp:upload', sid, p),
  sftpMkdir: (sid, p) => ipcRenderer.invoke('sftp:mkdir', sid, p),
  sftpDelete: (sid, p, isDir) => ipcRenderer.invoke('sftp:delete', sid, p, isDir),
  sftpRename: (sid, o, n) => ipcRenderer.invoke('sftp:rename', sid, o, n),
  onSftpProgress: (cb) => ipcRenderer.on('sftp:progress', (_e, sid, prog) => cb(sid, prog)),

  // --- Dialog ---
  openFileDialog: (opts) => ipcRenderer.invoke('dialog:open-file', opts),
});
