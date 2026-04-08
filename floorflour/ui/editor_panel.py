from PySide6.QtCore import Signal
from PySide6.QtWidgets import (
    QFormLayout,
    QGroupBox,
    QHBoxLayout,
    QLabel,
    QLineEdit,
    QPlainTextEdit,
    QPushButton,
    QComboBox,
    QVBoxLayout,
    QWidget,
)

from core.models import TextEntry


STATUS_OPTIONS = ["unmodified", "modified", "reviewed"]


class EditorPanel(QWidget):
    """Panel for editing a single TextEntry."""

    entry_changed = Signal(TextEntry)  # emitted when user modifies and confirms

    def __init__(self, parent=None):
        super().__init__(parent)
        self._current_entry: TextEntry | None = None

        # Fields
        self.key_edit = QLineEdit()
        self.key_edit.setReadOnly(True)

        self.original_display = QPlainTextEdit()
        self.original_display.setReadOnly(True)
        self.original_display.setMaximumHeight(100)

        self.current_edit = QPlainTextEdit()
        self.current_edit.setMaximumHeight(100)

        self.speaker_edit = QLineEdit()
        self.notes_edit = QPlainTextEdit()
        self.notes_edit.setMaximumHeight(60)

        self.status_combo = QComboBox()
        self.status_combo.addItems(STATUS_OPTIONS)

        self.context_display = QPlainTextEdit()
        self.context_display.setReadOnly(True)
        self.context_display.setMaximumHeight(60)

        # Buttons
        self.save_btn = QPushButton("Save")
        self.save_btn.clicked.connect(self._on_save)
        self.revert_btn = QPushButton("Revert to Original")
        self.revert_btn.clicked.connect(self._on_revert)

        btn_layout = QHBoxLayout()
        btn_layout.addWidget(self.save_btn)
        btn_layout.addWidget(self.revert_btn)
        btn_layout.addStretch()

        # Layout
        form = QFormLayout()
        form.addRow("Key:", self.key_edit)
        form.addRow("Original:", self.original_display)
        form.addRow("Current:", self.current_edit)
        form.addRow("Speaker:", self.speaker_edit)
        form.addRow("Status:", self.status_combo)
        form.addRow("Notes:", self.notes_edit)
        form.addRow("Context:", self.context_display)

        group = QGroupBox("Edit Entry")
        group_layout = QVBoxLayout(group)
        group_layout.addLayout(form)
        group_layout.addLayout(btn_layout)

        main_layout = QVBoxLayout(self)
        main_layout.setContentsMargins(0, 0, 0, 0)
        main_layout.addWidget(group)

        self.set_enabled(False)

    def set_enabled(self, enabled: bool):
        self.current_edit.setEnabled(enabled)
        self.speaker_edit.setEnabled(enabled)
        self.notes_edit.setEnabled(enabled)
        self.status_combo.setEnabled(enabled)
        self.save_btn.setEnabled(enabled)
        self.revert_btn.setEnabled(enabled)

    def load_entry(self, entry: TextEntry):
        self._current_entry = entry
        self.key_edit.setText(entry.key)
        self.original_display.setPlainText(entry.original_text)
        self.current_edit.setPlainText(entry.current_text)
        self.speaker_edit.setText(entry.speaker)
        self.notes_edit.setPlainText(entry.notes)
        self.context_display.setPlainText(entry.context)

        idx = STATUS_OPTIONS.index(entry.status) if entry.status in STATUS_OPTIONS else 0
        self.status_combo.setCurrentIndex(idx)

        self.set_enabled(True)

    def clear(self):
        self._current_entry = None
        self.key_edit.clear()
        self.original_display.clear()
        self.current_edit.clear()
        self.speaker_edit.clear()
        self.notes_edit.clear()
        self.context_display.clear()
        self.status_combo.setCurrentIndex(0)
        self.set_enabled(False)

    def _on_save(self):
        if self._current_entry is None:
            return
        entry = self._current_entry
        new_text = self.current_edit.toPlainText()
        new_speaker = self.speaker_edit.text()
        new_notes = self.notes_edit.toPlainText()
        new_status = self.status_combo.currentText()

        # Auto-set status to modified if text changed
        if new_text != entry.original_text and new_status == "unmodified":
            new_status = "modified"

        entry.current_text = new_text
        entry.speaker = new_speaker
        entry.notes = new_notes
        entry.status = new_status

        self.entry_changed.emit(entry)

    def _on_revert(self):
        if self._current_entry is None:
            return
        self.current_edit.setPlainText(self._current_entry.original_text)
        self.status_combo.setCurrentIndex(STATUS_OPTIONS.index("unmodified"))
