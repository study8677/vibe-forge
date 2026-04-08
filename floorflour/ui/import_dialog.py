from typing import List, Optional

from PySide6.QtWidgets import (
    QCheckBox,
    QComboBox,
    QDialog,
    QDialogButtonBox,
    QFileDialog,
    QFormLayout,
    QGroupBox,
    QHBoxLayout,
    QHeaderView,
    QLabel,
    QLineEdit,
    QPushButton,
    QTableWidget,
    QTableWidgetItem,
    QVBoxLayout,
)

from core.format_engine import FormatEngine
from formats.base import ExtractedEntry


class ImportDialog(QDialog):
    """Dialog for importing text entries from a game file."""

    def __init__(self, format_engine: FormatEngine, parent=None):
        super().__init__(parent)
        self.setWindowTitle("Import File")
        self.setMinimumSize(700, 500)
        self.format_engine = format_engine

        self.file_path = ""
        self.selected_format = ""
        self.replace_existing = False
        self._preview_entries: List[ExtractedEntry] = []

        # File selection
        self.file_edit = QLineEdit()
        self.file_edit.setReadOnly(True)
        browse_btn = QPushButton("Browse...")
        browse_btn.clicked.connect(self._browse_file)

        file_layout = QHBoxLayout()
        file_layout.addWidget(self.file_edit)
        file_layout.addWidget(browse_btn)

        # Format selection
        self.format_combo = QComboBox()
        self.format_combo.addItems(format_engine.get_format_names())
        self.format_combo.currentTextChanged.connect(self._on_format_changed)

        # Options
        self.replace_check = QCheckBox("Replace existing entries from same source")

        # Preview table
        self.preview_table = QTableWidget()
        self.preview_table.setColumnCount(3)
        self.preview_table.setHorizontalHeaderLabels(["Key", "Value", "Context"])
        header = self.preview_table.horizontalHeader()
        header.setSectionResizeMode(0, QHeaderView.ResizeToContents)
        header.setSectionResizeMode(1, QHeaderView.Stretch)
        header.setSectionResizeMode(2, QHeaderView.Stretch)

        self.preview_btn = QPushButton("Preview")
        self.preview_btn.clicked.connect(self._preview)

        self.status_label = QLabel("")

        # Buttons
        button_box = QDialogButtonBox(
            QDialogButtonBox.Ok | QDialogButtonBox.Cancel
        )
        button_box.accepted.connect(self._on_accept)
        button_box.rejected.connect(self.reject)

        # Layout
        form = QFormLayout()
        form.addRow("File:", file_layout)
        form.addRow("Format:", self.format_combo)
        form.addRow("", self.replace_check)

        preview_group = QGroupBox("Preview")
        preview_layout = QVBoxLayout(preview_group)
        preview_layout.addWidget(self.preview_btn)
        preview_layout.addWidget(self.preview_table)
        preview_layout.addWidget(self.status_label)

        main_layout = QVBoxLayout(self)
        main_layout.addLayout(form)
        main_layout.addWidget(preview_group)
        main_layout.addWidget(button_box)

    def _browse_file(self):
        path, _ = QFileDialog.getOpenFileName(
            self, "Select Game File", "", "All Files (*)"
        )
        if path:
            self.file_path = path
            self.file_edit.setText(path)
            # Auto-detect format
            detected = self.format_engine.auto_detect_format(path)
            if detected:
                idx = self.format_combo.findText(detected)
                if idx >= 0:
                    self.format_combo.setCurrentIndex(idx)

    def _on_format_changed(self, text: str):
        self.selected_format = text

    def _preview(self):
        if not self.file_path or not self.format_combo.currentText():
            self.status_label.setText("Please select a file and format.")
            return
        try:
            self._preview_entries = self.format_engine.preview_import(
                self.file_path, self.format_combo.currentText()
            )
            self.preview_table.setRowCount(len(self._preview_entries))
            for row, entry in enumerate(self._preview_entries):
                self.preview_table.setItem(row, 0, QTableWidgetItem(entry.key))
                self.preview_table.setItem(row, 1, QTableWidgetItem(entry.value))
                self.preview_table.setItem(row, 2, QTableWidgetItem(entry.context))
            self.status_label.setText(f"Found {len(self._preview_entries)} entries.")
        except Exception as e:
            self.status_label.setText(f"Error: {e}")
            self._preview_entries = []
            self.preview_table.setRowCount(0)

    def _on_accept(self):
        self.selected_format = self.format_combo.currentText()
        self.replace_existing = self.replace_check.isChecked()
        if self.file_path and self.selected_format:
            self.accept()

    def get_result(self) -> Optional[dict]:
        if self.result() == QDialog.Accepted:
            return {
                "file_path": self.file_path,
                "format_name": self.selected_format,
                "replace_existing": self.replace_existing,
            }
        return None
