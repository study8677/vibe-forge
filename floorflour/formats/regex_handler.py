import re
from typing import Dict, List, Optional

from .base import ExtractedEntry, FormatHandler


class RegexHandler(FormatHandler):
    """Extracts and injects text using regex patterns. Suitable for source code files."""

    def __init__(self, config: dict):
        super().__init__(config)
        rc = config.get("regex", {})
        self.patterns: List[dict] = rc.get("patterns", [])
        self.replacement: str = rc.get("replacement", "")

    def extract(self, file_path: str) -> List[ExtractedEntry]:
        with open(file_path, "r", encoding=self.encoding) as f:
            content = f.read()

        results = []
        for pat_cfg in self.patterns:
            pattern = pat_cfg.get("pattern", "")
            key_group: int = pat_cfg.get("key_group", 0)
            value_group: int = pat_cfg.get("value_group", 1)
            key_mode: str = pat_cfg.get("key_mode", "capture_group")
            context_lines: int = pat_cfg.get("context_lines", 0)

            lines = content.split("\n")
            for match in re.finditer(pattern, content):
                if key_mode == "line_number":
                    line_no = content[:match.start()].count("\n") + 1
                    key = f"L{line_no}"
                elif key_mode == "auto":
                    line_no = content[:match.start()].count("\n") + 1
                    try:
                        key = match.group(key_group) if key_group > 0 else f"L{line_no}"
                    except IndexError:
                        key = f"L{line_no}"
                else:
                    try:
                        key = match.group(key_group)
                    except IndexError:
                        line_no = content[:match.start()].count("\n") + 1
                        key = f"L{line_no}"

                try:
                    value = match.group(value_group)
                except IndexError:
                    value = match.group(0)

                entry = ExtractedEntry(
                    key=key,
                    value=value,
                    line_number=content[:match.start()].count("\n") + 1,
                )

                if context_lines > 0:
                    ln = entry.line_number - 1
                    start = max(0, ln - context_lines)
                    end = min(len(lines), ln + context_lines + 1)
                    entry.context = "\n".join(lines[start:end])

                results.append(entry)

        return results

    def inject(self, file_path: str, entries: Dict[str, str], output_path: Optional[str] = None):
        with open(file_path, "r", encoding=self.encoding) as f:
            content = f.read()

        for pat_cfg in self.patterns:
            pattern = pat_cfg.get("pattern", "")
            key_group: int = pat_cfg.get("key_group", 0)
            value_group: int = pat_cfg.get("value_group", 1)
            key_mode: str = pat_cfg.get("key_mode", "capture_group")
            replacement_template = pat_cfg.get("replacement", "") or self.replacement

            if not replacement_template:
                # Fall back to in-place group substitution
                content = self._inject_by_group_replace(
                    content, pattern, key_group, value_group, key_mode, entries
                )
            else:
                content = self._inject_by_template(
                    content, pattern, key_group, key_mode, replacement_template, entries
                )

        dest = output_path or file_path
        with open(dest, "w", encoding=self.encoding) as f:
            f.write(content)

    def _inject_by_template(
        self,
        content: str,
        pattern: str,
        key_group: int,
        key_mode: str,
        template: str,
        entries: Dict[str, str],
    ) -> str:
        def replacer(match: re.Match) -> str:
            if key_mode == "line_number":
                line_no = content[:match.start()].count("\n") + 1
                key = f"L{line_no}"
            else:
                try:
                    key = match.group(key_group)
                except IndexError:
                    return match.group(0)

            if key not in entries:
                return match.group(0)

            return template.format(key=key, value=entries[key])

        return re.sub(pattern, replacer, content)

    def _inject_by_group_replace(
        self,
        content: str,
        pattern: str,
        key_group: int,
        value_group: int,
        key_mode: str,
        entries: Dict[str, str],
    ) -> str:
        """Replace just the value group within each match, preserving surrounding text."""
        offset = 0
        result = list(content)

        for match in re.finditer(pattern, content):
            if key_mode == "line_number":
                line_no = content[:match.start()].count("\n") + 1
                key = f"L{line_no}"
            else:
                try:
                    key = match.group(key_group)
                except IndexError:
                    continue

            if key not in entries:
                continue

            try:
                start = match.start(value_group) + offset
                end = match.end(value_group) + offset
            except IndexError:
                continue

            new_val = entries[key]
            result[start:end] = list(new_val)
            offset += len(new_val) - (match.end(value_group) - match.start(value_group))

        return "".join(result)
