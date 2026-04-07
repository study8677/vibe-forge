from pathlib import Path
import pkgutil

__path__ = pkgutil.extend_path(__path__, __name__)  # type: ignore[name-defined]

SRC_PACKAGE = Path(__file__).resolve().parent.parent / "src" / "video_ai_tool"
if SRC_PACKAGE.exists():
    __path__.append(str(SRC_PACKAGE))  # type: ignore[attr-defined]
