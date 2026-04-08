import * as vscode from 'vscode';
import { registerExplainCommands } from './explainCommands';
import { LLMCompletionProvider } from './completionProvider';

export function activate(context: vscode.ExtensionContext) {
    console.log('MJJS AI Assistant 已激活');

    // --- Status bar ---
    const statusBar = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
    statusBar.text = '$(sparkle) MJJS AI';
    statusBar.tooltip = 'MJJS AI 助手 — 点击打开设置';
    statusBar.command = {
        title: '打开 MJJS AI 设置',
        command: 'workbench.action.openSettings',
        arguments: ['mjjs'],
    };
    statusBar.show();
    context.subscriptions.push(statusBar);

    // --- Right-click explain / optimize / etc. ---
    registerExplainCommands(context);

    // --- Inline completion ---
    const completionProvider = new LLMCompletionProvider(statusBar);
    context.subscriptions.push(
        vscode.languages.registerInlineCompletionItemProvider(
            { pattern: '**' },   // all files
            completionProvider
        )
    );

    // --- Manual trigger command ---
    context.subscriptions.push(
        vscode.commands.registerCommand('mjjs.triggerCompletion', () => {
            vscode.commands.executeCommand('editor.action.inlineSuggest.trigger');
        })
    );

    // --- First-launch guide ---
    const config = vscode.workspace.getConfiguration('mjjs');
    if (!config.get<string>('apiKey', '')) {
        vscode.window
            .showInformationMessage(
                'MJJS AI 助手：请先配置 API Key 和模型地址以启用 AI 功能',
                '打开设置'
            )
            .then((action) => {
                if (action === '打开设置') {
                    vscode.commands.executeCommand('workbench.action.openSettings', 'mjjs');
                }
            });
    }
}

export function deactivate() {
    console.log('MJJS AI Assistant 已停用');
}
