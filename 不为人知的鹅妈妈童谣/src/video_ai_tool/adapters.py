from __future__ import annotations


class _StrictFormatDict(dict[str, str]):
    def __missing__(self, key: str) -> str:
        raise KeyError(f"Unknown command placeholder: {key}")


def render_stage_command(template: str, placeholders: dict[str, str]) -> str:
    if not template.strip():
        raise ValueError("Stage command template is empty")
    return template.format_map(_StrictFormatDict(placeholders))
