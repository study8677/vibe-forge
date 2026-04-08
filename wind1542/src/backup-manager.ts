import { App, TFile, TFolder, normalizePath, Notice } from "obsidian";
import { BackupIndex, BackupMeta, WindPluginSettings } from "./types";

const INDEX_FILE = "_index.json";

export class BackupManager {
	constructor(
		private app: App,
		private settings: WindPluginSettings
	) {}

	updateSettings(settings: WindPluginSettings): void {
		this.settings = settings;
	}

	/**
	 * Create a timestamped backup of a file before modification.
	 */
	async backup(file: TFile, instruction: string): Promise<BackupMeta> {
		const content = await this.app.vault.read(file);
		const timestamp = Date.now();
		const dateStr = this.formatTimestamp(timestamp);
		const baseName = file.basename;
		const ext = file.extension;
		const backupFileName = `${baseName}-${dateStr}.${ext}`;
		const backupDir = this.getBackupDir(file);
		const backupPath = normalizePath(`${backupDir}/${backupFileName}`);

		// Ensure backup directory exists
		await this.ensureDir(backupDir);

		// Write backup file
		await this.app.vault.create(backupPath, content);

		const meta: BackupMeta = {
			filePath: file.path,
			backupFileName,
			timestamp,
			instruction,
			charCount: content.length,
		};

		// Update index
		await this.addToIndex(backupDir, meta);

		// Prune old backups
		await this.pruneBackups(file);

		return meta;
	}

	/**
	 * Restore a file from a specific backup.
	 */
	async restore(file: TFile, meta: BackupMeta): Promise<void> {
		const backupDir = this.getBackupDir(file);
		const backupPath = normalizePath(`${backupDir}/${meta.backupFileName}`);
		const backupFile = this.app.vault.getAbstractFileByPath(backupPath);

		if (!(backupFile instanceof TFile)) {
			throw new Error(`Backup file not found: ${backupPath}`);
		}

		const content = await this.app.vault.read(backupFile);
		await this.app.vault.modify(file, content);
	}

	/**
	 * List all backups for a given file, newest first.
	 */
	async listBackups(file: TFile): Promise<BackupMeta[]> {
		const backupDir = this.getBackupDir(file);
		const index = await this.readIndex(backupDir);
		return index.entries
			.filter((e) => e.filePath === file.path)
			.sort((a, b) => b.timestamp - a.timestamp);
	}

	/**
	 * Delete a specific backup.
	 */
	async deleteBackup(file: TFile, meta: BackupMeta): Promise<void> {
		const backupDir = this.getBackupDir(file);
		const backupPath = normalizePath(`${backupDir}/${meta.backupFileName}`);
		const backupFile = this.app.vault.getAbstractFileByPath(backupPath);

		if (backupFile instanceof TFile) {
			await this.app.vault.delete(backupFile);
		}

		// Remove from index
		const index = await this.readIndex(backupDir);
		index.entries = index.entries.filter(
			(e) =>
				!(
					e.filePath === meta.filePath &&
					e.timestamp === meta.timestamp
				)
		);
		await this.writeIndex(backupDir, index);
	}

	/**
	 * Preview backup content without restoring.
	 */
	async readBackupContent(file: TFile, meta: BackupMeta): Promise<string> {
		const backupDir = this.getBackupDir(file);
		const backupPath = normalizePath(`${backupDir}/${meta.backupFileName}`);
		const backupFile = this.app.vault.getAbstractFileByPath(backupPath);

		if (!(backupFile instanceof TFile)) {
			throw new Error(`Backup file not found: ${backupPath}`);
		}

		return this.app.vault.read(backupFile);
	}

	// --- Private helpers ---

	private getBackupDir(file: TFile): string {
		const parentDir = file.parent?.path || "";
		return normalizePath(
			parentDir
				? `${parentDir}/${this.settings.backupFolder}`
				: this.settings.backupFolder
		);
	}

	private async ensureDir(path: string): Promise<void> {
		const parts = path.split("/");
		let current = "";
		for (const part of parts) {
			current = current ? `${current}/${part}` : part;
			const normalized = normalizePath(current);
			const existing = this.app.vault.getAbstractFileByPath(normalized);
			if (!existing) {
				await this.app.vault.createFolder(normalized);
			}
		}
	}

	private async readIndex(backupDir: string): Promise<BackupIndex> {
		const indexPath = normalizePath(`${backupDir}/${INDEX_FILE}`);
		const indexFile = this.app.vault.getAbstractFileByPath(indexPath);

		if (indexFile instanceof TFile) {
			try {
				const raw = await this.app.vault.read(indexFile);
				return JSON.parse(raw) as BackupIndex;
			} catch {
				return { version: 1, entries: [] };
			}
		}

		return { version: 1, entries: [] };
	}

	private async writeIndex(
		backupDir: string,
		index: BackupIndex
	): Promise<void> {
		const indexPath = normalizePath(`${backupDir}/${INDEX_FILE}`);
		const content = JSON.stringify(index, null, 2);
		const existing = this.app.vault.getAbstractFileByPath(indexPath);

		if (existing instanceof TFile) {
			await this.app.vault.modify(existing, content);
		} else {
			await this.ensureDir(backupDir);
			await this.app.vault.create(indexPath, content);
		}
	}

	private async addToIndex(
		backupDir: string,
		meta: BackupMeta
	): Promise<void> {
		const index = await this.readIndex(backupDir);
		index.entries.push(meta);
		await this.writeIndex(backupDir, index);
	}

	private async pruneBackups(file: TFile): Promise<void> {
		const backups = await this.listBackups(file);
		const excess = backups.slice(this.settings.maxBackupsPerFile);

		for (const old of excess) {
			try {
				await this.deleteBackup(file, old);
			} catch {
				// Silently ignore cleanup errors
			}
		}
	}

	private formatTimestamp(ts: number): string {
		const d = new Date(ts);
		const pad = (n: number) => String(n).padStart(2, "0");
		return [
			d.getFullYear(),
			pad(d.getMonth() + 1),
			pad(d.getDate()),
			"-",
			pad(d.getHours()),
			pad(d.getMinutes()),
			pad(d.getSeconds()),
		].join("");
	}
}
