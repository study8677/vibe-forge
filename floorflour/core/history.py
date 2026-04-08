from typing import List, Optional

from .database import Database
from .models import TextEntry, HistoryRecord


class HistoryTracker:
    """Tracks changes to TextEntry fields and records them in the history table."""

    TRACKED_FIELDS = ("current_text", "status", "speaker", "context", "notes", "key")

    def __init__(self, db: Database):
        self.db = db

    def track_update(
        self,
        old_entry: TextEntry,
        new_entry: TextEntry,
        description: str = "",
    ) -> List[HistoryRecord]:
        """Compare two versions of an entry and record all changed fields."""
        records = []
        for field_name in self.TRACKED_FIELDS:
            old_val = str(getattr(old_entry, field_name, "") or "")
            new_val = str(getattr(new_entry, field_name, "") or "")
            if old_val != new_val:
                record = HistoryRecord(
                    entry_id=old_entry.id,
                    field_name=field_name,
                    old_value=old_val,
                    new_value=new_val,
                    description=description,
                )
                self.db.insert_history(record)
                records.append(record)
        return records

    def get_entry_history(self, entry_id: int) -> List[HistoryRecord]:
        return self.db.get_history_for_entry(entry_id)

    def get_recent(self, limit: int = 200) -> List[HistoryRecord]:
        return self.db.get_recent_history(limit)

    def revert_entry(self, entry_id: int, history_id: int) -> Optional[TextEntry]:
        """Revert a single field change identified by history_id."""
        entry = self.db.get_entry(entry_id)
        if entry is None:
            return None

        history = self.db.get_history_for_entry(entry_id)
        target = None
        for h in history:
            if h.id == history_id:
                target = h
                break
        if target is None:
            return None

        old_entry = TextEntry(**{k: getattr(entry, k) for k in entry.__dataclass_fields__})
        setattr(entry, target.field_name, target.old_value)

        if entry.current_text == entry.original_text:
            entry.status = "unmodified"

        self.db.update_entry(entry)
        self.track_update(old_entry, entry, description=f"revert history #{history_id}")
        return entry
