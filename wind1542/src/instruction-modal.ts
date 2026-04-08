import { App, Modal } from "obsidian";

/**
 * Simple modal to capture the user's modification instruction.
 */
export class InstructionModal extends Modal {
	private instruction = "";
	private resolvePromise: ((value: string | null) => void) | null = null;

	constructor(app: App) {
		super(app);
	}

	open(): Promise<string | null> {
		return new Promise((resolve) => {
			this.resolvePromise = resolve;
			super.open();
		});
	}

	onOpen(): void {
		const { contentEl, modalEl } = this;
		modalEl.addClass("wind-instruction-modal");

		contentEl.createEl("h3", {
			text: "AI Modification Instruction",
			cls: "wind-instruction-title",
		});

		contentEl.createEl("p", {
			text: "Describe what changes you want to make to this file:",
			cls: "wind-instruction-desc",
		});

		const textarea = contentEl.createEl("textarea", {
			cls: "wind-instruction-input",
			attr: {
				placeholder:
					"e.g., Fix grammar errors, add type annotations, refactor to use async/await...",
				rows: "4",
			},
		});

		textarea.addEventListener("keydown", (e) => {
			if (e.key === "Enter" && (e.metaKey || e.ctrlKey)) {
				e.preventDefault();
				this.submit(textarea.value);
			}
		});

		const footer = contentEl.createDiv({ cls: "wind-instruction-footer" });

		footer.createDiv({
			cls: "wind-instruction-hint",
			text: "Ctrl/Cmd + Enter to submit",
		});

		const btnGroup = footer.createDiv({ cls: "wind-instruction-buttons" });

		const cancelBtn = btnGroup.createEl("button", {
			text: "Cancel",
			cls: "wind-instruction-btn-cancel",
		});
		cancelBtn.addEventListener("click", () => this.cancel());

		const submitBtn = btnGroup.createEl("button", {
			text: "Send to AI",
			cls: "wind-instruction-btn-submit mod-cta",
		});
		submitBtn.addEventListener("click", () =>
			this.submit(textarea.value)
		);

		// Focus the textarea
		setTimeout(() => textarea.focus(), 50);
	}

	private submit(value: string): void {
		const trimmed = value.trim();
		if (!trimmed) return;
		this.instruction = trimmed;
		this.close();
	}

	private cancel(): void {
		this.instruction = "";
		this.close();
	}

	onClose(): void {
		if (this.resolvePromise) {
			this.resolvePromise(this.instruction || null);
			this.resolvePromise = null;
		}
		this.contentEl.empty();
	}
}
