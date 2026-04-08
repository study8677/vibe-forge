import { App, Modal } from "obsidian";
import { DiffResult, DiffLine, CharDiff } from "./types";

export interface DiffModalResult {
	accepted: boolean;
}

/**
 * Floating modal that displays a unified diff with accept/reject controls.
 */
export class DiffModal extends Modal {
	private result: DiffModalResult = { accepted: false };
	private resolvePromise: ((value: DiffModalResult) => void) | null = null;

	constructor(
		app: App,
		private fileName: string,
		private instruction: string,
		private diff: DiffResult
	) {
		super(app);
	}

	/**
	 * Show the modal and return a promise that resolves when user acts.
	 */
	open(): Promise<DiffModalResult> {
		return new Promise((resolve) => {
			this.resolvePromise = resolve;
			super.open();
		});
	}

	onOpen(): void {
		const { contentEl, modalEl } = this;
		modalEl.addClass("wind-diff-modal");

		// Prevent default close behavior — force explicit accept/reject
		// But keep Esc to reject
		this.scope.register([], "Escape", () => {
			this.reject();
			return false;
		});
		this.scope.register([], "Enter", () => {
			this.accept();
			return false;
		});

		// --- Header ---
		const header = contentEl.createDiv({ cls: "wind-diff-header" });

		const titleRow = header.createDiv({ cls: "wind-diff-title-row" });
		titleRow.createSpan({
			cls: "wind-diff-filename",
			text: this.fileName,
		});

		const statsEl = titleRow.createSpan({ cls: "wind-diff-stats" });
		const { additions, deletions } = this.diff.stats;
		if (additions > 0) {
			statsEl.createSpan({
				cls: "wind-diff-stat-add",
				text: `+${additions}`,
			});
		}
		if (deletions > 0) {
			statsEl.createSpan({
				cls: "wind-diff-stat-del",
				text: `-${deletions}`,
			});
		}
		if (additions === 0 && deletions === 0) {
			statsEl.createSpan({
				cls: "wind-diff-stat-none",
				text: "No changes",
			});
		}

		if (this.instruction) {
			header.createDiv({
				cls: "wind-diff-instruction",
				text: `"${this.instruction}"`,
			});
		}

		// --- Diff Content ---
		const diffContainer = contentEl.createDiv({ cls: "wind-diff-container" });
		const table = diffContainer.createEl("table", { cls: "wind-diff-table" });
		const tbody = table.createEl("tbody");

		for (const line of this.diff.lines) {
			this.renderDiffLine(tbody, line);
		}

		// --- Footer with buttons ---
		const footer = contentEl.createDiv({ cls: "wind-diff-footer" });

		const hint = footer.createDiv({ cls: "wind-diff-hint" });
		hint.createSpan({ text: "Enter to accept · Esc to reject" });

		const btnGroup = footer.createDiv({ cls: "wind-diff-buttons" });

		const rejectBtn = btnGroup.createEl("button", {
			cls: "wind-diff-btn wind-diff-btn-reject",
		});
		rejectBtn.createSpan({ cls: "wind-diff-btn-icon", text: "✕" });
		rejectBtn.createSpan({ text: " Reject" });
		rejectBtn.addEventListener("click", () => this.reject());

		const acceptBtn = btnGroup.createEl("button", {
			cls: "wind-diff-btn wind-diff-btn-accept",
		});
		acceptBtn.createSpan({ cls: "wind-diff-btn-icon", text: "✓" });
		acceptBtn.createSpan({ text: " Accept" });
		acceptBtn.addEventListener("click", () => this.accept());

		// Auto-focus accept button
		acceptBtn.focus();
	}

	private renderDiffLine(tbody: HTMLTableSectionElement, line: DiffLine): void {
		const tr = tbody.createEl("tr", {
			cls: `wind-diff-line wind-diff-line-${line.type}`,
		});

		// Old line number
		tr.createEl("td", {
			cls: "wind-diff-linenum wind-diff-linenum-old",
			text: line.oldLineNo !== null ? String(line.oldLineNo) : "",
		});

		// New line number
		tr.createEl("td", {
			cls: "wind-diff-linenum wind-diff-linenum-new",
			text: line.newLineNo !== null ? String(line.newLineNo) : "",
		});

		// Gutter symbol
		const gutterText =
			line.type === "add" ? "+" : line.type === "remove" ? "-" : " ";
		tr.createEl("td", {
			cls: "wind-diff-gutter",
			text: gutterText,
		});

		// Content cell
		const contentTd = tr.createEl("td", { cls: "wind-diff-content" });

		if (line.charDiffs && line.charDiffs.length > 0) {
			this.renderCharDiffs(contentTd, line.charDiffs);
		} else {
			contentTd.setText(line.content || " ");
		}
	}

	private renderCharDiffs(container: HTMLElement, diffs: CharDiff[]): void {
		for (const d of diffs) {
			if (d.type === "unchanged") {
				container.createSpan({ text: d.value });
			} else if (d.type === "add") {
				container.createSpan({
					cls: "wind-diff-char-add",
					text: d.value,
				});
			} else {
				container.createSpan({
					cls: "wind-diff-char-remove",
					text: d.value,
				});
			}
		}
	}

	private accept(): void {
		this.result = { accepted: true };
		this.close();
	}

	private reject(): void {
		this.result = { accepted: false };
		this.close();
	}

	onClose(): void {
		this.contentEl.empty();
		if (this.resolvePromise) {
			this.resolvePromise(this.result);
			this.resolvePromise = null;
		}
	}
}
