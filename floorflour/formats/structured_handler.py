import csv
import io
import json
import os
from collections import OrderedDict
from typing import Dict, List, Optional
from xml.etree import ElementTree as ET

import yaml

from .base import ExtractedEntry, FormatHandler


class StructuredHandler(FormatHandler):
    """Handles JSON, XML, CSV, and YAML files."""

    def __init__(self, config: dict):
        super().__init__(config)
        sc = config.get("structured", {})
        self.format: str = sc.get("format", "json")
        self.entries_path: str = sc.get("entries_path", "$")
        self.entry_mode: str = sc.get("entry_mode", "key_value")
        self.key_field: str = sc.get("key_field", "id")
        self.value_field: str = sc.get("value_field", "text")
        self.context_field: str = sc.get("context_field", "")
        self.speaker_field: str = sc.get("speaker_field", "")
        self.csv_delimiter: str = sc.get("csv_delimiter", ",")
        self.csv_key_col: int = sc.get("csv_key_col", 0)
        self.csv_value_col: int = sc.get("csv_value_col", 1)

    def extract(self, file_path: str) -> List[ExtractedEntry]:
        with open(file_path, "r", encoding=self.encoding) as f:
            content = f.read()

        dispatch = {
            "json": self._extract_json,
            "yaml": self._extract_yaml,
            "xml": self._extract_xml,
            "csv": self._extract_csv,
        }
        handler = dispatch.get(self.format)
        if handler is None:
            raise ValueError(f"Unknown structured format: {self.format}")
        return handler(content)

    def inject(self, file_path: str, entries: Dict[str, str], output_path: Optional[str] = None):
        with open(file_path, "r", encoding=self.encoding) as f:
            content = f.read()

        dispatch = {
            "json": self._inject_json,
            "yaml": self._inject_yaml,
            "xml": self._inject_xml,
            "csv": self._inject_csv,
        }
        handler = dispatch.get(self.format)
        if handler is None:
            raise ValueError(f"Unknown structured format: {self.format}")

        result = handler(content, entries)
        dest = output_path or file_path
        with open(dest, "w", encoding=self.encoding) as f:
            f.write(result)

    # ── JSON ──

    def _extract_json(self, content: str) -> List[ExtractedEntry]:
        data = json.loads(content)
        node = self._resolve_path_json(data, self.entries_path)
        return self._extract_from_node(node)

    def _inject_json(self, content: str, entries: Dict[str, str]) -> str:
        data = json.loads(content)
        node = self._resolve_path_json(data, self.entries_path)
        self._inject_into_node(node, entries)
        return json.dumps(data, ensure_ascii=False, indent=2) + "\n"

    # ── YAML ──

    def _extract_yaml(self, content: str) -> List[ExtractedEntry]:
        data = yaml.safe_load(content)
        node = self._resolve_path_json(data, self.entries_path)
        return self._extract_from_node(node)

    def _inject_yaml(self, content: str, entries: Dict[str, str]) -> str:
        data = yaml.safe_load(content)
        node = self._resolve_path_json(data, self.entries_path)
        self._inject_into_node(node, entries)
        return yaml.dump(data, allow_unicode=True, default_flow_style=False)

    # ── XML ──

    def _extract_xml(self, content: str) -> List[ExtractedEntry]:
        root = ET.fromstring(content)
        results = []
        xpath = self.entries_path.replace("$.", "").replace("$", ".")
        if xpath == ".":
            elements = list(root)
        else:
            elements = root.findall(xpath)

        for elem in elements:
            if self.entry_mode == "key_value":
                key = elem.get("name") or elem.get("id") or elem.tag
                value = elem.text or ""
                results.append(ExtractedEntry(key=key, value=value))
            elif self.entry_mode == "array_of_objects":
                key_elem = elem.find(self.key_field)
                val_elem = elem.find(self.value_field)
                if key_elem is not None and val_elem is not None:
                    entry = ExtractedEntry(
                        key=key_elem.text or "",
                        value=val_elem.text or "",
                    )
                    if self.context_field:
                        ctx_elem = elem.find(self.context_field)
                        if ctx_elem is not None:
                            entry.context = ctx_elem.text or ""
                    results.append(entry)
        return results

    def _inject_xml(self, content: str, entries: Dict[str, str]) -> str:
        root = ET.fromstring(content)
        xpath = self.entries_path.replace("$.", "").replace("$", ".")
        if xpath == ".":
            elements = list(root)
        else:
            elements = root.findall(xpath)

        for elem in elements:
            if self.entry_mode == "key_value":
                key = elem.get("name") or elem.get("id") or elem.tag
                if key in entries:
                    elem.text = entries[key]
            elif self.entry_mode == "array_of_objects":
                key_elem = elem.find(self.key_field)
                val_elem = elem.find(self.value_field)
                if key_elem is not None and val_elem is not None:
                    if key_elem.text in entries:
                        val_elem.text = entries[key_elem.text]

        return ET.tostring(root, encoding="unicode", xml_declaration=True) + "\n"

    # ── CSV ──

    def _extract_csv(self, content: str) -> List[ExtractedEntry]:
        reader = csv.reader(io.StringIO(content), delimiter=self.csv_delimiter)
        results = []
        for i, row in enumerate(reader):
            if len(row) <= max(self.csv_key_col, self.csv_value_col):
                continue
            if i == 0 and not any(c.isdigit() for c in row[self.csv_key_col]):
                # likely header row — skip
                continue
            results.append(ExtractedEntry(
                key=row[self.csv_key_col],
                value=row[self.csv_value_col],
                line_number=i + 1,
            ))
        return results

    def _inject_csv(self, content: str, entries: Dict[str, str]) -> str:
        reader = csv.reader(io.StringIO(content), delimiter=self.csv_delimiter)
        output = io.StringIO()
        writer = csv.writer(output, delimiter=self.csv_delimiter)
        for row in reader:
            if len(row) > max(self.csv_key_col, self.csv_value_col):
                key = row[self.csv_key_col]
                if key in entries:
                    row[self.csv_value_col] = entries[key]
            writer.writerow(row)
        return output.getvalue()

    # ── Helpers ──

    def _resolve_path_json(self, data, path: str):
        """Resolve a simple dot-path like '$.texts' or '$' on a dict/list."""
        if path in ("$", "", "."):
            return data
        parts = path.lstrip("$.").split(".")
        node = data
        for p in parts:
            if isinstance(node, dict):
                node = node[p]
            elif isinstance(node, list) and p.isdigit():
                node = node[int(p)]
            else:
                raise KeyError(f"Cannot resolve path segment '{p}' on {type(node)}")
        return node

    def _extract_from_node(self, node) -> List[ExtractedEntry]:
        results = []
        if self.entry_mode == "key_value" and isinstance(node, dict):
            for k, v in node.items():
                if isinstance(v, str):
                    results.append(ExtractedEntry(key=k, value=v))
                elif isinstance(v, dict) and self.value_field in v:
                    entry = ExtractedEntry(key=k, value=str(v[self.value_field]))
                    if self.context_field and self.context_field in v:
                        entry.context = str(v[self.context_field])
                    if self.speaker_field and self.speaker_field in v:
                        entry.speaker = str(v[self.speaker_field])
                    results.append(entry)
        elif self.entry_mode == "array_of_objects" and isinstance(node, list):
            for item in node:
                if not isinstance(item, dict):
                    continue
                key = str(item.get(self.key_field, ""))
                value = str(item.get(self.value_field, ""))
                if key:
                    entry = ExtractedEntry(key=key, value=value)
                    if self.context_field and self.context_field in item:
                        entry.context = str(item[self.context_field])
                    if self.speaker_field and self.speaker_field in item:
                        entry.speaker = str(item[self.speaker_field])
                    results.append(entry)
        return results

    def _inject_into_node(self, node, entries: Dict[str, str]):
        if self.entry_mode == "key_value" and isinstance(node, dict):
            for k in node:
                if k in entries:
                    if isinstance(node[k], str):
                        node[k] = entries[k]
                    elif isinstance(node[k], dict) and self.value_field in node[k]:
                        node[k][self.value_field] = entries[k]
        elif self.entry_mode == "array_of_objects" and isinstance(node, list):
            for item in node:
                if not isinstance(item, dict):
                    continue
                key = str(item.get(self.key_field, ""))
                if key in entries:
                    item[self.value_field] = entries[key]
