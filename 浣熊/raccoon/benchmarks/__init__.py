from .agent_loop import AgentLoopBenchmark
from .base import Benchmark
from .basic_call import BasicCallBenchmark
from .json_output import JsonOutputBenchmark
from .streaming import StreamingBenchmark
from .tool_use import ToolUseBenchmark

ALL_BENCHMARKS: dict[str, Benchmark] = {
    "basic_call": BasicCallBenchmark(),
    "streaming": StreamingBenchmark(),
    "tool_use": ToolUseBenchmark(),
    "agent_loop": AgentLoopBenchmark(),
    "json_output": JsonOutputBenchmark(),
}

__all__ = ["ALL_BENCHMARKS", "Benchmark"]
