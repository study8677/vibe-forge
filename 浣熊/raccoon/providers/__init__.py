from __future__ import annotations

from raccoon.config import ProviderConfig

from .anthropic_ import AnthropicProvider
from .base import ChatResponse, LLMProvider, StreamChunk, ToolCall
from .openai_compat import OpenAICompatProvider

__all__ = ["ChatResponse", "LLMProvider", "StreamChunk", "ToolCall", "create_provider"]

_FACTORIES = {
    "openai": OpenAICompatProvider,
    "anthropic": AnthropicProvider,
}


def create_provider(config: ProviderConfig, model: str) -> LLMProvider:
    cls = _FACTORIES.get(config.type)
    if cls is None:
        raise ValueError(f"Unknown provider type: {config.type}")
    return cls(
        name=config.name,
        model=model,
        api_key=config.api_key,
        base_url=config.base_url,
    )
