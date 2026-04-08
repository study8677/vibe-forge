export interface WindPluginSettings {
	apiProvider: "openai" | "anthropic";
	apiEndpoint: string;
	apiKey: string;
	model: string;
	systemPrompt: string;
	maxTokens: number;
	backupFolder: string;
	maxBackupsPerFile: number;
	diffViewMode: "unified" | "split";
	autoBackup: boolean;
}

export const DEFAULT_SETTINGS: WindPluginSettings = {
	apiProvider: "openai",
	apiEndpoint: "https://api.openai.com/v1/chat/completions",
	apiKey: "",
	model: "gpt-4o",
	systemPrompt:
		"You are a precise file editor. Modify the given content according to the user's instruction. Return ONLY the modified content with no explanations, no markdown code fences, no extra text. Preserve the original formatting style.",
	maxTokens: 8192,
	backupFolder: ".wind-backups",
	maxBackupsPerFile: 20,
	diffViewMode: "unified",
	autoBackup: true,
};

export interface DiffLine {
	type: "add" | "remove" | "unchanged";
	oldLineNo: number | null;
	newLineNo: number | null;
	content: string;
	charDiffs?: CharDiff[];
}

export interface CharDiff {
	type: "add" | "remove" | "unchanged";
	value: string;
}

export interface DiffResult {
	lines: DiffLine[];
	stats: {
		additions: number;
		deletions: number;
		unchanged: number;
	};
}

export interface BackupMeta {
	filePath: string;
	backupFileName: string;
	timestamp: number;
	instruction: string;
	charCount: number;
}

export interface BackupIndex {
	version: 1;
	entries: BackupMeta[];
}
