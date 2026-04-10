from __future__ import annotations

from abc import ABC, abstractmethod
from dataclasses import dataclass, field
from typing import Any, AsyncIterator


@dataclass
class ChatResponse:
    content: str = ""
    tool_calls: list[ToolCall] = field(default_factory=list)
    tokens_input: int = 0
    tokens_output: int = 0
    raw: Any = None


@dataclass
class ToolCall:
    id: str = ""
    name: str = ""
    arguments: dict[str, Any] = field(default_factory=dict)


@dataclass
class StreamChunk:
    content: str = ""
    is_first: bool = False
    is_done: bool = False
    tokens_input: int = 0
    tokens_output: int = 0


class LLMProvider(ABC):
    name: str
    model: str

    @abstractmethod
    async def chat(
        self,
        messages: list[dict[str, Any]],
        *,
        timeout: int = 30,
        **kwargs: Any,
    ) -> ChatResponse: ...

    @abstractmethod
    async def chat_stream(
        self,
        messages: list[dict[str, Any]],
        *,
        timeout: int = 60,
        **kwargs: Any,
    ) -> AsyncIterator[StreamChunk]: ...

    @abstractmethod
    async def chat_with_tools(
        self,
        messages: list[dict[str, Any]],
        tools: list[dict[str, Any]],
        *,
        timeout: int = 60,
        **kwargs: Any,
    ) -> ChatResponse: ...

    async def close(self) -> None:
        pass
