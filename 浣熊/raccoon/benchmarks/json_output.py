from __future__ import annotations

import json
import time

from raccoon.database import TestResult
from raccoon.providers.base import LLMProvider

from .basic_call import _classify_error
from .base import Benchmark


class JsonOutputBenchmark(Benchmark):
    name = "json_output"
    description = "JSON structured output - format compliance"

    PROMPT = [
        {
            "role": "system",
            "content": "You are an API that only responds in valid JSON. No markdown, no explanation, just pure JSON.",
        },
        {
            "role": "user",
            "content": (
                'List exactly 3 programming languages with their creation year. '
                'Respond with a JSON array using this exact schema: '
                '[{"name": "string", "year": number}]. '
                'Output ONLY the JSON array, no other text.'
            ),
        },
    ]

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

            # Score the JSON output
            score = 0.0
            content = resp.content.strip()

            # Strip markdown code fences if present
            if content.startswith("```"):
                lines = content.split("\n")
                lines = [l for l in lines if not l.strip().startswith("```")]
                content = "\n".join(lines).strip()

            try:
                data = json.loads(content)
                score += 0.3  # valid JSON

                if isinstance(data, list):
                    score += 0.2  # correct type (array)

                    if len(data) == 3:
                        score += 0.1  # correct count

                    valid_items = 0
                    for item in data:
                        if isinstance(item, dict):
                            has_name = "name" in item and isinstance(item["name"], str)
                            has_year = "year" in item and isinstance(item["year"], (int, float))
                            if has_name and has_year:
                                valid_items += 1

                    if data:
                        score += 0.4 * (valid_items / len(data))

            except json.JSONDecodeError:
                score = 0.0
                result.error_type = "content_error"
                result.error_message = f"Invalid JSON: {content[:200]}"

            result.score = round(score, 2)
            result.success = score >= 0.5

        except Exception as e:
            elapsed = (time.perf_counter() - t0) * 1000
            result.latency_total_ms = round(elapsed, 1)
            result.error_type = _classify_error(e)
            result.error_message = str(e)[:500]

        return result
