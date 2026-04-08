import { DiffLine, DiffResult, CharDiff } from "./types";

/**
 * Compute unified diff between two texts using Myers-like LCS algorithm.
 * Also computes character-level diffs for changed line pairs.
 */
export class DiffEngine {
	compute(original: string, modified: string): DiffResult {
		const oldLines = original.split("\n");
		const newLines = modified.split("\n");
		const ops = this.myersDiff(oldLines, newLines);

		const diffLines: DiffLine[] = [];
		let oldLineNo = 1;
		let newLineNo = 1;
		let stats = { additions: 0, deletions: 0, unchanged: 0 };

		// Group consecutive removes/adds for char-level diff
		let i = 0;
		while (i < ops.length) {
			const op = ops[i];
			if (op.type === "equal") {
				diffLines.push({
					type: "unchanged",
					oldLineNo: oldLineNo++,
					newLineNo: newLineNo++,
					content: op.value,
				});
				stats.unchanged++;
				i++;
			} else {
				// Collect consecutive remove/insert block
				const removes: string[] = [];
				const adds: string[] = [];
				while (i < ops.length && ops[i].type !== "equal") {
					if (ops[i].type === "delete") {
						removes.push(ops[i].value);
					} else {
						adds.push(ops[i].value);
					}
					i++;
				}

				// Pair up removes and adds for char-level diff
				const pairCount = Math.min(removes.length, adds.length);
				for (let p = 0; p < pairCount; p++) {
					const charDiffs = this.charDiff(removes[p], adds[p]);
					diffLines.push({
						type: "remove",
						oldLineNo: oldLineNo++,
						newLineNo: null,
						content: removes[p],
						charDiffs: charDiffs.old,
					});
					diffLines.push({
						type: "add",
						oldLineNo: null,
						newLineNo: newLineNo++,
						content: adds[p],
						charDiffs: charDiffs.new,
					});
					stats.deletions++;
					stats.additions++;
				}
				// Remaining removes
				for (let p = pairCount; p < removes.length; p++) {
					diffLines.push({
						type: "remove",
						oldLineNo: oldLineNo++,
						newLineNo: null,
						content: removes[p],
					});
					stats.deletions++;
				}
				// Remaining adds
				for (let p = pairCount; p < adds.length; p++) {
					diffLines.push({
						type: "add",
						oldLineNo: null,
						newLineNo: newLineNo++,
						content: adds[p],
					});
					stats.additions++;
				}
			}
		}

		return { lines: diffLines, stats };
	}

	/**
	 * Myers diff algorithm for line-level comparison.
	 */
	private myersDiff(
		a: string[],
		b: string[]
	): { type: "equal" | "delete" | "insert"; value: string }[] {
		const n = a.length;
		const m = b.length;
		const max = n + m;

		// For very large files, fall back to simpler LCS
		if (max > 10000) {
			return this.simpleLCSDiff(a, b);
		}

		const v: Map<number, number> = new Map();
		v.set(1, 0);
		const trace: Map<number, number>[] = [];

		outer: for (let d = 0; d <= max; d++) {
			const vSnap = new Map(v);
			trace.push(vSnap);

			for (let k = -d; k <= d; k += 2) {
				let x: number;
				if (k === -d || (k !== d && (v.get(k - 1) ?? 0) < (v.get(k + 1) ?? 0))) {
					x = v.get(k + 1) ?? 0;
				} else {
					x = (v.get(k - 1) ?? 0) + 1;
				}
				let y = x - k;

				while (x < n && y < m && a[x] === b[y]) {
					x++;
					y++;
				}

				v.set(k, x);

				if (x >= n && y >= m) {
					break outer;
				}
			}
		}

		// Backtrack to get the edit script
		const result: { type: "equal" | "delete" | "insert"; value: string }[] = [];
		let x = n;
		let y = m;

		for (let d = trace.length - 1; d > 0; d--) {
			const vPrev = trace[d - 1];
			const k = x - y;

			let prevK: number;
			if (k === -d || (k !== d && (vPrev.get(k - 1) ?? 0) < (vPrev.get(k + 1) ?? 0))) {
				prevK = k + 1;
			} else {
				prevK = k - 1;
			}

			const prevX = vPrev.get(prevK) ?? 0;
			const prevY = prevX - prevK;

			// Diagonal (equal)
			while (x > prevX && y > prevY) {
				x--;
				y--;
				result.unshift({ type: "equal", value: a[x] });
			}

			if (d > 0) {
				if (x === prevX) {
					// Insert
					y--;
					result.unshift({ type: "insert", value: b[y] });
				} else {
					// Delete
					x--;
					result.unshift({ type: "delete", value: a[x] });
				}
			}
		}

		// Handle remaining diagonal at d=0
		while (x > 0 && y > 0) {
			x--;
			y--;
			result.unshift({ type: "equal", value: a[x] });
		}

		return result;
	}

	/**
	 * Fallback for very large files: simple LCS-based diff.
	 */
	private simpleLCSDiff(
		a: string[],
		b: string[]
	): { type: "equal" | "delete" | "insert"; value: string }[] {
		const result: { type: "equal" | "delete" | "insert"; value: string }[] = [];
		let ai = 0;
		let bi = 0;

		// Build hash map of b lines for quick lookup
		const bMap = new Map<string, number[]>();
		for (let i = 0; i < b.length; i++) {
			const arr = bMap.get(b[i]) || [];
			arr.push(i);
			bMap.set(b[i], arr);
		}

		while (ai < a.length && bi < b.length) {
			if (a[ai] === b[bi]) {
				result.push({ type: "equal", value: a[ai] });
				ai++;
				bi++;
			} else {
				// Look ahead to find the best match
				const bIndices = bMap.get(a[ai]);
				const nextBInA = b[bi] !== undefined ? a.indexOf(b[bi], ai) : -1;

				if (bIndices && bIndices.some((idx) => idx >= bi && idx - bi < 5)) {
					// Current a[ai] appears soon in b - delete from a
					result.push({ type: "delete", value: a[ai] });
					ai++;
				} else if (nextBInA >= 0 && nextBInA - ai < 5) {
					// Current b[bi] appears soon in a - insert from b
					result.push({ type: "insert", value: b[bi] });
					bi++;
				} else {
					result.push({ type: "delete", value: a[ai] });
					ai++;
				}
			}
		}

		while (ai < a.length) {
			result.push({ type: "delete", value: a[ai++] });
		}
		while (bi < b.length) {
			result.push({ type: "insert", value: b[bi++] });
		}

		return result;
	}

	/**
	 * Character-level diff between two lines for inline highlighting.
	 */
	private charDiff(
		oldLine: string,
		newLine: string
	): { old: CharDiff[]; new: CharDiff[] } {
		const oldChars = oldLine.split("");
		const newChars = newLine.split("");

		// Simple LCS for characters
		const lcs = this.lcs(oldChars, newChars);

		const oldResult: CharDiff[] = [];
		const newResult: CharDiff[] = [];

		let oi = 0;
		let ni = 0;
		let li = 0;

		while (oi < oldChars.length || ni < newChars.length) {
			if (
				li < lcs.length &&
				oi < oldChars.length &&
				ni < newChars.length &&
				oldChars[oi] === lcs[li] &&
				newChars[ni] === lcs[li]
			) {
				oldResult.push({ type: "unchanged", value: oldChars[oi] });
				newResult.push({ type: "unchanged", value: newChars[ni] });
				oi++;
				ni++;
				li++;
			} else {
				if (
					oi < oldChars.length &&
					(li >= lcs.length || oldChars[oi] !== lcs[li])
				) {
					oldResult.push({ type: "remove", value: oldChars[oi] });
					oi++;
				}
				if (
					ni < newChars.length &&
					(li >= lcs.length || newChars[ni] !== lcs[li])
				) {
					newResult.push({ type: "add", value: newChars[ni] });
					ni++;
				}
			}
		}

		return { old: this.mergeCharDiffs(oldResult), new: this.mergeCharDiffs(newResult) };
	}

	/**
	 * Merge consecutive CharDiff entries of the same type.
	 */
	private mergeCharDiffs(diffs: CharDiff[]): CharDiff[] {
		if (diffs.length === 0) return [];
		const merged: CharDiff[] = [{ ...diffs[0] }];
		for (let i = 1; i < diffs.length; i++) {
			const last = merged[merged.length - 1];
			if (last.type === diffs[i].type) {
				last.value += diffs[i].value;
			} else {
				merged.push({ ...diffs[i] });
			}
		}
		return merged;
	}

	/**
	 * Classic LCS for character arrays (bounded for performance).
	 */
	private lcs(a: string[], b: string[]): string[] {
		const n = a.length;
		const m = b.length;

		// For very long lines, use a simpler approach
		if (n * m > 100000) {
			return this.greedyLCS(a, b);
		}

		const dp: number[][] = Array.from({ length: n + 1 }, () =>
			new Array(m + 1).fill(0)
		);

		for (let i = 1; i <= n; i++) {
			for (let j = 1; j <= m; j++) {
				if (a[i - 1] === b[j - 1]) {
					dp[i][j] = dp[i - 1][j - 1] + 1;
				} else {
					dp[i][j] = Math.max(dp[i - 1][j], dp[i][j - 1]);
				}
			}
		}

		// Backtrack
		const result: string[] = [];
		let i = n;
		let j = m;
		while (i > 0 && j > 0) {
			if (a[i - 1] === b[j - 1]) {
				result.unshift(a[i - 1]);
				i--;
				j--;
			} else if (dp[i - 1][j] > dp[i][j - 1]) {
				i--;
			} else {
				j--;
			}
		}

		return result;
	}

	/**
	 * Greedy LCS for very long lines.
	 */
	private greedyLCS(a: string[], b: string[]): string[] {
		const result: string[] = [];
		let bi = 0;
		for (let ai = 0; ai < a.length && bi < b.length; ai++) {
			const idx = b.indexOf(a[ai], bi);
			if (idx !== -1) {
				result.push(a[ai]);
				bi = idx + 1;
			}
		}
		return result;
	}
}
