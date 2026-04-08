import * as vscode from 'vscode';

let currentPanel: vscode.WebviewPanel | undefined;

function getWebviewContent(title: string, initialContent: string): string {
    return /*html*/`<!DOCTYPE html>
<html lang="zh-CN">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<style>
    :root {
        --bg: var(--vscode-editor-background);
        --fg: var(--vscode-editor-foreground);
        --border: var(--vscode-panel-border);
        --link: var(--vscode-textLink-foreground);
        --code-bg: var(--vscode-textCodeBlock-background);
        --badge-bg: var(--vscode-badge-background);
        --badge-fg: var(--vscode-badge-foreground);
    }
    * { margin: 0; padding: 0; box-sizing: border-box; }
    body {
        font-family: var(--vscode-font-family, -apple-system, sans-serif);
        font-size: var(--vscode-font-size, 13px);
        color: var(--fg);
        background: var(--bg);
        padding: 16px;
        line-height: 1.6;
    }
    h1 { font-size: 1.3em; margin-bottom: 12px; display: flex; align-items: center; gap: 8px; }
    h1 .badge {
        font-size: 0.65em;
        background: var(--badge-bg);
        color: var(--badge-fg);
        padding: 2px 8px;
        border-radius: 10px;
        font-weight: normal;
    }
    #content {
        white-space: pre-wrap;
        word-wrap: break-word;
    }
    /* Basic markdown rendering */
    #content h2, #content h3, #content h4 {
        margin-top: 16px;
        margin-bottom: 8px;
    }
    pre {
        background: var(--code-bg);
        padding: 12px;
        border-radius: 6px;
        overflow-x: auto;
        margin: 8px 0;
        font-family: var(--vscode-editor-font-family, monospace);
        font-size: var(--vscode-editor-font-size, 13px);
    }
    code {
        font-family: var(--vscode-editor-font-family, monospace);
        background: var(--code-bg);
        padding: 1px 4px;
        border-radius: 3px;
    }
    pre code { background: none; padding: 0; }
    .loading {
        display: inline-block;
        width: 12px;
        height: 12px;
        border: 2px solid var(--fg);
        border-top-color: transparent;
        border-radius: 50%;
        animation: spin 0.8s linear infinite;
        margin-left: 8px;
        vertical-align: middle;
    }
    @keyframes spin { to { transform: rotate(360deg); } }
    .status { color: var(--vscode-descriptionForeground); font-size: 0.9em; margin-bottom: 12px; }
    hr { border: none; border-top: 1px solid var(--border); margin: 12px 0; }
</style>
</head>
<body>
    <h1>${escapeHtml(title)} <span class="badge" id="status-badge">生成中</span></h1>
    <div class="status" id="status"><span class="loading"></span> 正在调用大模型...</div>
    <hr>
    <div id="content">${escapeHtml(initialContent)}</div>
    <script>
        const vscode = acquireVsCodeApi();
        const contentEl = document.getElementById('content');
        const statusEl = document.getElementById('status');
        const badgeEl = document.getElementById('status-badge');

        window.addEventListener('message', event => {
            const msg = event.data;
            switch (msg.type) {
                case 'append':
                    contentEl.textContent += msg.text;
                    break;
                case 'replace':
                    contentEl.innerHTML = msg.html;
                    break;
                case 'done':
                    statusEl.textContent = msg.usage || '完成';
                    badgeEl.textContent = '完成';
                    badgeEl.style.background = 'var(--vscode-testing-iconPassed)';
                    break;
                case 'error':
                    statusEl.textContent = '出错: ' + msg.message;
                    badgeEl.textContent = '错误';
                    badgeEl.style.background = 'var(--vscode-testing-iconFailed)';
                    break;
            }
        });
    </script>
</body>
</html>`;
}

function escapeHtml(text: string): string {
    return text.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
}

export function showResultPanel(title: string, extensionUri: vscode.Uri): vscode.WebviewPanel {
    if (currentPanel) {
        currentPanel.reveal(vscode.ViewColumn.Beside);
        currentPanel.title = title;
        currentPanel.webview.html = getWebviewContent(title, '');
        return currentPanel;
    }

    currentPanel = vscode.window.createWebviewPanel(
        'mjjsAiResult',
        title,
        { viewColumn: vscode.ViewColumn.Beside, preserveFocus: true },
        { enableScripts: true, retainContextWhenHidden: true }
    );

    currentPanel.webview.html = getWebviewContent(title, '');

    currentPanel.onDidDispose(() => {
        currentPanel = undefined;
    });

    return currentPanel;
}

export function appendToPanel(panel: vscode.WebviewPanel, text: string) {
    panel.webview.postMessage({ type: 'append', text });
}

export function setPanelDone(panel: vscode.WebviewPanel, usage?: string) {
    panel.webview.postMessage({ type: 'done', usage: usage || '' });
}

export function setPanelError(panel: vscode.WebviewPanel, message: string) {
    panel.webview.postMessage({ type: 'error', message });
}
