"""Configuration loader — reads ``config/gateway.yaml`` and expands env vars."""

from __future__ import annotations

import os
import re
from pathlib import Path
from typing import Any

import yaml
from pydantic import BaseModel, Field

from .models import Protocol

_ENV_RE = re.compile(r"\$\{(\w+)(?::([^}]*))?\}")


def _expand_env(value: str) -> str:
    """Replace ``${VAR}`` / ``${VAR:default}`` with environment variable."""
    def _sub(m: re.Match) -> str:
        name, default = m.group(1), m.group(2)
        return os.environ.get(name, default if default is not None else m.group(0))
    return _ENV_RE.sub(_sub, value)


def _walk_expand(obj: Any) -> Any:
    if isinstance(obj, str):
        return _expand_env(obj)
    if isinstance(obj, dict):
        return {k: _walk_expand(v) for k, v in obj.items()}
    if isinstance(obj, list):
        return [_walk_expand(i) for i in obj]
    return obj


# ── Config models ────────────────────────────────────────────────────────────

class GatewaySettings(BaseModel):
    name: str = "travelport-unified-gateway"
    version: str = "1.0.0"
    default_timeout: int = 30
    log_level: str = "INFO"


class AccountCredentials(BaseModel):
    # SOAP
    target_branch: str | None = None
    username: str | None = None
    password: str | None = None
    # JSON / OAuth2
    client_id: str | None = None
    client_secret: str | None = None
    token_endpoint: str | None = None


class AccountConfig(BaseModel):
    id: str
    name: str
    protocol: Protocol
    pcc: str
    provider: str = "1G"
    credentials: AccountCredentials
    endpoints: dict[str, str] = Field(default_factory=dict)


class RoutingCondition(BaseModel):
    field: str
    operator: str = "eq"   # eq | neq | in | not_in
    value: Any = None


class RoutingRule(BaseModel):
    operation: str
    account_id: str
    conditions: list[RoutingCondition] = Field(default_factory=list)


class RoutingConfig(BaseModel):
    default_account: str
    rules: list[RoutingRule] = Field(default_factory=list)


class GatewayConfig(BaseModel):
    gateway: GatewaySettings = Field(default_factory=GatewaySettings)
    schema_version: str = "v52_0"
    accounts: list[AccountConfig] = Field(default_factory=list)
    routing: RoutingConfig = Field(default_factory=lambda: RoutingConfig(default_account=""))


def load_config(path: str | Path | None = None) -> GatewayConfig:
    """Load and validate gateway config from YAML, expanding env-var placeholders."""
    if path is None:
        path = Path(__file__).resolve().parents[2] / "config" / "gateway.yaml"
    path = Path(path)
    raw = yaml.safe_load(path.read_text(encoding="utf-8"))
    expanded = _walk_expand(raw)
    return GatewayConfig.model_validate(expanded)
