from __future__ import annotations

from abc import ABC, abstractmethod

from raccoon.database import TestResult
from raccoon.providers.base import LLMProvider


class Benchmark(ABC):
    name: str
    description: str

    @abstractmethod
    async def run(self, provider: LLMProvider, run_id: str, timeout: int) -> TestResult: ...
