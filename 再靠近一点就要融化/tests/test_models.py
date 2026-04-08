import unittest

from research_agent.models import ExecutionResult, ExperimentIdea, Paper


class ModelShapeTest(unittest.TestCase):
    def test_models_capture_core_fields(self) -> None:
        paper = Paper(
            paper_id="1234.5678",
            title="A Paper",
            summary="Summary",
            authors=["Alice"],
            pdf_url="https://arxiv.org/pdf/1234.5678.pdf",
            published="2026-01-01T00:00:00Z",
        )
        idea = ExperimentIdea(
            title="Try a baseline",
            hypothesis="The baseline will converge",
            method="Train a small model",
            success_metric="validation accuracy",
        )
        result = ExecutionResult(success=True, returncode=0, stdout="ok", stderr="")
        self.assertEqual(paper.paper_id, "1234.5678")
        self.assertEqual(idea.success_metric, "validation accuracy")
        self.assertTrue(result.success)
