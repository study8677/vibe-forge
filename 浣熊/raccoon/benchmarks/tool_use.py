from __future__ import annotations

import time

from raccoon.database import TestResult
from raccoon.providers.base import LLMProvider

from .basic_call import _classify_error
from .base import Benchmark

WEATHER_TOOL = {
    "type": "function",
    "function": {
        "name": "get_weather",
        "description": "Get the current weather for a given city",
        "parameters": {
            "type": "object",
            "properties": {
                "city": {
                    "type": "string",
                    "description": "The city name, e.g. 'Tokyo'",
                },
            },
            "required": ["city"],
        },
    },
}


class ToolUseBenchmark(Benchmark):
    name = "tool_use"
    description = "Tool/function calling - correctness of tool invocation"

    PROMPT = [{"role": "user", "content": "What's the weather like in Tokyo right now?"}]
    TOOLS = [WEATHER_TOOL]

    async def run(self, provider: LLMProvider, run_id: str, timeout: int) -> TestResult:
        result = TestResult(
            run_id=run_id,
            provider=provider.name,
            model=provider.model,
            test_type=self.name,
        )
        t0 = time.perf_counter()
        try:
            resp = await provider.chat_with_tools(self.PROMPT, self.TOOLS, timeout=timeout)
            elapsed = (time.perf_counter() - t0) * 1000

            result.latency_total_ms = round(elapsed, 1)
            result.tokens_input = resp.tokens_input
            result.tokens_output = resp.tokens_output
            if resp.tokens_output and elapsed > 0:
                result.throughput_tps = round(resp.tokens_output / (elapsed / 1000), 1)

            # Score the tool call
            score = 0.0
            if resp.tool_calls:
                tc = resp.tool_calls[0]
                if tc.name == "get_weather":
                    score += 0.5  # correct tool
                    city = tc.arguments.get("city", "").lower()
                    if "tokyo" in city or "东京" in city:
                        score += 0.5  # correct argument
            result.score = score
            result.success = score >= 0.5  # at least called the right tool
            if not result.success:
                result.error_type = "content_error"
                result.error_message = "Model did not call get_weather tool"

        except Exception as e:
            elapsed = (time.perf_counter() - t0) * 1000
            result.latency_total_ms = round(elapsed, 1)
            err_type = _classify_error(e)
            # Check for unsupported feature
            msg = str(e).lower()
            if "tool" in msg and ("not support" in msg or "unsupported" in msg or "invalid" in msg):
                err_type = "unsupported"
            result.error_type = err_type
            result.error_message = str(e)[:500]

        return result
