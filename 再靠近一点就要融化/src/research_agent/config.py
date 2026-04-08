from __future__ import annotations

import os
from dataclasses import dataclass


@dataclass(slots=True)
class OpenAIConfig:
    api_key: str
    model: str = "gpt-5.4-mini"
    api_base: str = "https://api.openai.com/v1"
    reasoning_effort: str = "medium"
    timeout_seconds: int = 120

    @classmethod
    def from_env(
        cls,
        *,
        model_override: str | None = None,
        timeout_override: int | None = None,
    ) -> "OpenAIConfig":
        api_key = os.environ.get("OPENAI_API_KEY", "").strip()
        if not api_key:
            raise ValueError("OPENAI_API_KEY is required for the OpenAI backend.")
        return cls(
            api_key=api_key,
            model=model_override or os.environ.get("OPENAI_MODEL", "gpt-5.4-mini"),
            api_base=os.environ.get("OPENAI_API_BASE", "https://api.openai.com/v1"),
            reasoning_effort=os.environ.get("OPENAI_REASONING_EFFORT", "medium"),
            timeout_seconds=timeout_override
            or int(os.environ.get("OPENAI_TIMEOUT_SECONDS", "120")),
        )
