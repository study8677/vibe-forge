import * as vscode from 'vscode';
import { callLLMStream, LLMMessage } from './llmClient';
import { showResultPanel, appendToPanel, setPanelDone, setPanelError } from './resultPanel';

let activeRequest: { abort: () => void } | undefined;

interface CommandConfig {
    title: string;
    prompt: string;
}

const COMMANDS: Record<string, CommandConfig> = {
    'mjjs.explainCode': {
        title: '解释代码',
        prompt: '请简要解释以下代码的功能和逻辑：',
    },
    'mjjs.explainCodeDetail': {
        title: '详细解释代码',
        prompt: '请详细解释以下代码，包括：\n1. 整体功能概述\n2. 逐行/逐块解释\n3. 关键算法或设计模式\n4. 输入输出说明\n5. 可能的边界情况',
    },
    'mjjs.optimizeCode': {
        title: '优化代码',
        prompt: '请分析以下代码并给出优化建议，包括性能优化、可读性提升、最佳实践等。如果有优化方案，请给出优化后的代码：',
    },
    'mjjs.addComments': {
        title: '添加注释',
        prompt: '请为以下代码添加详细的中文注释。保持原有代码不变，只添加注释。对于函数请添加文档注释（JSDoc/docstring等），对于关键逻辑添加行内注释：',
    },
    'mjjs.findBugs': {
        title: '查找潜在问题',
        prompt: '请分析以下代码中可能存在的问题，包括：\n1. 潜在的 Bug\n2. 安全隐患\n3. 性能问题\n4. 错误处理不当\n5. 边界条件未处理\n\n如果发现问题，请给出修复建议：',
    },
};

function getSelectedCode(editor: vscode.TextEditor): { code: string; language: string; fileName: string } | undefined {
    const selection = editor.selection;
    if (selection.isEmpty) {
        vscode.window.showWarningMessage('请先选中要分析的代码');
        return undefined;
    }
    return {
        code: editor.document.getText(selection),
        language: editor.document.languageId,
        fileName: editor.document.fileName.split('/').pop() || '',
    };
}

function executeCommand(commandId: string, extensionUri: vscode.Uri) {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        vscode.window.showWarningMessage('请先打开一个文件');
        return;
    }

    const selected = getSelectedCode(editor);
    if (!selected) { return; }

    const cmdConfig = COMMANDS[commandId];
    if (!cmdConfig) { return; }

    // Cancel any in-flight request
    if (activeRequest) {
        activeRequest.abort();
        activeRequest = undefined;
    }

    const config = vscode.workspace.getConfiguration('mjjs');
    const systemPrompt = config.get<string>('systemPrompt', '你是一个专业的编程助手。请用中文回答。');

    const messages: LLMMessage[] = [
        { role: 'system', content: systemPrompt },
        {
            role: 'user',
            content: `${cmdConfig.prompt}\n\n文件: ${selected.fileName}\n语言: ${selected.language}\n\n\`\`\`${selected.language}\n${selected.code}\n\`\`\``,
        },
    ];

    const panel = showResultPanel(`AI: ${cmdConfig.title}`, extensionUri);
    let totalContent = '';

    activeRequest = callLLMStream(messages, {
        onToken(token) {
            totalContent += token;
            appendToPanel(panel, token);
        },
        onDone() {
            activeRequest = undefined;
            setPanelDone(panel, `共 ${totalContent.length} 字`);
        },
        onError(err) {
            activeRequest = undefined;
            setPanelError(panel, err.message);
            vscode.window.showErrorMessage(`AI 助手出错: ${err.message}`);
        },
    });
}

export function registerExplainCommands(context: vscode.ExtensionContext) {
    for (const commandId of Object.keys(COMMANDS)) {
        context.subscriptions.push(
            vscode.commands.registerCommand(commandId, () => {
                executeCommand(commandId, context.extensionUri);
            })
        );
    }
}
