from __future__ import annotations

import json
import time

from raccoon.database import TestResult
from raccoon.providers.base import LLMProvider

from .basic_call import _classify_error
from .base import Benchmark

TOOLS = [
    {
        "type": "function",
        "function": {
            "name": "get_weather",
            "description": "Get current weather for a city. Returns temperature in Celsius.",
            "parameters": {
                "type": "object",
                "properties": {
                    "city": {"type": "string", "description": "City name"},
                },
                "required": ["city"],
            },
        },
    },
    {
        "type": "function",
        "function": {
            "name": "calculate",
            "description": "Evaluate a mathematical expression and return the numeric result.",
            "parameters": {
                "type": "object",
                "properties": {
                    "expression": {"type": "string", "description": "Math expression, e.g. '22 * 9/5 + 32'"},
                },
                "required": ["expression"],
            },
        },
    },
]

# Mock tool responses
MOCK_RESPONSES = {
    "get_weather": lambda args: json.dumps({"temperature": 22, "unit": "celsius", "condition": "sunny", "city": args.get("city", "")}),
    "calculate": lambda args: json.dumps({"result": _safe_eval(args.get("expression", "0"))}),
}

PROMPT = (
    "What is the temperature in Tokyo in Fahrenheit? "
    "First use get_weather to find the temperature in Celsius, "
    "then use calculate to convert it. Formula: F = C * 9/5 + 32"
)

MAX_STEPS = 6


def _safe_eval(expr: str) -> float:
    """Evaluate simple math expressions safely."""
    allowed = set("0123456789+-*/.() ")
    cleaned = "".join(c for c in expr if c in allowed)
    try:
        return round(float(eval(cleaned)), 2)  # noqa: S307
    except Exception:
        return 0.0


class AgentLoopBenchmark(Benchmark):
    name = "agent_loop"
    description = "Multi-step agent - tool chaining and task completion"

    async def run(self, provider: LLMProvider, run_id: str, timeout: int) -> TestResult:
        result = TestResult(
            run_id=run_id,
            provider=provider.name,
            model=provider.model,
            test_type=self.name,
        )
        t0 = time.perf_counter()
        total_in = 0
        total_out = 0

        try:
            messages: list[dict] = [{"role": "user", "content": PROMPT}]
            called_tools: list[str] = []
            steps = 0

            while steps < MAX_STEPS:
                steps += 1
                resp = await provider.chat_with_tools(messages, TOOLS, timeout=timeout)
                total_in += resp.tokens_input
                total_out += resp.tokens_output

                if not resp.tool_calls:
                    # Model finished (gave final answer)
                    break

                # Build assistant message and tool result messages
                # For OpenAI-compatible: append assistant msg with tool_calls, then tool results
                # For Anthropic: handled similarly through the provider abstraction
                assistant_msg = _build_assistant_msg(resp, provider.name)
                messages.append(assistant_msg)

                for tc in resp.tool_calls:
                    called_tools.append(tc.name)
                    mock_fn = MOCK_RESPONSES.get(tc.name)
                    tool_output = mock_fn(tc.arguments) if mock_fn else '{"error": "unknown tool"}'
                    messages.append({
                        "role": "tool",
                        "tool_call_id": tc.id,
                        "content": tool_output,
                    })

            elapsed = (time.perf_counter() - t0) * 1000
            result.latency_total_ms = round(elapsed, 1)
            result.tokens_input = total_in
            result.tokens_output = total_out
            if total_out and elapsed > 0:
                result.throughput_tps = round(total_out / (elapsed / 1000), 1)

            # Score: did the model call both tools correctly?
            score = 0.0
            if "get_weather" in called_tools:
                score += 0.4
            if "calculate" in called_tools:
                score += 0.4
            # Check if final answer mentions 71.6 (22 * 9/5 + 32)
            final_content = resp.content if resp else ""
            if "71.6" in final_content or "71.60" in final_content:
                score += 0.2
            elif any(str(x) in final_content for x in ["71", "72"]):
                score += 0.1  # close enough

            result.score = round(score, 2)
            result.success = score >= 0.8
            result.metadata = {"steps": steps, "tools_called": called_tools}

            if not result.success:
                result.error_type = "content_error"
                result.error_message = f"Score {score}: tools={called_tools}, steps={steps}"

        except Exception as e:
            elapsed = (time.perf_counter() - t0) * 1000
            result.latency_total_ms = round(elapsed, 1)
            result.tokens_input = total_in
            result.tokens_output = total_out
            err_type = _classify_error(e)
            msg = str(e).lower()
            if "tool" in msg and ("not support" in msg or "unsupported" in msg):
                err_type = "unsupported"
            result.error_type = err_type
            result.error_message = str(e)[:500]

        return result


def _build_assistant_msg(resp, provider_name: str) -> dict:
    """Build an assistant message that includes tool calls for the conversation history."""
    msg: dict = {"role": "assistant", "content": resp.content or ""}
    if resp.tool_calls:
        msg["tool_calls"] = [
            {
                "id": tc.id,
                "type": "function",
                "function": {
                    "name": tc.name,
                    "arguments": json.dumps(tc.arguments),
                },
            }
            for tc in resp.tool_calls
        ]
    return msg
