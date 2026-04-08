import os
from pathlib import Path
from typing import Dict, List, Optional

import yaml

from .database import Database
from .models import SourceFile, TextEntry
from formats.base import ExtractedEntry, FormatHandler
from formats.structured_handler import StructuredHandler
from formats.regex_handler import RegexHandler
from formats.binary_handler import BinaryHandler


HANDLER_MAP = {
    "structured": StructuredHandler,
    "regex": RegexHandler,
    "binary": BinaryHandler,
}


def load_format_definition(yaml_path: str) -> dict:
    with open(yaml_path, "r", encoding="utf-8") as f:
        return yaml.safe_load(f)


def create_handler(config: dict) -> FormatHandler:
    fmt_type = config.get("type", "structured")
    handler_cls = HANDLER_MAP.get(fmt_type)
    if handler_cls is None:
        raise ValueError(f"Unknown format type: {fmt_type}")
    return handler_cls(config)


class FormatEngine:
    """Manages format definitions and performs import/export operations."""

    def __init__(self, db: Database):
        self.db = db
        self._formats: Dict[str, dict] = {}

    def load_formats_from_dir(self, directory: str):
        """Load all .yaml format definitions from a directory."""
        dir_path = Path(directory)
        if not dir_path.exists():
            return
        for yaml_file in sorted(dir_path.glob("*.yaml")):
            try:
                config = load_format_definition(str(yaml_file))
                name = config.get("name", yaml_file.stem)
                self._formats[name] = config
            except Exception:
                pass

    def load_builtin_formats(self):
        """Load built-in format definitions shipped with the application."""
        builtin_dir = Path(__file__).parent.parent / "builtin_formats"
        self.load_formats_from_dir(str(builtin_dir))

    def get_format_names(self) -> List[str]:
        return list(self._formats.keys())

    def get_format_config(self, name: str) -> Optional[dict]:
        return self._formats.get(name)

    def add_format(self, config: dict):
        name = config.get("name", "")
        if name:
            self._formats[name] = config

    # ── Import ──

    def import_file(
        self,
        file_path: str,
        format_name: str,
        replace_existing: bool = False,
    ) -> List[TextEntry]:
        """Extract text from a file and store as TextEntry rows."""
        config = self._formats.get(format_name)
        if config is None:
            raise ValueError(f"Unknown format: {format_name}")

        handler = create_handler(config)
        extracted = handler.extract(file_path)

        rel_path = os.path.basename(file_path)

        if replace_existing:
            self.db.delete_entries_by_source(rel_path)

        # Register source file
        sf = SourceFile(path=rel_path, format_name=format_name)
        self.db.insert_source_file(sf)

        entries = []
        for ex in extracted:
            entry = TextEntry(
                key=ex.key,
                source_file=rel_path,
                original_text=ex.value,
                current_text=ex.value,
                status="unmodified",
                speaker=ex.speaker,
                context=ex.context,
            )
            entry.id = self.db.insert_entry(entry)
            entries.append(entry)

        return entries

    def preview_import(self, file_path: str, format_name: str) -> List[ExtractedEntry]:
        """Extract text from a file without storing, for preview purposes."""
        config = self._formats.get(format_name)
        if config is None:
            raise ValueError(f"Unknown format: {format_name}")
        handler = create_handler(config)
        return handler.extract(file_path)

    # ── Export ──

    def export_file(
        self,
        source_file_path: str,
        format_name: str,
        output_path: str,
        entries: Optional[List[TextEntry]] = None,
    ):
        """Write edited text back into a file using the format definition."""
        config = self._formats.get(format_name)
        if config is None:
            raise ValueError(f"Unknown format: {format_name}")

        if entries is None:
            rel_name = os.path.basename(source_file_path)
            entries = self.db.get_entries_by_source(rel_name)

        mapping = {e.key: e.current_text for e in entries}
        handler = create_handler(config)
        handler.inject(source_file_path, mapping, output_path)

    def auto_detect_format(self, file_path: str) -> Optional[str]:
        """Try to find a matching format based on file extension."""
        for name, config in self._formats.items():
            handler = create_handler(config)
            if handler.matches_extension(file_path):
                return name
        return None
