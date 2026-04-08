from pathlib import Path
from typing import Optional

import yaml
from PySide6.QtWidgets import (
    QComboBox,
    QDialog,
    QDialogButtonBox,
    QFormLayout,
    QLabel,
    QLineEdit,
    QPlainTextEdit,
    QVBoxLayout,
)


FORMAT_TEMPLATES = {
    "structured": {
        "name": "",
        "extensions": [".json"],
        "encoding": "utf-8",
        "type": "structured",
        "structured": {
            "format": "json",
            "entries_path": "$",
            "entry_mode": "key_value",
        },
    },
    "regex": {
        "name": "",
        "extensions": [".lua"],
        "encoding": "utf-8",
        "type": "regex",
        "regex": {
            "patterns": [
                {
                    "pattern": r'"([^"]*)"',
                    "key_mode": "line_number",
                    "value_group": 1,
                }
            ],
            "replacement": "",
        },
    },
    "binary": {
        "name": "",
        "extensions": [".bin"],
        "type": "binary",
        "binary": {
            "endian": "little",
            "string_encoding": "utf-8",
            "null_terminated": True,
            "pointer_table": {
                "offset": 0,
                "count": 0,
                "entry_size": 4,
            },
        },
    },
}


class FormatEditorDialog(QDialog):
    """Dialog for creating or editing a format definition."""

    def __init__(self, existing_config: Optional[dict] = None, parent=None):
        super().__init__(parent)
        self.setWindowTitle("Format Definition Editor")
        self.setMinimumSize(600, 500)

        self._config = existing_config

        self.name_edit = QLineEdit()
        self.type_combo = QComboBox()
        self.type_combo.addItems(["structured", "regex", "binary"])
        self.type_combo.currentTextChanged.connect(self._on_type_changed)

        self.yaml_edit = QPlainTextEdit()
        self.yaml_edit.setFont(self.yaml_edit.document().defaultFont())

        self.status_label = QLabel("")

        button_box = QDialogButtonBox(
            QDialogButtonBox.Ok | QDialogButtonBox.Cancel
        )
        button_box.accepted.connect(self._on_accept)
        button_box.rejected.connect(self.reject)

        form = QFormLayout()
        form.addRow("Name:", self.name_edit)
        form.addRow("Type:", self.type_combo)

        layout = QVBoxLayout(self)
        layout.addLayout(form)
        layout.addWidget(QLabel("YAML Definition:"))
        layout.addWidget(self.yaml_edit, 1)
        layout.addWidget(self.status_label)
        layout.addWidget(button_box)

        if existing_config:
            self.name_edit.setText(existing_config.get("name", ""))
            fmt_type = existing_config.get("type", "structured")
            idx = self.type_combo.findText(fmt_type)
            if idx >= 0:
                self.type_combo.setCurrentIndex(idx)
            self.yaml_edit.setPlainText(
                yaml.dump(existing_config, allow_unicode=True, default_flow_style=False)
            )
        else:
            self._on_type_changed(self.type_combo.currentText())

    def _on_type_changed(self, fmt_type: str):
        if self._config:
            return  # don't overwrite when editing
        template = FORMAT_TEMPLATES.get(fmt_type, FORMAT_TEMPLATES["structured"])
        template = dict(template)
        template["name"] = self.name_edit.text() or "New Format"
        self.yaml_edit.setPlainText(
            yaml.dump(template, allow_unicode=True, default_flow_style=False)
        )

    def _on_accept(self):
        try:
            config = yaml.safe_load(self.yaml_edit.toPlainText())
            if not isinstance(config, dict):
                self.status_label.setText("Error: YAML must be a mapping.")
                return
            name = self.name_edit.text().strip()
            if not name:
                self.status_label.setText("Error: Name is required.")
                return
            config["name"] = name
            self._config = config
            self.accept()
        except yaml.YAMLError as e:
            self.status_label.setText(f"YAML Error: {e}")

    def get_config(self) -> Optional[dict]:
        return self._config if self.result() == QDialog.Accepted else None

    def save_to_file(self, directory: str) -> Optional[Path]:
        """Save the format definition to a YAML file in the given directory."""
        config = self.get_config()
        if config is None:
            return None
        name = config.get("name", "format").replace(" ", "_").lower()
        path = Path(directory) / f"{name}.yaml"
        with open(path, "w", encoding="utf-8") as f:
            yaml.dump(config, f, allow_unicode=True, default_flow_style=False)
        return path
