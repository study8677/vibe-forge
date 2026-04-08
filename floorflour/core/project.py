import os
import shutil
from datetime import datetime
from pathlib import Path
from typing import Optional

import yaml

from .database import Database
from .history import HistoryTracker


class Project:
    """Manages a FloorFlour project directory."""

    CONFIG_FILE = "project.yaml"
    DB_FILE = "texts.db"
    FORMATS_DIR = "formats"

    def __init__(self):
        self.path: Optional[Path] = None
        self.name: str = ""
        self.description: str = ""
        self.created_at: str = ""
        self.db: Optional[Database] = None
        self.history: Optional[HistoryTracker] = None

    @property
    def is_open(self) -> bool:
        return self.path is not None and self.db is not None

    @property
    def formats_dir(self) -> Optional[Path]:
        return self.path / self.FORMATS_DIR if self.path else None

    # ── Lifecycle ──

    @classmethod
    def create(cls, directory: str, name: str, description: str = "") -> "Project":
        """Create a new project in the given directory."""
        proj_dir = Path(directory)
        proj_dir.mkdir(parents=True, exist_ok=True)

        formats_dir = proj_dir / cls.FORMATS_DIR
        formats_dir.mkdir(exist_ok=True)

        now = datetime.now().isoformat()
        config = {
            "name": name,
            "description": description,
            "created_at": now,
        }
        config_path = proj_dir / cls.CONFIG_FILE
        with open(config_path, "w", encoding="utf-8") as f:
            yaml.dump(config, f, allow_unicode=True, default_flow_style=False)

        project = cls()
        project.path = proj_dir
        project.name = name
        project.description = description
        project.created_at = now

        db_path = str(proj_dir / cls.DB_FILE)
        project.db = Database(db_path)
        project.db.connect()
        project.history = HistoryTracker(project.db)

        return project

    @classmethod
    def open(cls, directory: str) -> "Project":
        """Open an existing project from a directory."""
        proj_dir = Path(directory)
        config_path = proj_dir / cls.CONFIG_FILE
        if not config_path.exists():
            raise FileNotFoundError(
                f"Not a FloorFlour project: {config_path} not found"
            )

        with open(config_path, "r", encoding="utf-8") as f:
            config = yaml.safe_load(f)

        project = cls()
        project.path = proj_dir
        project.name = config.get("name", "")
        project.description = config.get("description", "")
        project.created_at = config.get("created_at", "")

        formats_dir = proj_dir / cls.FORMATS_DIR
        formats_dir.mkdir(exist_ok=True)

        db_path = str(proj_dir / cls.DB_FILE)
        project.db = Database(db_path)
        project.db.connect()
        project.history = HistoryTracker(project.db)

        return project

    def save_config(self):
        """Write current project metadata back to project.yaml."""
        if not self.path:
            return
        config = {
            "name": self.name,
            "description": self.description,
            "created_at": self.created_at,
        }
        config_path = self.path / self.CONFIG_FILE
        with open(config_path, "w", encoding="utf-8") as f:
            yaml.dump(config, f, allow_unicode=True, default_flow_style=False)

    def close(self):
        """Close the project and release database connection."""
        if self.db:
            self.db.close()
            self.db = None
        self.history = None
        self.path = None

    # ── Format definitions ──

    def get_format_files(self) -> list[Path]:
        """List all .yaml format definition files in the project."""
        if not self.formats_dir or not self.formats_dir.exists():
            return []
        return sorted(self.formats_dir.glob("*.yaml"))

    def add_format_file(self, source_path: str) -> Path:
        """Copy a format definition file into the project's formats directory."""
        src = Path(source_path)
        dest = self.formats_dir / src.name
        shutil.copy2(src, dest)
        return dest
