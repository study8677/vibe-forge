from __future__ import annotations

import os
import re
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

import yaml

_ENV_RE = re.compile(r"\$\{(\w+)\}")


def _resolve_env(value: str) -> str:
    """Replace ${VAR} placeholders with environment variable values."""
    def _sub(m: re.Match) -> str:
        return os.environ.get(m.group(1), "")
    return _ENV_RE.sub(_sub, value)


@dataclass
class TestConfig:
    enabled: bool = True
    timeout: int = 30


@dataclass
class ProviderConfig:
    name: str
    type: str  # "openai" or "anthropic"
    api_key: str = ""
    base_url: str = ""
    models: list[str] = field(default_factory=list)


@dataclass
class AppConfig:
    db_path: str = "./raccoon.db"
    schedule_interval: int = 30
    concurrency: int = 5
    providers: list[ProviderConfig] = field(default_factory=list)
    tests: dict[str, TestConfig] = field(default_factory=dict)


_DEFAULT_TESTS = {
    "basic_call": TestConfig(True, 30),
    "streaming": TestConfig(True, 60),
    "tool_use": TestConfig(True, 60),
    "agent_loop": TestConfig(True, 120),
    "json_output": TestConfig(True, 30),
}


def _find_config() -> Path | None:
    """Search for config.yaml in cwd and upward."""
    candidates = ["config.yaml", "config.yml", "raccoon.yaml", "raccoon.yml"]
    cwd = Path.cwd()
    for name in candidates:
        p = cwd / name
        if p.exists():
            return p
    return None


def load_config(path: str | None = None) -> AppConfig:
    if path:
        config_path = Path(path)
    else:
        config_path = _find_config()

    if config_path is None or not config_path.exists():
        return AppConfig(tests=dict(_DEFAULT_TESTS))

    with open(config_path) as f:
        raw: dict[str, Any] = yaml.safe_load(f) or {}

    db_path = raw.get("database", {}).get("path", "./raccoon.db")
    schedule_interval = raw.get("schedule", {}).get("interval", 30)
    concurrency = raw.get("concurrency", 5)

    providers: list[ProviderConfig] = []
    for name, pconf in raw.get("providers", {}).items():
        api_key = _resolve_env(pconf.get("api_key", ""))
        if not api_key:
            continue  # skip providers without API keys
        providers.append(ProviderConfig(
            name=name,
            type=pconf.get("type", "openai"),
            api_key=api_key,
            base_url=pconf.get("base_url", ""),
            models=pconf.get("models", []),
        ))

    tests: dict[str, TestConfig] = {}
    for tname, tdefault in _DEFAULT_TESTS.items():
        tconf = raw.get("tests", {}).get(tname, {})
        tests[tname] = TestConfig(
            enabled=tconf.get("enabled", tdefault.enabled),
            timeout=tconf.get("timeout", tdefault.timeout),
        )

    return AppConfig(
        db_path=db_path,
        schedule_interval=schedule_interval,
        concurrency=concurrency,
        providers=providers,
        tests=tests,
    )
