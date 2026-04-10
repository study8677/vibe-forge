from __future__ import annotations

import time

from raccoon.database import TestResult
from raccoon.providers.base import LLMProvider

from .basic_call import _classify_error
from .base import Benchmark


class StreamingBenchmark(Benchmark):
    name = "streaming"
    description = "Streaming response - TTFB and throughput"

    PROMPT = [{"role": "user", "content": "Count from 1 to 20, one number per line."}]

    async def run(self, provider: LLMProvider, run_id: str, timeout: int) -> TestResult:
        result = TestResult(
            run_id=run_id,
            provider=provider.name,
            model=provider.model,
            test_type=self.name,
        )
        t0 = time.perf_counter()
        t_first: float | None = None
        full_content = ""

        try:
            async for chunk in provider.chat_stream(self.PROMPT, timeout=timeout):
                if chunk.is_first and chunk.content:
                    t_first = time.perf_counter()
                full_content += chunk.content
                if chunk.is_done:
                    result.tokens_input = chunk.tokens_input
                    result.tokens_output = chunk.tokens_output

            elapsed = (time.perf_counter() - t0) * 1000
            result.latency_total_ms = round(elapsed, 1)
            if t_first is not None:
                result.latency_first_token_ms = round((t_first - t0) * 1000, 1)

            if result.tokens_output and elapsed > 0:
                result.throughput_tps = round(result.tokens_output / (elapsed / 1000), 1)

            # Validate: should contain numbers 1-20
            found = sum(1 for i in range(1, 21) if str(i) in full_content)
            if found >= 15:  # allow some flexibility
                result.success = True
                result.score = round(found / 20, 2)
            else:
                result.success = False
                result.score = round(found / 20, 2)
                result.error_type = "content_error"
                result.error_message = f"Only found {found}/20 numbers in response"

        except Exception as e:
            elapsed = (time.perf_counter() - t0) * 1000
            result.latency_total_ms = round(elapsed, 1)
            if t_first is not None:
                result.latency_first_token_ms = round((t_first - t0) * 1000, 1)
            result.error_type = _classify_error(e)
            result.error_message = str(e)[:500]

        return result
