import os
from dataclasses import asdict
from typing import Optional

from PySide6.QtCore import Qt
from PySide6.QtWidgets import (
    QApplication,
    QFileDialog,
    QHBoxLayout,
    QMainWindow,
    QMenuBar,
    QMessageBox,
    QSplitter,
    QStatusBar,
    QToolBar,
    QVBoxLayout,
    QWidget,
)

from core.format_engine import FormatEngine
from core.models import TextEntry
from core.project import Project

from .editor_panel import EditorPanel
from .entry_table import EntryTable
from .export_dialog import ExportDialog
from .format_editor import FormatEditorDialog
from .history_panel import HistoryPanel
from .import_dialog import ImportDialog
from .project_tree import ProjectTree
from .search_bar import SearchBar


class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("FloorFlour - Game Text Editor")
        self.setMinimumSize(1100, 700)

        self.project: Optional[Project] = None
        self.format_engine: Optional[FormatEngine] = None
        self._current_source_filter: Optional[str] = None

        self._build_ui()
        self._build_menus()
        self._build_toolbar()
        self._connect_signals()
        self._update_title()

    # ── UI Construction ──

    def _build_ui(self):
        self.project_tree = ProjectTree()
        self.entry_table = EntryTable()
        self.editor_panel = EditorPanel()
        self.history_panel = HistoryPanel()
        self.search_bar = SearchBar()

        # Right side: search + table + editor
        right_top = QWidget()
        right_top_layout = QVBoxLayout(right_top)
        right_top_layout.setContentsMargins(0, 0, 0, 0)
        right_top_layout.addWidget(self.search_bar)
        right_top_layout.addWidget(self.entry_table, 2)
        right_top_layout.addWidget(self.editor_panel, 1)

        # Horizontal split: tree | right
        h_splitter = QSplitter(Qt.Horizontal)
        h_splitter.addWidget(self.project_tree)
        h_splitter.addWidget(right_top)
        h_splitter.setSizes([200, 800])

        # Vertical split: top | history
        v_splitter = QSplitter(Qt.Vertical)
        v_splitter.addWidget(h_splitter)
        v_splitter.addWidget(self.history_panel)
        v_splitter.setSizes([500, 200])

        self.setCentralWidget(v_splitter)
        self.statusBar().showMessage("Ready")

    def _build_menus(self):
        menubar = self.menuBar()

        # File menu
        file_menu = menubar.addMenu("File")
        file_menu.addAction("New Project...", self._new_project)
        file_menu.addAction("Open Project...", self._open_project)
        file_menu.addSeparator()
        file_menu.addAction("Import File...", self._import_file)
        file_menu.addAction("Export File...", self._export_file)
        file_menu.addSeparator()
        file_menu.addAction("Quit", self.close)

        # Format menu
        fmt_menu = menubar.addMenu("Format")
        fmt_menu.addAction("New Format Definition...", self._new_format)
        fmt_menu.addAction("Edit Format Definition...", self._edit_format)

        # View menu
        view_menu = menubar.addMenu("View")
        view_menu.addAction("Refresh", self._refresh_all)

    def _build_toolbar(self):
        toolbar = QToolBar("Main")
        toolbar.setMovable(False)
        self.addToolBar(toolbar)

        toolbar.addAction("New Project", self._new_project)
        toolbar.addAction("Open Project", self._open_project)
        toolbar.addSeparator()
        toolbar.addAction("Import", self._import_file)
        toolbar.addAction("Export", self._export_file)

    def _connect_signals(self):
        self.project_tree.source_file_selected.connect(self._filter_by_source)
        self.project_tree.all_entries_selected.connect(self._show_all_entries)
        self.entry_table.entry_selected.connect(self._on_entry_selected)
        self.editor_panel.entry_changed.connect(self._on_entry_changed)
        self.history_panel.revert_requested.connect(self._on_revert_requested)
        self.search_bar.search_requested.connect(self._on_search)
        self.search_bar.filter_changed.connect(self._on_status_filter)

    # ── Project Operations ──

    def _new_project(self):
        directory = QFileDialog.getExistingDirectory(
            self, "Select Directory for New Project"
        )
        if not directory:
            return
        name, ok = self._input_dialog("Project Name", "Enter project name:")
        if not ok or not name:
            return
        try:
            if self.project:
                self.project.close()
            self.project = Project.create(directory, name)
            self._init_format_engine()
            self._refresh_all()
            self.statusBar().showMessage(f"Created project: {name}")
        except Exception as e:
            QMessageBox.critical(self, "Error", str(e))

    def _open_project(self):
        directory = QFileDialog.getExistingDirectory(
            self, "Open Project Directory"
        )
        if not directory:
            return
        try:
            if self.project:
                self.project.close()
            self.project = Project.open(directory)
            self._init_format_engine()
            self._refresh_all()
            self.statusBar().showMessage(f"Opened project: {self.project.name}")
        except Exception as e:
            QMessageBox.critical(self, "Error", str(e))

    def _init_format_engine(self):
        if not self.project or not self.project.db:
            return
        self.format_engine = FormatEngine(self.project.db)
        self.format_engine.load_builtin_formats()
        if self.project.formats_dir:
            self.format_engine.load_formats_from_dir(str(self.project.formats_dir))

    # ── Import / Export ──

    def _import_file(self):
        if not self._require_project():
            return
        dlg = ImportDialog(self.format_engine, self)
        if dlg.exec() == ImportDialog.Accepted:
            result = dlg.get_result()
            if result:
                try:
                    entries = self.format_engine.import_file(
                        result["file_path"],
                        result["format_name"],
                        result["replace_existing"],
                    )
                    self._refresh_all()
                    self.statusBar().showMessage(
                        f"Imported {len(entries)} entries from {os.path.basename(result['file_path'])}"
                    )
                except Exception as e:
                    QMessageBox.critical(self, "Import Error", str(e))

    def _export_file(self):
        if not self._require_project():
            return
        source_files = self.project.db.get_source_file_names()
        dlg = ExportDialog(self.format_engine, source_files, self)
        if dlg.exec() == ExportDialog.Accepted:
            result = dlg.get_result()
            if result:
                try:
                    self.format_engine.export_file(
                        result["source_file"],
                        result["format_name"],
                        result["output_path"],
                    )
                    self.statusBar().showMessage(
                        f"Exported to {result['output_path']}"
                    )
                except Exception as e:
                    QMessageBox.critical(self, "Export Error", str(e))

    # ── Format Operations ──

    def _new_format(self):
        if not self._require_project():
            return
        dlg = FormatEditorDialog(parent=self)
        if dlg.exec() == FormatEditorDialog.Accepted:
            config = dlg.get_config()
            if config:
                path = dlg.save_to_file(str(self.project.formats_dir))
                self.format_engine.add_format(config)
                self._refresh_all()
                self.statusBar().showMessage(f"Created format: {config.get('name', '')}")

    def _edit_format(self):
        if not self._require_project():
            return
        formats = self.format_engine.get_format_names()
        if not formats:
            QMessageBox.information(self, "Info", "No format definitions found.")
            return
        # Simple selection — use first format or let user type
        name, ok = self._input_dialog(
            "Edit Format",
            f"Enter format name to edit:\n({', '.join(formats)})",
        )
        if not ok or not name or name not in formats:
            return
        config = self.format_engine.get_format_config(name)
        dlg = FormatEditorDialog(existing_config=config, parent=self)
        if dlg.exec() == FormatEditorDialog.Accepted:
            new_config = dlg.get_config()
            if new_config:
                dlg.save_to_file(str(self.project.formats_dir))
                self.format_engine.add_format(new_config)
                self.statusBar().showMessage(f"Updated format: {new_config.get('name', '')}")

    # ── Entry Interaction ──

    def _on_entry_selected(self, entry_id: int):
        if not self.project or not self.project.db:
            return
        entry = self.project.db.get_entry(entry_id)
        if entry:
            self.editor_panel.load_entry(entry)
            records = self.project.history.get_entry_history(entry_id)
            self.history_panel.load_records(records)

    def _on_entry_changed(self, entry: TextEntry):
        if not self.project or not self.project.db:
            return
        old_entry = self.project.db.get_entry(entry.id)
        if old_entry is None:
            return
        self.project.history.track_update(old_entry, entry)
        self.project.db.update_entry(entry)
        self.entry_table.update_row_for_entry(entry)

        # Refresh history panel
        records = self.project.history.get_entry_history(entry.id)
        self.history_panel.load_records(records)
        self.statusBar().showMessage(f"Saved entry: {entry.key}")

    def _on_revert_requested(self, entry_id: int, history_id: int):
        if not self.project or not self.project.history:
            return
        entry = self.project.history.revert_entry(entry_id, history_id)
        if entry:
            self.editor_panel.load_entry(entry)
            self.entry_table.update_row_for_entry(entry)
            records = self.project.history.get_entry_history(entry_id)
            self.history_panel.load_records(records)
            self.statusBar().showMessage(f"Reverted entry: {entry.key}")

    # ── Search / Filter ──

    def _on_search(self, query: str):
        if not self.project or not self.project.db:
            return
        if query:
            entries = self.project.db.search_entries(query)
        else:
            entries = self._get_current_entries()
        self.entry_table.load_entries(entries)
        self.statusBar().showMessage(f"Found {len(entries)} entries")

    def _on_status_filter(self, status: str):
        if not self.project or not self.project.db:
            return
        if status == "All":
            entries = self._get_current_entries()
        else:
            entries = self.project.db.get_entries_by_status(status)
        self.entry_table.load_entries(entries)

    def _filter_by_source(self, source_file: str):
        if not self.project or not self.project.db:
            return
        self._current_source_filter = source_file
        entries = self.project.db.get_entries_by_source(source_file)
        self.entry_table.load_entries(entries)
        self.editor_panel.clear()
        self.statusBar().showMessage(f"Showing entries from: {source_file}")

    def _show_all_entries(self):
        self._current_source_filter = None
        if self.project and self.project.db:
            entries = self.project.db.get_all_entries()
            self.entry_table.load_entries(entries)
            self.editor_panel.clear()
            self.statusBar().showMessage(f"Showing all {len(entries)} entries")

    def _get_current_entries(self):
        if not self.project or not self.project.db:
            return []
        if self._current_source_filter:
            return self.project.db.get_entries_by_source(self._current_source_filter)
        return self.project.db.get_all_entries()

    # ── Refresh ──

    def _refresh_all(self):
        self._update_title()
        if self.project and self.project.is_open:
            self.project_tree.refresh(self.project)
            entries = self.project.db.get_all_entries()
            self.entry_table.load_entries(entries)
            self.editor_panel.clear()
            records = self.project.history.get_recent(limit=50)
            self.history_panel.load_records(records)

    def _update_title(self):
        if self.project and self.project.is_open:
            self.setWindowTitle(f"FloorFlour - {self.project.name}")
        else:
            self.setWindowTitle("FloorFlour - Game Text Editor")

    # ── Helpers ──

    def _require_project(self) -> bool:
        if not self.project or not self.project.is_open:
            QMessageBox.warning(
                self, "No Project", "Please create or open a project first."
            )
            return False
        return True

    def _input_dialog(self, title: str, label: str):
        from PySide6.QtWidgets import QInputDialog
        return QInputDialog.getText(self, title, label)

    def closeEvent(self, event):
        if self.project:
            self.project.close()
        event.accept()
