import * as vscode from 'vscode';
import { callLLM, LLMMessage } from './llmClient';

/**
 * Inline completion provider that calls the configured LLM for code suggestions.
 * Uses debouncing to avoid excessive API calls while the user is typing.
 */
export class LLMCompletionProvider implements vscode.InlineCompletionItemProvider {
    private debounceTimer: ReturnType<typeof setTimeout> | undefined;
    private abortController: AbortController | undefined;
    private lastRequestId = 0;
    private statusBarItem: vscode.StatusBarItem;

    constructor(statusBarItem: vscode.StatusBarItem) {
        this.statusBarItem = statusBarItem;
    }

    async provideInlineCompletionItems(
        document: vscode.TextDocument,
        position: vscode.Position,
        context: vscode.InlineCompletionContext,
        token: vscode.CancellationToken
    ): Promise<vscode.InlineCompletionItem[] | undefined> {
        const config = vscode.workspace.getConfiguration('mjjs');
        if (!config.get<boolean>('completionEnabled', true)) {
            return undefined;
        }

        if (!config.get<string>('apiKey', '')) {
            return undefined;
        }

        // Cancel any pending request
        if (this.debounceTimer) {
            clearTimeout(this.debounceTimer);
        }

        const requestId = ++this.lastRequestId;
        const debounceMs = config.get<number>('completionDebounceMs', 500);

        return new Promise((resolve) => {
            this.debounceTimer = setTimeout(async () => {
                if (token.isCancellationRequested || requestId !== this.lastRequestId) {
                    resolve(undefined);
                    return;
                }

                try {
                    const completion = await this.getCompletion(document, position, config, token, requestId);
                    if (!completion || requestId !== this.lastRequestId) {
                        resolve(undefined);
                        return;
                    }
                    resolve([new vscode.InlineCompletionItem(completion, new vscode.Range(position, position))]);
                } catch {
                    resolve(undefined);
                }
            }, debounceMs);

            // If the token is cancelled, clear the timer
            token.onCancellationRequested(() => {
                if (this.debounceTimer) {
                    clearTimeout(this.debounceTimer);
                }
                resolve(undefined);
            });
        });
    }

    private async getCompletion(
        document: vscode.TextDocument,
        position: vscode.Position,
        config: vscode.WorkspaceConfiguration,
        token: vscode.CancellationToken,
        requestId: number
    ): Promise<string | undefined> {
        const maxLines = config.get<number>('completionMaxLines', 50);
        const systemPrompt = config.get<string>(
            'completionSystemPrompt',
            'You are a code completion engine. Only output the code that should come next. Do not include explanations, markdown formatting, or the existing code. Output raw code only.'
        );

        // Gather context: lines before and after cursor
        const startLine = Math.max(0, position.line - maxLines);
        const endLine = Math.min(document.lineCount - 1, position.line + 10);

        const prefix = document.getText(new vscode.Range(startLine, 0, position.line, position.character));
        const suffix = document.getText(new vscode.Range(position.line, position.character, endLine, document.lineAt(endLine).text.length));

        const language = document.languageId;
        const fileName = document.fileName.split('/').pop() || '';

        const userPrompt = [
            `File: ${fileName} (${language})`,
            `Complete the code at the cursor position marked by <CURSOR>.`,
            `Only output the new code to insert. Do not repeat existing code. Keep it concise (1-5 lines typically).`,
            '',
            '```' + language,
            prefix + '<CURSOR>' + suffix,
            '```',
        ].join('\n');

        const messages: LLMMessage[] = [
            { role: 'system', content: systemPrompt },
            { role: 'user', content: userPrompt },
        ];

        this.statusBarItem.text = '$(loading~spin) AI 补全中...';
        this.statusBarItem.show();

        try {
            if (token.isCancellationRequested || requestId !== this.lastRequestId) {
                return undefined;
            }
            const response = await callLLM(messages, 256);
            return this.cleanCompletion(response.content, language);
        } finally {
            this.statusBarItem.text = '$(sparkle) MJJS AI';
            this.statusBarItem.show();
        }
    }

    /**
     * Clean up the completion result — strip markdown fences and leading/trailing whitespace.
     */
    private cleanCompletion(raw: string, language: string): string | undefined {
        let result = raw.trim();
        if (!result) { return undefined; }

        // Strip markdown code fences if present
        const fenceRegex = new RegExp('^```(?:' + language + ')?\\s*\\n?', 'i');
        result = result.replace(fenceRegex, '');
        result = result.replace(/\n?```\s*$/, '');

        result = result.trim();
        return result || undefined;
    }
}
