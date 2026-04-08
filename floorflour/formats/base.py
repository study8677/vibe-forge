from abc import ABC, abstractmethod
from dataclasses import dataclass
from typing import Dict, List, Optional


@dataclass
class ExtractedEntry:
    """A single text entry extracted from a source file."""
    key: str
    value: str
    context: str = ""
    speaker: str = ""
    line_number: int = 0


class FormatHandler(ABC):
    """Base class for all format handlers."""

    def __init__(self, config: dict):
        self.config = config
        self.name: str = config.get("name", "")
        self.extensions: List[str] = config.get("extensions", [])
        self.encoding: str = config.get("encoding", "utf-8")

    @abstractmethod
    def extract(self, file_path: str) -> List[ExtractedEntry]:
        """Extract text entries from a file."""
        ...

    @abstractmethod
    def inject(self, file_path: str, entries: Dict[str, str], output_path: Optional[str] = None):
        """Write modified text entries back into a file.

        Args:
            file_path: Path to the original source file (used as template).
            entries: Mapping of key -> new text value.
            output_path: Where to write. If None, overwrite file_path.
        """
        ...

    def matches_extension(self, file_path: str) -> bool:
        for ext in self.extensions:
            if file_path.lower().endswith(ext.lower()):
                return True
        return False
