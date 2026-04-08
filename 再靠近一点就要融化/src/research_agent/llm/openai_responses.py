from __future__ import annotations

import json
import urllib.error
import urllib.request
from dataclasses import asdict
from typing import Any

from research_agent.config import OpenAIConfig
from research_agent.llm.base import ResearchLLM
from research_agent.models import EvaluationResult, ExperimentIdea, Paper, PaperDigest, RankedPaper


def _strip_code_fences(text: str) -> str:
    stripped = text.strip()
    if stripped.startswith("```"):
        lines = stripped.splitlines()
        if lines:
            lines = lines[1:]
        if lines and lines[-1].strip() == "```":
            lines = lines[:-1]
        return "\n".join(lines).strip()
    return stripped


class OpenAIResponsesClient:
    def __init__(self, config: OpenAIConfig):
        self.config = config
        self.endpoint = f"{config.api_base.rstrip('/')}/responses"

    def create_text(self, *, system_prompt: str, user_prompt: str) -> str:
        payload = {
            "model": self.config.model,
            "input": self._build_input(system_prompt=system_prompt, user_prompt=user_prompt),
            "reasoning": {"effort": self.config.reasoning_effort},
        }
        response = self._request(payload)
        return _strip_code_fences(self._extract_output_text(response))

    def create_json(
        self,
        *,
        system_prompt: str,
        user_prompt: str,
        schema_name: str,
        schema: dict[str, Any],
    ) -> dict[str, Any]:
        payload = {
            "model": self.config.model,
            "input": self._build_input(system_prompt=system_prompt, user_prompt=user_prompt),
            "reasoning": {"effort": self.config.reasoning_effort},
            "text": {
                "format": {
                    "type": "json_schema",
                    "name": schema_name,
                    "strict": True,
                    "schema": schema,
                }
            },
        }
        response = self._request(payload)
        raw = self._extract_output_text(response)
        try:
            return json.loads(raw)
        except json.JSONDecodeError as exc:
            raise ValueError(
                f"Model did not return valid JSON for schema {schema_name}: {raw}"
            ) from exc

    def _request(self, payload: dict[str, Any]) -> dict[str, Any]:
        request = urllib.request.Request(
            self.endpoint,
            data=json.dumps(payload).encode("utf-8"),
            headers={
                "Authorization": f"Bearer {self.config.api_key}",
                "Content-Type": "application/json",
            },
            method="POST",
        )
        try:
            with urllib.request.urlopen(request, timeout=self.config.timeout_seconds) as response:
                return json.loads(response.read().decode("utf-8"))
        except urllib.error.HTTPError as exc:
            detail = exc.read().decode("utf-8", errors="replace")
            raise RuntimeError(f"OpenAI Responses API request failed: {exc.code} {detail}") from exc

    @staticmethod
    def _build_input(*, system_prompt: str, user_prompt: str) -> list[dict[str, Any]]:
        return [
            {
                "role": "system",
                "content": [{"type": "input_text", "text": system_prompt}],
            },
            {
                "role": "user",
                "content": [{"type": "input_text", "text": user_prompt}],
            },
        ]

    @staticmethod
    def _extract_output_text(response: dict[str, Any]) -> str:
        if isinstance(response.get("output_text"), str) and response["output_text"].strip():
            return response["output_text"]

        parts: list[str] = []
        for item in response.get("output", []):
            for content in item.get("content", []):
                text = content.get("text")
                if isinstance(text, str):
                    parts.append(text)
        if parts:
            return "\n".join(parts)
        raise ValueError(f"Unable to extract text from OpenAI response: {response}")


class OpenAIResearchLLM(ResearchLLM):
    def __init__(self, client: OpenAIResponsesClient):
        self.client = client

    def rank_papers(self, goal: str, papers: list[Paper]) -> list[RankedPaper]:
        schema = {
            "type": "object",
            "properties": {
                "ranked": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "paper_id": {"type": "string"},
                            "relevance_score": {"type": "number"},
                            "rationale": {"type": "string"},
                        },
                        "required": ["paper_id", "relevance_score", "rationale"],
                        "additionalProperties": False,
                    },
                }
            },
            "required": ["ranked"],
            "additionalProperties": False,
        }
        payload = self.client.create_json(
            system_prompt=(
                "You rank arXiv papers for a research goal. Return only structured JSON. "
                "Prefer papers with direct methodological relevance."
            ),
            user_prompt=(
                f"Research goal:\n{goal}\n\nCandidate papers:\n"
                f"{json.dumps([asdict(paper) for paper in papers], ensure_ascii=False, indent=2)}"
            ),
            schema_name="paper_ranking",
            schema=schema,
        )
        paper_lookup = {paper.paper_id: paper for paper in papers}
        ranked: list[RankedPaper] = []
        for item in payload["ranked"]:
            paper = paper_lookup.get(item["paper_id"])
            if paper is None:
                continue
            ranked.append(
                RankedPaper(
                    paper=paper,
                    relevance_score=float(item["relevance_score"]),
                    rationale=item["rationale"],
                )
            )
        return ranked

    def digest_papers(self, goal: str, ranked_papers: list[RankedPaper]) -> list[PaperDigest]:
        schema = {
            "type": "object",
            "properties": {
                "digests": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "paper_id": {"type": "string"},
                            "takeaway": {"type": "string"},
                            "relevance": {"type": "string"},
                        },
                        "required": ["paper_id", "takeaway", "relevance"],
                        "additionalProperties": False,
                    },
                }
            },
            "required": ["digests"],
            "additionalProperties": False,
        }
        payload = self.client.create_json(
            system_prompt=(
                "You read ranked research papers and extract short actionable "
                "takeaways for experiment design."
            ),
            user_prompt=(
                f"Research goal:\n{goal}\n\nRanked papers:\n"
                f"{json.dumps(
                    [self._ranked_to_payload(item) for item in ranked_papers],
                    ensure_ascii=False,
                    indent=2,
                )}"
            ),
            schema_name="paper_digests",
            schema=schema,
        )
        return [PaperDigest(**item) for item in payload["digests"]]

    def generate_idea(
        self,
        goal: str,
        digests: list[PaperDigest],
        previous_cycles: list[object],
    ) -> ExperimentIdea:
        schema = {
            "type": "object",
            "properties": {
                "title": {"type": "string"},
                "hypothesis": {"type": "string"},
                "method": {"type": "string"},
                "success_metric": {"type": "string"},
                "risks": {"type": "array", "items": {"type": "string"}},
            },
            "required": ["title", "hypothesis", "method", "success_metric", "risks"],
            "additionalProperties": False,
        }
        payload = self.client.create_json(
            system_prompt=(
                "You design one concrete next experiment for a research agent. "
                "Keep the idea runnable as a single Python script."
            ),
            user_prompt=(
                f"Research goal:\n{goal}\n\nPaper digests:\n"
                f"{json.dumps(
                    [asdict(item) for item in digests],
                    ensure_ascii=False,
                    indent=2,
                )}\n\n"
                f"Previous cycles:\n{json.dumps(
                    previous_cycles,
                    ensure_ascii=False,
                    indent=2,
                    default=str,
                )}"
            ),
            schema_name="experiment_idea",
            schema=schema,
        )
        return ExperimentIdea(**payload)

    def generate_code(
        self,
        goal: str,
        idea: ExperimentIdea,
        digests: list[PaperDigest],
        previous_attempts: list[object],
    ) -> str:
        return self.client.create_text(
            system_prompt=(
                "You write a single Python script for a research experiment. "
                "Return only code, no markdown. Prefer the Python standard library. "
                "The script must be self-contained and runnable with python3."
            ),
            user_prompt=(
                f"Research goal:\n{goal}\n\nExperiment idea:\n"
                f"{json.dumps(asdict(idea), ensure_ascii=False, indent=2)}\n\n"
                f"Paper digests:\n"
                f"{json.dumps(
                    [asdict(item) for item in digests],
                    ensure_ascii=False,
                    indent=2,
                )}\n\n"
                f"Previous attempts:\n"
                f"{json.dumps(
                    previous_attempts,
                    ensure_ascii=False,
                    indent=2,
                    default=str,
                )}"
            ),
        )

    def repair_code(
        self,
        goal: str,
        idea: ExperimentIdea,
        broken_code: str,
        error_context: str,
        previous_attempts: list[object],
    ) -> str:
        return self.client.create_text(
            system_prompt=(
                "You fix a broken Python script. Return only the full corrected code. "
                "Do not explain the fix. Keep the script self-contained."
            ),
            user_prompt=(
                f"Research goal:\n{goal}\n\nExperiment idea:\n"
                f"{json.dumps(asdict(idea), ensure_ascii=False, indent=2)}\n\n"
                f"Current broken code:\n{broken_code}\n\n"
                f"Execution error:\n{error_context}\n\n"
                f"Previous attempts:\n"
                f"{json.dumps(
                    previous_attempts,
                    ensure_ascii=False,
                    indent=2,
                    default=str,
                )}"
            ),
        )

    def evaluate_cycle(
        self,
        goal: str,
        idea: ExperimentIdea,
        execution_result: object,
        digests: list[PaperDigest],
        previous_cycles: list[object],
    ) -> EvaluationResult:
        schema = {
            "type": "object",
            "properties": {
                "summary": {"type": "string"},
                "strengths": {"type": "array", "items": {"type": "string"}},
                "limitations": {"type": "array", "items": {"type": "string"}},
                "confidence": {"type": "number"},
                "recommendation": {"type": "string", "enum": ["stop", "continue"]},
                "next_step": {"type": "string"},
            },
            "required": [
                "summary",
                "strengths",
                "limitations",
                "confidence",
                "recommendation",
                "next_step",
            ],
            "additionalProperties": False,
        }
        payload = self.client.create_json(
            system_prompt=(
                "You evaluate a research cycle. Decide whether the agent should stop or continue. "
                "Use `continue` only when another cycle is justified."
            ),
            user_prompt=(
                f"Research goal:\n{goal}\n\nExperiment idea:\n"
                f"{json.dumps(asdict(idea), ensure_ascii=False, indent=2)}\n\n"
                f"Execution result:\n"
                f"{json.dumps(
                    execution_result,
                    ensure_ascii=False,
                    indent=2,
                    default=str,
                )}\n\n"
                f"Paper digests:\n"
                f"{json.dumps(
                    [asdict(item) for item in digests],
                    ensure_ascii=False,
                    indent=2,
                )}\n\n"
                f"Previous cycles:\n"
                f"{json.dumps(
                    previous_cycles,
                    ensure_ascii=False,
                    indent=2,
                    default=str,
                )}"
            ),
            schema_name="cycle_evaluation",
            schema=schema,
        )
        return EvaluationResult(**payload)

    @staticmethod
    def _ranked_to_payload(item: RankedPaper) -> dict[str, Any]:
        return {
            "paper": asdict(item.paper),
            "relevance_score": item.relevance_score,
            "rationale": item.rationale,
        }
