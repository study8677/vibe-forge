from typing import List, Optional

from PySide6.QtCore import Qt, Signal
from PySide6.QtGui import QColor
from PySide6.QtWidgets import (
    QAbstractItemView,
    QHeaderView,
    QTableWidget,
    QTableWidgetItem,
    QVBoxLayout,
    QWidget,
)

from core.models import TextEntry

# Column indices
COL_ID = 0
COL_KEY = 1
COL_ORIGINAL = 2
COL_CURRENT = 3
COL_STATUS = 4
COL_SOURCE = 5

HEADERS = ["ID", "Key", "Original", "Current", "Status", "Source"]

STATUS_COLORS = {
    "unmodified": QColor(220, 220, 220),
    "modified": QColor(255, 255, 180),
    "reviewed": QColor(180, 255, 180),
}


class EntryTable(QWidget):
    """Table view displaying text entries."""

    entry_selected = Signal(int)  # emits entry id

    def __init__(self, parent=None):
        super().__init__(parent)
        self.table = QTableWidget()
        self.table.setColumnCount(len(HEADERS))
        self.table.setHorizontalHeaderLabels(HEADERS)
        self.table.setSelectionBehavior(QAbstractItemView.SelectRows)
        self.table.setSelectionMode(QAbstractItemView.SingleSelection)
        self.table.setEditTriggers(QAbstractItemView.NoEditTriggers)
        self.table.verticalHeader().setVisible(False)
        self.table.currentCellChanged.connect(self._on_selection_changed)

        header = self.table.horizontalHeader()
        header.setSectionResizeMode(COL_KEY, QHeaderView.ResizeToContents)
        header.setSectionResizeMode(COL_ORIGINAL, QHeaderView.Stretch)
        header.setSectionResizeMode(COL_CURRENT, QHeaderView.Stretch)
        header.setSectionResizeMode(COL_STATUS, QHeaderView.ResizeToContents)
        header.setSectionResizeMode(COL_SOURCE, QHeaderView.ResizeToContents)
        self.table.setColumnHidden(COL_ID, True)

        layout = QVBoxLayout(self)
        layout.setContentsMargins(0, 0, 0, 0)
        layout.addWidget(self.table)

        self._entries: List[TextEntry] = []

    def load_entries(self, entries: List[TextEntry]):
        self._entries = entries
        self.table.setRowCount(len(entries))
        for row, entry in enumerate(entries):
            self._set_row(row, entry)

    def _set_row(self, row: int, entry: TextEntry):
        items = [
            str(entry.id or ""),
            entry.key,
            self._truncate(entry.original_text),
            self._truncate(entry.current_text),
            entry.status,
            entry.source_file,
        ]
        bg = STATUS_COLORS.get(entry.status, QColor(255, 255, 255))
        for col, text in enumerate(items):
            item = QTableWidgetItem(text)
            item.setBackground(bg)
            self.table.setItem(row, col, item)

    def update_row_for_entry(self, entry: TextEntry):
        for row in range(self.table.rowCount()):
            id_item = self.table.item(row, COL_ID)
            if id_item and id_item.text() == str(entry.id):
                self._set_row(row, entry)
                self._entries[row] = entry
                break

    def get_selected_entry_id(self) -> Optional[int]:
        row = self.table.currentRow()
        if row < 0:
            return None
        id_item = self.table.item(row, COL_ID)
        if id_item:
            try:
                return int(id_item.text())
            except ValueError:
                pass
        return None

    def _on_selection_changed(self, row: int, col: int, prev_row: int, prev_col: int):
        if row < 0:
            return
        id_item = self.table.item(row, COL_ID)
        if id_item:
            try:
                self.entry_selected.emit(int(id_item.text()))
            except ValueError:
                pass

    @staticmethod
    def _truncate(text: str, max_len: int = 80) -> str:
        text = text.replace("\n", " ").replace("\r", "")
        if len(text) > max_len:
            return text[:max_len] + "..."
        return text
