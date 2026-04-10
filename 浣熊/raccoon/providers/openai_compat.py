from __future__ import annotations

import json
from typing import Any, AsyncIterator

from openai import AsyncOpenAI

from .base import ChatResponse, LLMProvider, StreamChunk, ToolCall


class OpenAICompatProvider(LLMProvider):
    """Provider for OpenAI and all OpenAI-compatible APIs (DeepSeek, Qwen, Moonshot, Zhipu, Google, etc.)."""

    def __init__(self, name: str, model: str, api_key: str, base_url: str = ""):
        self.name = name
        self.model = model
        kwargs: dict[str, Any] = {"api_key": api_key}
        if base_url:
            kwargs["base_url"] = base_url
        self.client = AsyncOpenAI(**kwargs)

    async def chat(
        self,
        messages: list[dict[str, Any]],
        *,
        timeout: int = 30,
        **kwargs: Any,
    ) -> ChatResponse:
        resp = await self.client.chat.completions.create(
            model=self.model,
            messages=messages,
            timeout=timeout,
            **kwargs,
        )
        choice = resp.choices[0]
        usage = resp.usage
        return ChatResponse(
            content=choice.message.content or "",
            tokens_input=usage.prompt_tokens if usage else 0,
            tokens_output=usage.completion_tokens if usage else 0,
            raw=resp,
        )

    async def chat_stream(
        self,
        messages: list[dict[str, Any]],
        *,
        timeout: int = 60,
        **kwargs: Any,
    ) -> AsyncIterator[StreamChunk]:
        stream = await self.client.chat.completions.create(
            model=self.model,
            messages=messages,
            stream=True,
            stream_options={"include_usage": True},
            timeout=timeout,
            **kwargs,
        )
        first = True
        async for chunk in stream:
            if not chunk.choices:
                # usage-only final chunk
                if chunk.usage:
                    yield StreamChunk(
                        is_done=True,
                        tokens_input=chunk.usage.prompt_tokens,
                        tokens_output=chunk.usage.completion_tokens,
                    )
                continue
            delta = chunk.choices[0].delta
            content = delta.content or ""
            is_done = chunk.choices[0].finish_reason is not None
            yield StreamChunk(content=content, is_first=first, is_done=is_done)
            if first and content:
                first = False

    async def chat_with_tools(
        self,
        messages: list[dict[str, Any]],
        tools: list[dict[str, Any]],
        *,
        timeout: int = 60,
        **kwargs: Any,
    ) -> ChatResponse:
        resp = await self.client.chat.completions.create(
            model=self.model,
            messages=messages,
            tools=tools,
            timeout=timeout,
            **kwargs,
        )
        choice = resp.choices[0]
        usage = resp.usage

        tool_calls: list[ToolCall] = []
        if choice.message.tool_calls:
            for tc in choice.message.tool_calls:
                try:
                    args = json.loads(tc.function.arguments)
                except (json.JSONDecodeError, TypeError):
                    args = {"_raw": tc.function.arguments}
                tool_calls.append(ToolCall(
                    id=tc.id,
                    name=tc.function.name,
                    arguments=args,
                ))

        return ChatResponse(
            content=choice.message.content or "",
            tool_calls=tool_calls,
            tokens_input=usage.prompt_tokens if usage else 0,
            tokens_output=usage.completion_tokens if usage else 0,
            raw=resp,
        )

    async def close(self) -> None:
        await self.client.close()
