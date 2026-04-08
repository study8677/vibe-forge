from PySide6.QtCore import Signal
from PySide6.QtWidgets import (
    QComboBox,
    QHBoxLayout,
    QLineEdit,
    QPushButton,
    QWidget,
)


class SearchBar(QWidget):
    """Search and filter bar for text entries."""

    search_requested = Signal(str)
    filter_changed = Signal(str)  # status filter

    def __init__(self, parent=None):
        super().__init__(parent)

        self.search_edit = QLineEdit()
        self.search_edit.setPlaceholderText("Search key, text, notes...")
        self.search_edit.returnPressed.connect(self._on_search)

        self.search_btn = QPushButton("Search")
        self.search_btn.clicked.connect(self._on_search)

        self.clear_btn = QPushButton("Clear")
        self.clear_btn.clicked.connect(self._on_clear)

        self.filter_combo = QComboBox()
        self.filter_combo.addItems(["All", "unmodified", "modified", "reviewed"])
        self.filter_combo.currentTextChanged.connect(self._on_filter)

        layout = QHBoxLayout(self)
        layout.setContentsMargins(0, 0, 0, 0)
        layout.addWidget(self.search_edit, 1)
        layout.addWidget(self.search_btn)
        layout.addWidget(self.clear_btn)
        layout.addWidget(self.filter_combo)

    def _on_search(self):
        self.search_requested.emit(self.search_edit.text())

    def _on_clear(self):
        self.search_edit.clear()
        self.filter_combo.setCurrentIndex(0)
        self.search_requested.emit("")

    def _on_filter(self, text: str):
        self.filter_changed.emit(text)
