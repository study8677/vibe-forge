import { requestUrl, RequestUrlResponse } from "obsidian";
import { WindPluginSettings } from "./types";

export class ApiService {
	constructor(private settings: WindPluginSettings) {}

	updateSettings(settings: WindPluginSettings): void {
		this.settings = settings;
	}

	async modifyContent(
		content: string,
		instruction: string
	): Promise<string> {
		if (!this.settings.apiKey) {
			throw new Error("API Key is not configured. Please set it in plugin settings.");
		}

		if (this.settings.apiProvider === "anthropic") {
			return this.callAnthropic(content, instruction);
		}
		return this.callOpenAI(content, instruction);
	}

	private buildUserMessage(content: string, instruction: string): string {
		return [
			`Instruction: ${instruction}`,
			"",
			"--- FILE CONTENT START ---",
			content,
			"--- FILE CONTENT END ---",
		].join("\n");
	}

	private async callOpenAI(
		content: string,
		instruction: string
	): Promise<string> {
		const body = {
			model: this.settings.model,
			max_tokens: this.settings.maxTokens,
			messages: [
				{ role: "system", content: this.settings.systemPrompt },
				{
					role: "user",
					content: this.buildUserMessage(content, instruction),
				},
			],
		};

		let response: RequestUrlResponse;
		try {
			response = await requestUrl({
				url: this.settings.apiEndpoint,
				method: "POST",
				headers: {
					"Content-Type": "application/json",
					Authorization: `Bearer ${this.settings.apiKey}`,
				},
				body: JSON.stringify(body),
			});
		} catch (e: unknown) {
			const msg = e instanceof Error ? e.message : String(e);
			throw new Error(`API request failed: ${msg}`);
		}

		const data = response.json;
		if (!data.choices?.[0]?.message?.content) {
			throw new Error(
				`Unexpected API response: ${JSON.stringify(data).slice(0, 300)}`
			);
		}
		return data.choices[0].message.content;
	}

	private async callAnthropic(
		content: string,
		instruction: string
	): Promise<string> {
		const body = {
			model: this.settings.model,
			max_tokens: this.settings.maxTokens,
			system: this.settings.systemPrompt,
			messages: [
				{
					role: "user",
					content: this.buildUserMessage(content, instruction),
				},
			],
		};

		let response: RequestUrlResponse;
		try {
			response = await requestUrl({
				url: this.settings.apiEndpoint,
				method: "POST",
				headers: {
					"Content-Type": "application/json",
					"x-api-key": this.settings.apiKey,
					"anthropic-version": "2023-06-01",
				},
				body: JSON.stringify(body),
			});
		} catch (e: unknown) {
			const msg = e instanceof Error ? e.message : String(e);
			throw new Error(`API request failed: ${msg}`);
		}

		const data = response.json;
		if (!data.content?.[0]?.text) {
			throw new Error(
				`Unexpected API response: ${JSON.stringify(data).slice(0, 300)}`
			);
		}
		return data.content[0].text;
	}

	async testConnection(): Promise<boolean> {
		try {
			await this.modifyContent("Hello world", "Return this text unchanged.");
			return true;
		} catch {
			return false;
		}
	}
}
