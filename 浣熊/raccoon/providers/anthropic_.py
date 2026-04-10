from __future__ import annotations

import json
from typing import Any, AsyncIterator

from anthropic import AsyncAnthropic

from .base import ChatResponse, LLMProvider, StreamChunk, ToolCall


class AnthropicProvider(LLMProvider):
    """Provider for Anthropic Claude models."""

    def __init__(self, name: str, model: str, api_key: str, base_url: str = ""):
        self.name = name
        self.model = model
        kwargs: dict[str, Any] = {"api_key": api_key}
        if base_url:
            kwargs["base_url"] = base_url
        self.client = AsyncAnthropic(**kwargs)

    def _extract_system(self, messages: list[dict[str, Any]]) -> tuple[str, list[dict[str, Any]]]:
        """Separate system message from the list (Anthropic uses a separate param)."""
        system = ""
        filtered = []
        for m in messages:
            if m.get("role") == "system":
                system = m.get("content", "")
            else:
                filtered.append(m)
        return system, filtered

    async def chat(
        self,
        messages: list[dict[str, Any]],
        *,
        timeout: int = 30,
        **kwargs: Any,
    ) -> ChatResponse:
        system, msgs = self._extract_system(messages)
        call_kwargs: dict[str, Any] = {
            "model": self.model,
            "messages": msgs,
            "max_tokens": kwargs.pop("max_tokens", 4096),
            "timeout": timeout,
        }
        if system:
            call_kwargs["system"] = system
        call_kwargs.update(kwargs)

        resp = await self.client.messages.create(**call_kwargs)
        content = ""
        for block in resp.content:
            if block.type == "text":
                content += block.text

        return ChatResponse(
            content=content,
            tokens_input=resp.usage.input_tokens,
            tokens_output=resp.usage.output_tokens,
            raw=resp,
        )

    async def chat_stream(
        self,
        messages: list[dict[str, Any]],
        *,
        timeout: int = 60,
        **kwargs: Any,
    ) -> AsyncIterator[StreamChunk]:
        system, msgs = self._extract_system(messages)
        call_kwargs: dict[str, Any] = {
            "model": self.model,
            "messages": msgs,
            "max_tokens": kwargs.pop("max_tokens", 4096),
            "timeout": timeout,
        }
        if system:
            call_kwargs["system"] = system
        call_kwargs.update(kwargs)

        first = True
        async with self.client.messages.stream(**call_kwargs) as stream:
            async for text in stream.text_stream:
                yield StreamChunk(content=text, is_first=first)
                if first:
                    first = False

            final = await stream.get_final_message()
            yield StreamChunk(
                is_done=True,
                tokens_input=final.usage.input_tokens,
                tokens_output=final.usage.output_tokens,
            )

    async def chat_with_tools(
        self,
        messages: list[dict[str, Any]],
        tools: list[dict[str, Any]],
        *,
        timeout: int = 60,
        **kwargs: Any,
    ) -> ChatResponse:
        system, msgs = self._extract_system(messages)

        # Convert OpenAI tool format to Anthropic format
        anthropic_tools = []
        for t in tools:
            func = t.get("function", t)
            anthropic_tools.append({
                "name": func["name"],
                "description": func.get("description", ""),
                "input_schema": func.get("parameters", func.get("input_schema", {})),
            })

        call_kwargs: dict[str, Any] = {
            "model": self.model,
            "messages": msgs,
            "tools": anthropic_tools,
            "max_tokens": kwargs.pop("max_tokens", 4096),
            "timeout": timeout,
        }
        if system:
            call_kwargs["system"] = system
        call_kwargs.update(kwargs)

        resp = await self.client.messages.create(**call_kwargs)

        content = ""
        tool_calls: list[ToolCall] = []
        for block in resp.content:
            if block.type == "text":
                content += block.text
            elif block.type == "tool_use":
                tool_calls.append(ToolCall(
                    id=block.id,
                    name=block.name,
                    arguments=block.input if isinstance(block.input, dict) else {},
                ))

        return ChatResponse(
            content=content,
            tool_calls=tool_calls,
            tokens_input=resp.usage.input_tokens,
            tokens_output=resp.usage.output_tokens,
            raw=resp,
        )

    async def close(self) -> None:
        await self.client.close()
