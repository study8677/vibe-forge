import { App, Modal, TFile, Notice } from "obsidian";
import { BackupMeta } from "./types";
import { BackupManager } from "./backup-manager";
import { DiffEngine } from "./diff-engine";
import { DiffModal } from "./diff-modal";

/**
 * Modal for browsing and restoring timestamped backups.
 */
export class RollbackModal extends Modal {
	private backups: BackupMeta[] = [];

	constructor(
		app: App,
		private file: TFile,
		private backupManager: BackupManager,
		private diffEngine: DiffEngine
	) {
		super(app);
	}

	async onOpen(): Promise<void> {
		const { contentEl, modalEl } = this;
		modalEl.addClass("wind-rollback-modal");

		contentEl.createEl("h3", {
			text: `Backups — ${this.file.name}`,
			cls: "wind-rollback-title",
		});

		const listContainer = contentEl.createDiv({
			cls: "wind-rollback-list",
		});

		try {
			this.backups = await this.backupManager.listBackups(this.file);
		} catch {
			listContainer.createDiv({
				cls: "wind-rollback-empty",
				text: "Failed to load backups.",
			});
			return;
		}

		if (this.backups.length === 0) {
			listContainer.createDiv({
				cls: "wind-rollback-empty",
				text: "No backups found for this file.",
			});
			return;
		}

		for (const backup of this.backups) {
			this.renderBackupEntry(listContainer, backup);
		}
	}

	private renderBackupEntry(
		container: HTMLElement,
		backup: BackupMeta
	): void {
		const entry = container.createDiv({ cls: "wind-rollback-entry" });

		const info = entry.createDiv({ cls: "wind-rollback-info" });
		const date = new Date(backup.timestamp);
		info.createDiv({
			cls: "wind-rollback-date",
			text: date.toLocaleString(),
		});
		info.createDiv({
			cls: "wind-rollback-instruction",
			text: backup.instruction || "(no instruction)",
		});
		info.createDiv({
			cls: "wind-rollback-size",
			text: `${backup.charCount.toLocaleString()} chars`,
		});

		const actions = entry.createDiv({ cls: "wind-rollback-actions" });

		// Preview (diff) button
		const previewBtn = actions.createEl("button", {
			cls: "wind-rollback-btn wind-rollback-btn-preview",
			text: "Diff",
		});
		previewBtn.addEventListener("click", async () => {
			await this.previewBackup(backup);
		});

		// Restore button
		const restoreBtn = actions.createEl("button", {
			cls: "wind-rollback-btn wind-rollback-btn-restore",
			text: "Restore",
		});
		restoreBtn.addEventListener("click", async () => {
			await this.restoreBackup(backup);
		});

		// Delete button
		const deleteBtn = actions.createEl("button", {
			cls: "wind-rollback-btn wind-rollback-btn-delete",
			text: "Delete",
		});
		deleteBtn.addEventListener("click", async () => {
			await this.deleteBackup(backup, entry);
		});
	}

	private async previewBackup(backup: BackupMeta): Promise<void> {
		try {
			const currentContent = await this.app.vault.read(this.file);
			const backupContent =
				await this.backupManager.readBackupContent(this.file, backup);
			const diff = this.diffEngine.compute(currentContent, backupContent);

			const modal = new DiffModal(
				this.app,
				`${this.file.name} ← backup ${new Date(backup.timestamp).toLocaleString()}`,
				`Restore: "${backup.instruction}"`,
				diff
			);
			const result = await modal.open();

			if (result.accepted) {
				await this.backupManager.restore(this.file, backup);
				new Notice(`Restored from backup: ${new Date(backup.timestamp).toLocaleString()}`);
				this.close();
			}
		} catch (e: unknown) {
			const msg = e instanceof Error ? e.message : String(e);
			new Notice(`Preview failed: ${msg}`);
		}
	}

	private async restoreBackup(backup: BackupMeta): Promise<void> {
		try {
			await this.backupManager.restore(this.file, backup);
			new Notice(
				`Restored from backup: ${new Date(backup.timestamp).toLocaleString()}`
			);
			this.close();
		} catch (e: unknown) {
			const msg = e instanceof Error ? e.message : String(e);
			new Notice(`Restore failed: ${msg}`);
		}
	}

	private async deleteBackup(
		backup: BackupMeta,
		entryEl: HTMLElement
	): Promise<void> {
		try {
			await this.backupManager.deleteBackup(this.file, backup);
			entryEl.remove();
			this.backups = this.backups.filter(
				(b) => b.timestamp !== backup.timestamp
			);

			if (this.backups.length === 0) {
				this.contentEl
					.querySelector(".wind-rollback-list")
					?.createDiv({
						cls: "wind-rollback-empty",
						text: "No backups remaining.",
					});
			}

			new Notice("Backup deleted.");
		} catch (e: unknown) {
			const msg = e instanceof Error ? e.message : String(e);
			new Notice(`Delete failed: ${msg}`);
		}
	}

	onClose(): void {
		this.contentEl.empty();
	}
}
