from typing import List, Optional

from PySide6.QtWidgets import (
    QComboBox,
    QDialog,
    QDialogButtonBox,
    QFileDialog,
    QFormLayout,
    QHBoxLayout,
    QLabel,
    QLineEdit,
    QPushButton,
    QVBoxLayout,
)

from core.format_engine import FormatEngine


class ExportDialog(QDialog):
    """Dialog for exporting edited text entries back to a game file."""

    def __init__(
        self,
        format_engine: FormatEngine,
        source_files: List[str],
        parent=None,
    ):
        super().__init__(parent)
        self.setWindowTitle("Export File")
        self.setMinimumWidth(500)
        self.format_engine = format_engine

        # Source file (template)
        self.source_combo = QComboBox()
        self.source_combo.setEditable(True)
        self.source_combo.addItems(source_files)

        self.source_browse_btn = QPushButton("Browse...")
        self.source_browse_btn.clicked.connect(self._browse_source)

        source_layout = QHBoxLayout()
        source_layout.addWidget(self.source_combo, 1)
        source_layout.addWidget(self.source_browse_btn)

        # Format
        self.format_combo = QComboBox()
        self.format_combo.addItems(format_engine.get_format_names())

        # Output path
        self.output_edit = QLineEdit()
        output_browse_btn = QPushButton("Browse...")
        output_browse_btn.clicked.connect(self._browse_output)

        output_layout = QHBoxLayout()
        output_layout.addWidget(self.output_edit, 1)
        output_layout.addWidget(output_browse_btn)

        self.status_label = QLabel("")

        # Buttons
        button_box = QDialogButtonBox(
            QDialogButtonBox.Ok | QDialogButtonBox.Cancel
        )
        button_box.accepted.connect(self._on_accept)
        button_box.rejected.connect(self.reject)

        # Layout
        form = QFormLayout()
        form.addRow("Source file (template):", source_layout)
        form.addRow("Format:", self.format_combo)
        form.addRow("Output path:", output_layout)

        main_layout = QVBoxLayout(self)
        main_layout.addLayout(form)
        main_layout.addWidget(self.status_label)
        main_layout.addWidget(button_box)

    def _browse_source(self):
        path, _ = QFileDialog.getOpenFileName(
            self, "Select Source File", "", "All Files (*)"
        )
        if path:
            self.source_combo.setCurrentText(path)

    def _browse_output(self):
        path, _ = QFileDialog.getSaveFileName(
            self, "Save As", "", "All Files (*)"
        )
        if path:
            self.output_edit.setText(path)

    def _on_accept(self):
        if not self.source_combo.currentText():
            self.status_label.setText("Please select a source file.")
            return
        if not self.output_edit.text():
            self.status_label.setText("Please specify an output path.")
            return
        if not self.format_combo.currentText():
            self.status_label.setText("Please select a format.")
            return
        self.accept()

    def get_result(self) -> Optional[dict]:
        if self.result() == QDialog.Accepted:
            return {
                "source_file": self.source_combo.currentText(),
                "format_name": self.format_combo.currentText(),
                "output_path": self.output_edit.text(),
            }
        return None
