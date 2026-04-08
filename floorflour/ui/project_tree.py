from PySide6.QtCore import Signal
from PySide6.QtWidgets import (
    QTreeWidget,
    QTreeWidgetItem,
    QVBoxLayout,
    QWidget,
)

from core.project import Project


class ProjectTree(QWidget):
    """Left panel showing source files and format definitions as a tree."""

    source_file_selected = Signal(str)   # emits source file name
    format_selected = Signal(str)        # emits format file path
    all_entries_selected = Signal()       # user clicked "All Entries"

    def __init__(self, parent=None):
        super().__init__(parent)
        self.tree = QTreeWidget()
        self.tree.setHeaderLabel("Project")
        self.tree.itemClicked.connect(self._on_item_clicked)

        layout = QVBoxLayout(self)
        layout.setContentsMargins(0, 0, 0, 0)
        layout.addWidget(self.tree)

        self._all_item: QTreeWidgetItem | None = None
        self._sources_root: QTreeWidgetItem | None = None
        self._formats_root: QTreeWidgetItem | None = None

    def refresh(self, project: Project):
        self.tree.clear()

        # "All Entries" node
        self._all_item = QTreeWidgetItem(self.tree, ["All Entries"])
        self._all_item.setData(0, 0x100, "__all__")

        # Source files
        self._sources_root = QTreeWidgetItem(self.tree, ["Source Files"])
        self._sources_root.setExpanded(True)
        if project.db:
            for name in project.db.get_source_file_names():
                item = QTreeWidgetItem(self._sources_root, [name])
                item.setData(0, 0x100, f"source:{name}")

        # Format definitions
        self._formats_root = QTreeWidgetItem(self.tree, ["Format Definitions"])
        self._formats_root.setExpanded(True)
        for fp in project.get_format_files():
            item = QTreeWidgetItem(self._formats_root, [fp.stem])
            item.setData(0, 0x100, f"format:{fp}")

    def _on_item_clicked(self, item: QTreeWidgetItem, column: int):
        data = item.data(0, 0x100)
        if data is None:
            return
        if data == "__all__":
            self.all_entries_selected.emit()
        elif isinstance(data, str) and data.startswith("source:"):
            self.source_file_selected.emit(data[7:])
        elif isinstance(data, str) and data.startswith("format:"):
            self.format_selected.emit(data[7:])
