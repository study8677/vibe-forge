import unittest

from research_agent.cli import build_parser


class CliParserTest(unittest.TestCase):
    def test_cli_parses_goal_and_cycle_flags(self) -> None:
        parser = build_parser()
        args = parser.parse_args(["run", "--goal", "study agents", "--max-cycles", "2"])
        self.assertEqual(args.command, "run")
        self.assertEqual(args.goal, "study agents")
        self.assertEqual(args.max_cycles, 2)
