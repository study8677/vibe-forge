from dataclasses import dataclass, field
from datetime import datetime
from typing import Optional


@dataclass
class TextEntry:
    id: Optional[int] = None
    key: str = ""
    source_file: str = ""
    original_text: str = ""
    current_text: str = ""
    status: str = "unmodified"
    speaker: str = ""
    context: str = ""
    notes: str = ""
    created_at: Optional[str] = None
    updated_at: Optional[str] = None


@dataclass
class HistoryRecord:
    id: Optional[int] = None
    entry_id: int = 0
    field_name: str = ""
    old_value: str = ""
    new_value: str = ""
    timestamp: Optional[str] = None
    description: str = ""


@dataclass
class SourceFile:
    id: Optional[int] = None
    path: str = ""
    format_name: str = ""
    last_imported: Optional[str] = None
