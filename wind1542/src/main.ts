import { Plugin, MarkdownView, Notice, TFile } from "obsidian";
import { WindPluginSettings, DEFAULT_SETTINGS } from "./types";
import { ApiService } from "./api-service";
import { DiffEngine } from "./diff-engine";
import { DiffModal } from "./diff-modal";
import { BackupManager } from "./backup-manager";
import { RollbackModal } from "./rollback-modal";
import { InstructionModal } from "./instruction-modal";
import { WindSettingTab } from "./settings";

export default class WindAIEditorPlugin extends Plugin {
	settings: WindPluginSettings = DEFAULT_SETTINGS;
	apiService!: ApiService;
	diffEngine!: DiffEngine;
	backupManager!: BackupManager;

	async onload(): Promise<void> {
		await this.loadSettings();

		this.apiService = new ApiService(this.settings);
		this.diffEngine = new DiffEngine();
		this.backupManager = new BackupManager(this.app, this.settings);

		// --- Ribbon icon ---
		this.addRibbonIcon("wand-2", "Wind AI: Modify File", () => {
			this.runAIModify();
		});

		// --- Commands ---
		this.addCommand({
			id: "wind-ai-modify",
			name: "AI Modify Current File",
			editorCallback: () => {
				this.runAIModify();
			},
		});

		this.addCommand({
			id: "wind-ai-modify-selection",
			name: "AI Modify Selection",
			editorCallback: (editor) => {
				const selection = editor.getSelection();
				if (!selection) {
					new Notice("No text selected.");
					return;
				}
				this.runAIModifySelection();
			},
		});

		this.addCommand({
			id: "wind-rollback",
			name: "Rollback File from Backup",
			editorCallback: () => {
				this.runRollback();
			},
		});

		// --- Settings tab ---
		this.addSettingTab(new WindSettingTab(this.app, this));
	}

	async loadSettings(): Promise<void> {
		this.settings = Object.assign(
			{},
			DEFAULT_SETTINGS,
			await this.loadData()
		);
	}

	async saveSettings(): Promise<void> {
		await this.saveData(this.settings);
		this.apiService?.updateSettings(this.settings);
		this.backupManager?.updateSettings(this.settings);
	}

	/**
	 * Main workflow: AI modify the entire active file.
	 */
	private async runAIModify(): Promise<void> {
		const file = this.getActiveFile();
		if (!file) return;

		// Get instruction from user
		const instructionModal = new InstructionModal(this.app);
		const instruction = await instructionModal.open();
		if (!instruction) return;

		// Read current content
		const originalContent = await this.app.vault.read(file);

		// Call API with loading notice
		let modifiedContent: string;
		const loadingNotice = new Notice("⏳ AI is processing...", 0);
		try {
			modifiedContent = await this.apiService.modifyContent(
				originalContent,
				instruction
			);
		} catch (e: unknown) {
			loadingNotice.hide();
			const msg = e instanceof Error ? e.message : String(e);
			new Notice(`AI Error: ${msg}`, 8000);
			return;
		}
		loadingNotice.hide();

		// Strip markdown code fences if the AI wrapped the output
		modifiedContent = this.stripCodeFences(modifiedContent);

		// Compute diff
		const diff = this.diffEngine.compute(originalContent, modifiedContent);

		if (diff.stats.additions === 0 && diff.stats.deletions === 0) {
			new Notice("No changes detected from AI.");
			return;
		}

		// Show floating diff modal
		const diffModal = new DiffModal(
			this.app,
			file.name,
			instruction,
			diff
		);
		const result = await diffModal.open();

		if (result.accepted) {
			// Auto-backup before applying
			if (this.settings.autoBackup) {
				try {
					await this.backupManager.backup(file, instruction);
				} catch (e: unknown) {
					const msg = e instanceof Error ? e.message : String(e);
					new Notice(`Backup warning: ${msg}`, 5000);
				}
			}

			// Apply modification
			await this.app.vault.modify(file, modifiedContent);
			new Notice(
				`✓ Applied: +${diff.stats.additions} -${diff.stats.deletions} lines`
			);
		} else {
			new Notice("Changes rejected.");
		}
	}

	/**
	 * AI modify only the selected text.
	 */
	private async runAIModifySelection(): Promise<void> {
		const view = this.app.workspace.getActiveViewOfType(MarkdownView);
		if (!view) {
			new Notice("No active markdown editor.");
			return;
		}
		const editor = view.editor;
		const file = view.file;
		if (!file) {
			new Notice("No active file.");
			return;
		}

		const selection = editor.getSelection();
		if (!selection) {
			new Notice("No text selected.");
			return;
		}

		// Get instruction
		const instructionModal = new InstructionModal(this.app);
		const instruction = await instructionModal.open();
		if (!instruction) return;

		// Call API
		let modifiedSelection: string;
		const loadingNotice = new Notice("⏳ AI is processing...", 0);
		try {
			modifiedSelection = await this.apiService.modifyContent(
				selection,
				instruction
			);
		} catch (e: unknown) {
			loadingNotice.hide();
			const msg = e instanceof Error ? e.message : String(e);
			new Notice(`AI Error: ${msg}`, 8000);
			return;
		}
		loadingNotice.hide();

		modifiedSelection = this.stripCodeFences(modifiedSelection);

		// Compute diff on selection
		const diff = this.diffEngine.compute(selection, modifiedSelection);

		if (diff.stats.additions === 0 && diff.stats.deletions === 0) {
			new Notice("No changes detected from AI.");
			return;
		}

		// Show diff
		const diffModal = new DiffModal(
			this.app,
			`${file.name} (selection)`,
			instruction,
			diff
		);
		const result = await diffModal.open();

		if (result.accepted) {
			// Backup full file before applying
			if (this.settings.autoBackup) {
				try {
					await this.backupManager.backup(file, instruction);
				} catch (e: unknown) {
					const msg = e instanceof Error ? e.message : String(e);
					new Notice(`Backup warning: ${msg}`, 5000);
				}
			}

			editor.replaceSelection(modifiedSelection);
			new Notice(
				`✓ Selection applied: +${diff.stats.additions} -${diff.stats.deletions} lines`
			);
		} else {
			new Notice("Changes rejected.");
		}
	}

	/**
	 * Open rollback modal for the active file.
	 */
	private async runRollback(): Promise<void> {
		const file = this.getActiveFile();
		if (!file) return;

		const modal = new RollbackModal(
			this.app,
			file,
			this.backupManager,
			this.diffEngine
		);
		modal.open();
	}

	private getActiveFile(): TFile | null {
		const view = this.app.workspace.getActiveViewOfType(MarkdownView);
		if (!view?.file) {
			new Notice("No active markdown file.");
			return null;
		}
		return view.file;
	}

	/**
	 * Remove markdown code fences that AI models sometimes add around output.
	 */
	private stripCodeFences(text: string): string {
		const trimmed = text.trim();
		// Match ```lang\n...\n``` pattern
		const match = trimmed.match(
			/^```[a-zA-Z]*\n([\s\S]*?)\n```$/
		);
		if (match) {
			return match[1];
		}
		return text;
	}
}
