from typing import List, Optional

from PySide6.QtCore import Signal
from PySide6.QtWidgets import (
    QAbstractItemView,
    QGroupBox,
    QHBoxLayout,
    QHeaderView,
    QPushButton,
    QTableWidget,
    QTableWidgetItem,
    QVBoxLayout,
    QWidget,
)

from core.models import HistoryRecord


class HistoryPanel(QWidget):
    """Displays modification history for the selected entry or globally."""

    revert_requested = Signal(int, int)  # entry_id, history_id

    def __init__(self, parent=None):
        super().__init__(parent)

        self.table = QTableWidget()
        headers = ["Time", "Entry ID", "Field", "Old Value", "New Value", "Description"]
        self.table.setColumnCount(len(headers))
        self.table.setHorizontalHeaderLabels(headers)
        self.table.setSelectionBehavior(QAbstractItemView.SelectRows)
        self.table.setSelectionMode(QAbstractItemView.SingleSelection)
        self.table.setEditTriggers(QAbstractItemView.NoEditTriggers)
        self.table.verticalHeader().setVisible(False)

        header = self.table.horizontalHeader()
        header.setSectionResizeMode(0, QHeaderView.ResizeToContents)
        header.setSectionResizeMode(1, QHeaderView.ResizeToContents)
        header.setSectionResizeMode(2, QHeaderView.ResizeToContents)
        header.setSectionResizeMode(3, QHeaderView.Stretch)
        header.setSectionResizeMode(4, QHeaderView.Stretch)
        header.setSectionResizeMode(5, QHeaderView.ResizeToContents)

        self.revert_btn = QPushButton("Revert Selected")
        self.revert_btn.clicked.connect(self._on_revert)
        self.revert_btn.setEnabled(False)

        self.table.currentCellChanged.connect(
            lambda r, c, pr, pc: self.revert_btn.setEnabled(r >= 0)
        )

        btn_layout = QHBoxLayout()
        btn_layout.addStretch()
        btn_layout.addWidget(self.revert_btn)

        group = QGroupBox("History")
        group_layout = QVBoxLayout(group)
        group_layout.addWidget(self.table)
        group_layout.addLayout(btn_layout)

        layout = QVBoxLayout(self)
        layout.setContentsMargins(0, 0, 0, 0)
        layout.addWidget(group)

        self._records: List[HistoryRecord] = []

    def load_records(self, records: List[HistoryRecord]):
        self._records = records
        self.table.setRowCount(len(records))
        for row, rec in enumerate(records):
            self.table.setItem(row, 0, QTableWidgetItem(rec.timestamp or ""))
            self.table.setItem(row, 1, QTableWidgetItem(str(rec.entry_id)))
            self.table.setItem(row, 2, QTableWidgetItem(rec.field_name))
            self.table.setItem(row, 3, QTableWidgetItem(self._truncate(rec.old_value)))
            self.table.setItem(row, 4, QTableWidgetItem(self._truncate(rec.new_value)))
            self.table.setItem(row, 5, QTableWidgetItem(rec.description))
        self.revert_btn.setEnabled(False)

    def _on_revert(self):
        row = self.table.currentRow()
        if row < 0 or row >= len(self._records):
            return
        rec = self._records[row]
        if rec.id is not None:
            self.revert_requested.emit(rec.entry_id, rec.id)

    @staticmethod
    def _truncate(text: str, max_len: int = 60) -> str:
        text = text.replace("\n", " ").replace("\r", "")
        if len(text) > max_len:
            return text[:max_len] + "..."
        return text
