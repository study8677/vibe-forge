from __future__ import annotations

import time
import traceback

from raccoon.database import TestResult
from raccoon.providers.base import LLMProvider

from .base import Benchmark


class BasicCallBenchmark(Benchmark):
    name = "basic_call"
    description = "Basic API call - success and latency"

    PROMPT = [{"role": "user", "content": "What is 2+2? Answer with just the number, nothing else."}]

    async def run(self, provider: LLMProvider, run_id: str, timeout: int) -> TestResult:
        result = TestResult(
            run_id=run_id,
            provider=provider.name,
            model=provider.model,
            test_type=self.name,
        )
        t0 = time.perf_counter()
        try:
            resp = await provider.chat(self.PROMPT, timeout=timeout)
            elapsed = (time.perf_counter() - t0) * 1000

            result.latency_total_ms = round(elapsed, 1)
            result.tokens_input = resp.tokens_input
            result.tokens_output = resp.tokens_output
            if resp.tokens_output and elapsed > 0:
                result.throughput_tps = round(resp.tokens_output / (elapsed / 1000), 1)

            # Validate response contains "4"
            if "4" in resp.content:
                result.success = True
                result.score = 1.0
            else:
                result.success = False
                result.score = 0.0
                result.error_type = "content_error"
                result.error_message = f"Expected '4' in response, got: {resp.content[:200]}"

        except Exception as e:
            elapsed = (time.perf_counter() - t0) * 1000
            result.latency_total_ms = round(elapsed, 1)
            result.error_type = _classify_error(e)
            result.error_message = str(e)[:500]

        return result


def _classify_error(e: Exception) -> str:
    msg = str(e).lower()
    type_name = type(e).__name__.lower()

    if "timeout" in type_name or "timeout" in msg:
        return "timeout"
    if "401" in msg or "403" in msg or "auth" in msg or "unauthorized" in msg:
        return "auth_error"
    if "429" in msg or "rate" in msg:
        return "rate_limit"
    if "500" in msg or "502" in msg or "503" in msg or "server" in msg:
        return "server_error"
    if "connect" in msg or "network" in msg or "dns" in msg:
        return "network_error"
    return "unknown"
