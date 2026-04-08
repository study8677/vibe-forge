import sqlite3
from datetime import datetime
from typing import List, Optional

from .models import TextEntry, HistoryRecord, SourceFile


class Database:
    def __init__(self, db_path: str):
        self.db_path = db_path
        self.conn: Optional[sqlite3.Connection] = None

    def connect(self):
        self.conn = sqlite3.connect(self.db_path)
        self.conn.row_factory = sqlite3.Row
        self.conn.execute("PRAGMA foreign_keys = ON")
        self._create_tables()

    def close(self):
        if self.conn:
            self.conn.close()
            self.conn = None

    def _create_tables(self):
        self.conn.executescript("""
            CREATE TABLE IF NOT EXISTS text_entries (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                key TEXT NOT NULL,
                source_file TEXT DEFAULT '',
                original_text TEXT DEFAULT '',
                current_text TEXT DEFAULT '',
                status TEXT DEFAULT 'unmodified',
                speaker TEXT DEFAULT '',
                context TEXT DEFAULT '',
                notes TEXT DEFAULT '',
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                entry_id INTEGER NOT NULL,
                field_name TEXT NOT NULL,
                old_value TEXT DEFAULT '',
                new_value TEXT DEFAULT '',
                timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                description TEXT DEFAULT '',
                FOREIGN KEY (entry_id) REFERENCES text_entries(id) ON DELETE CASCADE
            );

            CREATE TABLE IF NOT EXISTS source_files (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                path TEXT NOT NULL UNIQUE,
                format_name TEXT DEFAULT '',
                last_imported TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );

            CREATE INDEX IF NOT EXISTS idx_history_entry ON history(entry_id);
            CREATE INDEX IF NOT EXISTS idx_history_timestamp ON history(timestamp);
            CREATE INDEX IF NOT EXISTS idx_entries_source ON text_entries(source_file);
            CREATE INDEX IF NOT EXISTS idx_entries_status ON text_entries(status);
            CREATE INDEX IF NOT EXISTS idx_entries_key ON text_entries(key);
        """)
        self.conn.commit()

    # ── TextEntry CRUD ──

    def insert_entry(self, entry: TextEntry) -> int:
        now = datetime.now().isoformat()
        cursor = self.conn.execute(
            """INSERT INTO text_entries
               (key, source_file, original_text, current_text,
                status, speaker, context, notes, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)""",
            (entry.key, entry.source_file, entry.original_text,
             entry.current_text, entry.status, entry.speaker,
             entry.context, entry.notes, now, now),
        )
        self.conn.commit()
        return cursor.lastrowid

    def insert_entries_batch(self, entries: List[TextEntry]) -> List[int]:
        now = datetime.now().isoformat()
        ids = []
        for e in entries:
            cursor = self.conn.execute(
                """INSERT INTO text_entries
                   (key, source_file, original_text, current_text,
                    status, speaker, context, notes, created_at, updated_at)
                   VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)""",
                (e.key, e.source_file, e.original_text,
                 e.current_text, e.status, e.speaker,
                 e.context, e.notes, now, now),
            )
            ids.append(cursor.lastrowid)
        self.conn.commit()
        return ids

    def update_entry(self, entry: TextEntry):
        now = datetime.now().isoformat()
        self.conn.execute(
            """UPDATE text_entries
               SET key=?, source_file=?, original_text=?, current_text=?,
                   status=?, speaker=?, context=?, notes=?, updated_at=?
               WHERE id=?""",
            (entry.key, entry.source_file, entry.original_text,
             entry.current_text, entry.status, entry.speaker,
             entry.context, entry.notes, now, entry.id),
        )
        self.conn.commit()

    def delete_entry(self, entry_id: int):
        self.conn.execute("DELETE FROM text_entries WHERE id=?", (entry_id,))
        self.conn.commit()

    def delete_entries_by_source(self, source_file: str):
        self.conn.execute(
            "DELETE FROM text_entries WHERE source_file=?", (source_file,)
        )
        self.conn.commit()

    def get_entry(self, entry_id: int) -> Optional[TextEntry]:
        row = self.conn.execute(
            "SELECT * FROM text_entries WHERE id=?", (entry_id,)
        ).fetchone()
        return self._row_to_entry(row) if row else None

    def get_all_entries(self) -> List[TextEntry]:
        rows = self.conn.execute(
            "SELECT * FROM text_entries ORDER BY id"
        ).fetchall()
        return [self._row_to_entry(r) for r in rows]

    def get_entries_by_source(self, source_file: str) -> List[TextEntry]:
        rows = self.conn.execute(
            "SELECT * FROM text_entries WHERE source_file=? ORDER BY id",
            (source_file,),
        ).fetchall()
        return [self._row_to_entry(r) for r in rows]

    def get_entries_by_status(self, status: str) -> List[TextEntry]:
        rows = self.conn.execute(
            "SELECT * FROM text_entries WHERE status=? ORDER BY id",
            (status,),
        ).fetchall()
        return [self._row_to_entry(r) for r in rows]

    def search_entries(self, query: str) -> List[TextEntry]:
        pattern = f"%{query}%"
        rows = self.conn.execute(
            """SELECT * FROM text_entries
               WHERE key LIKE ? OR original_text LIKE ?
                  OR current_text LIKE ? OR notes LIKE ?
               ORDER BY id""",
            (pattern, pattern, pattern, pattern),
        ).fetchall()
        return [self._row_to_entry(r) for r in rows]

    def get_source_file_names(self) -> List[str]:
        rows = self.conn.execute(
            "SELECT DISTINCT source_file FROM text_entries ORDER BY source_file"
        ).fetchall()
        return [r["source_file"] for r in rows]

    def _row_to_entry(self, row: sqlite3.Row) -> TextEntry:
        return TextEntry(
            id=row["id"],
            key=row["key"],
            source_file=row["source_file"],
            original_text=row["original_text"],
            current_text=row["current_text"],
            status=row["status"],
            speaker=row["speaker"],
            context=row["context"],
            notes=row["notes"],
            created_at=row["created_at"],
            updated_at=row["updated_at"],
        )

    # ── History CRUD ──

    def insert_history(self, record: HistoryRecord) -> int:
        now = datetime.now().isoformat()
        cursor = self.conn.execute(
            """INSERT INTO history
               (entry_id, field_name, old_value, new_value, timestamp, description)
               VALUES (?, ?, ?, ?, ?, ?)""",
            (record.entry_id, record.field_name, record.old_value,
             record.new_value, now, record.description),
        )
        self.conn.commit()
        return cursor.lastrowid

    def get_history_for_entry(self, entry_id: int) -> List[HistoryRecord]:
        rows = self.conn.execute(
            "SELECT * FROM history WHERE entry_id=? ORDER BY timestamp DESC",
            (entry_id,),
        ).fetchall()
        return [self._row_to_history(r) for r in rows]

    def get_recent_history(self, limit: int = 200) -> List[HistoryRecord]:
        rows = self.conn.execute(
            "SELECT * FROM history ORDER BY timestamp DESC LIMIT ?",
            (limit,),
        ).fetchall()
        return [self._row_to_history(r) for r in rows]

    def _row_to_history(self, row: sqlite3.Row) -> HistoryRecord:
        return HistoryRecord(
            id=row["id"],
            entry_id=row["entry_id"],
            field_name=row["field_name"],
            old_value=row["old_value"],
            new_value=row["new_value"],
            timestamp=row["timestamp"],
            description=row["description"],
        )

    # ── SourceFile CRUD ──

    def insert_source_file(self, sf: SourceFile) -> int:
        now = datetime.now().isoformat()
        cursor = self.conn.execute(
            "INSERT OR REPLACE INTO source_files (path, format_name, last_imported) VALUES (?, ?, ?)",
            (sf.path, sf.format_name, now),
        )
        self.conn.commit()
        return cursor.lastrowid

    def get_source_files(self) -> List[SourceFile]:
        rows = self.conn.execute(
            "SELECT * FROM source_files ORDER BY path"
        ).fetchall()
        return [
            SourceFile(
                id=r["id"], path=r["path"],
                format_name=r["format_name"],
                last_imported=r["last_imported"],
            )
            for r in rows
        ]

    def get_source_file_by_path(self, path: str) -> Optional[SourceFile]:
        row = self.conn.execute(
            "SELECT * FROM source_files WHERE path=?", (path,)
        ).fetchone()
        if row:
            return SourceFile(
                id=row["id"], path=row["path"],
                format_name=row["format_name"],
                last_imported=row["last_imported"],
            )
        return None

    def delete_source_file(self, sf_id: int):
        self.conn.execute("DELETE FROM source_files WHERE id=?", (sf_id,))
        self.conn.commit()
