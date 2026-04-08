import * as vscode from 'vscode';
import * as https from 'https';
import * as http from 'http';

export interface LLMMessage {
    role: 'system' | 'user' | 'assistant';
    content: string;
}

export interface LLMResponse {
    content: string;
    usage?: {
        prompt_tokens: number;
        completion_tokens: number;
        total_tokens: number;
    };
}

interface StreamCallback {
    onToken: (token: string) => void;
    onDone: () => void;
    onError: (error: Error) => void;
}

function getConfig() {
    const config = vscode.workspace.getConfiguration('mjjs');
    return {
        apiEndpoint: config.get<string>('apiEndpoint', 'https://api.openai.com/v1/chat/completions'),
        apiKey: config.get<string>('apiKey', ''),
        modelName: config.get<string>('modelName', 'gpt-3.5-turbo'),
        maxTokens: config.get<number>('maxTokens', 2048),
        temperature: config.get<number>('temperature', 0.3),
    };
}

/**
 * Send a non-streaming request to the LLM API (OpenAI-compatible format).
 */
export async function callLLM(messages: LLMMessage[], maxTokens?: number): Promise<LLMResponse> {
    const cfg = getConfig();

    if (!cfg.apiKey) {
        throw new Error('请先在设置中配置 API Key（设置 > MJJS AI Assistant > Api Key）');
    }

    const body = JSON.stringify({
        model: cfg.modelName,
        messages,
        max_tokens: maxTokens ?? cfg.maxTokens,
        temperature: cfg.temperature,
        stream: false,
    });

    return new Promise((resolve, reject) => {
        const url = new URL(cfg.apiEndpoint);
        const isHttps = url.protocol === 'https:';
        const options: https.RequestOptions = {
            hostname: url.hostname,
            port: url.port || (isHttps ? 443 : 80),
            path: url.pathname + url.search,
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${cfg.apiKey}`,
                'Content-Length': Buffer.byteLength(body),
            },
            timeout: 60000,
        };

        const transport = isHttps ? https : http;
        const req = transport.request(options, (res) => {
            let data = '';
            res.on('data', (chunk: Buffer) => { data += chunk.toString(); });
            res.on('end', () => {
                if (res.statusCode && res.statusCode >= 400) {
                    reject(new Error(`API 请求失败 (${res.statusCode}): ${data}`));
                    return;
                }
                try {
                    const json = JSON.parse(data);
                    if (json.error) {
                        reject(new Error(`API 错误: ${json.error.message || JSON.stringify(json.error)}`));
                        return;
                    }
                    const choice = json.choices?.[0];
                    resolve({
                        content: choice?.message?.content ?? '',
                        usage: json.usage,
                    });
                } catch (e) {
                    reject(new Error(`解析响应失败: ${data.slice(0, 200)}`));
                }
            });
        });

        req.on('error', (e) => reject(new Error(`网络错误: ${e.message}`)));
        req.on('timeout', () => { req.destroy(); reject(new Error('请求超时')); });
        req.write(body);
        req.end();
    });
}

/**
 * Send a streaming request to the LLM API. Calls onToken for each chunk.
 */
export function callLLMStream(messages: LLMMessage[], callback: StreamCallback, maxTokens?: number): { abort: () => void } {
    const cfg = getConfig();

    if (!cfg.apiKey) {
        callback.onError(new Error('请先在设置中配置 API Key'));
        return { abort: () => {} };
    }

    const body = JSON.stringify({
        model: cfg.modelName,
        messages,
        max_tokens: maxTokens ?? cfg.maxTokens,
        temperature: cfg.temperature,
        stream: true,
    });

    const url = new URL(cfg.apiEndpoint);
    const isHttps = url.protocol === 'https:';
    const options: https.RequestOptions = {
        hostname: url.hostname,
        port: url.port || (isHttps ? 443 : 80),
        path: url.pathname + url.search,
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
            'Authorization': `Bearer ${cfg.apiKey}`,
            'Content-Length': Buffer.byteLength(body),
        },
        timeout: 60000,
    };

    const transport = isHttps ? https : http;
    const req = transport.request(options, (res) => {
        if (res.statusCode && res.statusCode >= 400) {
            let data = '';
            res.on('data', (chunk: Buffer) => { data += chunk.toString(); });
            res.on('end', () => callback.onError(new Error(`API 请求失败 (${res.statusCode}): ${data}`)));
            return;
        }

        let buffer = '';
        res.on('data', (chunk: Buffer) => {
            buffer += chunk.toString();
            const lines = buffer.split('\n');
            // Keep the last potentially incomplete line in the buffer
            buffer = lines.pop() || '';

            for (const line of lines) {
                const trimmed = line.trim();
                if (!trimmed || !trimmed.startsWith('data:')) { continue; }
                const jsonStr = trimmed.slice(5).trim();
                if (jsonStr === '[DONE]') {
                    callback.onDone();
                    return;
                }
                try {
                    const json = JSON.parse(jsonStr);
                    const delta = json.choices?.[0]?.delta?.content;
                    if (delta) {
                        callback.onToken(delta);
                    }
                } catch {
                    // skip malformed chunks
                }
            }
        });

        res.on('end', () => {
            // Process any remaining data in buffer
            if (buffer.trim()) {
                const trimmed = buffer.trim();
                if (trimmed.startsWith('data:')) {
                    const jsonStr = trimmed.slice(5).trim();
                    if (jsonStr !== '[DONE]') {
                        try {
                            const json = JSON.parse(jsonStr);
                            const delta = json.choices?.[0]?.delta?.content;
                            if (delta) { callback.onToken(delta); }
                        } catch { /* ignore */ }
                    }
                }
            }
            callback.onDone();
        });
    });

    req.on('error', (e) => callback.onError(new Error(`网络错误: ${e.message}`)));
    req.on('timeout', () => { req.destroy(); callback.onError(new Error('请求超时')); });
    req.write(body);
    req.end();

    return { abort: () => req.destroy() };
}
