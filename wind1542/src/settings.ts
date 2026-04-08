import { App, PluginSettingTab, Setting, Notice } from "obsidian";
import type WindAIEditorPlugin from "./main";

export class WindSettingTab extends PluginSettingTab {
	constructor(
		app: App,
		private plugin: WindAIEditorPlugin
	) {
		super(app, plugin);
	}

	display(): void {
		const { containerEl } = this;
		containerEl.empty();

		// --- API Configuration ---
		containerEl.createEl("h2", { text: "API Configuration" });

		new Setting(containerEl)
			.setName("API Provider")
			.setDesc("Select your LLM API provider")
			.addDropdown((dropdown) =>
				dropdown
					.addOption("openai", "OpenAI Compatible")
					.addOption("anthropic", "Anthropic Claude")
					.setValue(this.plugin.settings.apiProvider)
					.onChange(async (value: string) => {
						const provider = value as "openai" | "anthropic";
						this.plugin.settings.apiProvider = provider;
						// Set sensible defaults when switching
						if (provider === "anthropic") {
							this.plugin.settings.apiEndpoint =
								"https://api.anthropic.com/v1/messages";
							this.plugin.settings.model = "claude-sonnet-4-20250514";
						} else {
							this.plugin.settings.apiEndpoint =
								"https://api.openai.com/v1/chat/completions";
							this.plugin.settings.model = "gpt-4o";
						}
						await this.plugin.saveSettings();
						this.display(); // Refresh to show updated defaults
					})
			);

		new Setting(containerEl)
			.setName("API Endpoint")
			.setDesc("Full URL of the chat completions / messages endpoint")
			.addText((text) =>
				text
					.setPlaceholder("https://api.openai.com/v1/chat/completions")
					.setValue(this.plugin.settings.apiEndpoint)
					.onChange(async (value) => {
						this.plugin.settings.apiEndpoint = value;
						await this.plugin.saveSettings();
					})
			);

		new Setting(containerEl)
			.setName("API Key")
			.setDesc("Your API key (stored locally in plugin data)")
			.addText((text) => {
				text
					.setPlaceholder("sk-...")
					.setValue(this.plugin.settings.apiKey)
					.onChange(async (value) => {
						this.plugin.settings.apiKey = value;
						await this.plugin.saveSettings();
					});
				text.inputEl.type = "password";
				text.inputEl.style.width = "100%";
			});

		new Setting(containerEl)
			.setName("Model")
			.setDesc("Model identifier")
			.addText((text) =>
				text
					.setPlaceholder("gpt-4o")
					.setValue(this.plugin.settings.model)
					.onChange(async (value) => {
						this.plugin.settings.model = value;
						await this.plugin.saveSettings();
					})
			);

		new Setting(containerEl)
			.setName("Max Tokens")
			.setDesc("Maximum tokens in the API response")
			.addText((text) =>
				text
					.setPlaceholder("8192")
					.setValue(String(this.plugin.settings.maxTokens))
					.onChange(async (value) => {
						const n = parseInt(value, 10);
						if (!isNaN(n) && n > 0) {
							this.plugin.settings.maxTokens = n;
							await this.plugin.saveSettings();
						}
					})
			);

		new Setting(containerEl)
			.setName("System Prompt")
			.setDesc("Instructions sent to the AI before your modification request")
			.addTextArea((textarea) => {
				textarea
					.setPlaceholder("You are a precise file editor...")
					.setValue(this.plugin.settings.systemPrompt)
					.onChange(async (value) => {
						this.plugin.settings.systemPrompt = value;
						await this.plugin.saveSettings();
					});
				textarea.inputEl.rows = 5;
				textarea.inputEl.style.width = "100%";
			});

		// Test connection button
		new Setting(containerEl)
			.setName("Test Connection")
			.setDesc("Send a test request to verify API connectivity")
			.addButton((btn) =>
				btn
					.setButtonText("Test")
					.setCta()
					.onClick(async () => {
						btn.setButtonText("Testing...");
						btn.setDisabled(true);
						const ok = await this.plugin.apiService.testConnection();
						btn.setDisabled(false);
						if (ok) {
							btn.setButtonText("✓ Success");
							new Notice("API connection successful!");
						} else {
							btn.setButtonText("✕ Failed");
							new Notice(
								"API connection failed. Check your endpoint and key."
							);
						}
						setTimeout(() => btn.setButtonText("Test"), 3000);
					})
			);

		// --- Backup Configuration ---
		containerEl.createEl("h2", { text: "Backup & Rollback" });

		new Setting(containerEl)
			.setName("Auto Backup")
			.setDesc("Automatically create a backup before accepting AI changes")
			.addToggle((toggle) =>
				toggle
					.setValue(this.plugin.settings.autoBackup)
					.onChange(async (value) => {
						this.plugin.settings.autoBackup = value;
						await this.plugin.saveSettings();
					})
			);

		new Setting(containerEl)
			.setName("Backup Folder")
			.setDesc(
				"Folder name for backups (created in the same directory as the file)"
			)
			.addText((text) =>
				text
					.setPlaceholder(".wind-backups")
					.setValue(this.plugin.settings.backupFolder)
					.onChange(async (value) => {
						this.plugin.settings.backupFolder =
							value || ".wind-backups";
						await this.plugin.saveSettings();
					})
			);

		new Setting(containerEl)
			.setName("Max Backups Per File")
			.setDesc("Maximum number of backups to keep per file (oldest pruned first)")
			.addText((text) =>
				text
					.setPlaceholder("20")
					.setValue(String(this.plugin.settings.maxBackupsPerFile))
					.onChange(async (value) => {
						const n = parseInt(value, 10);
						if (!isNaN(n) && n > 0) {
							this.plugin.settings.maxBackupsPerFile = n;
							await this.plugin.saveSettings();
						}
					})
			);
	}
}
