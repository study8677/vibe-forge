import unittest

from research_agent.arxiv import parse_arxiv_feed

SAMPLE_FEED = """<?xml version="1.0" encoding="UTF-8"?>
<feed xmlns="http://www.w3.org/2005/Atom">
  <entry>
    <id>http://arxiv.org/abs/1234.5678v1</id>
    <updated>2026-04-01T00:00:00Z</updated>
    <published>2026-04-01T00:00:00Z</published>
    <title> Sample Paper </title>
    <summary> Example abstract. </summary>
    <author><name>Alice</name></author>
    <author><name>Bob</name></author>
    <link title="pdf" href="http://arxiv.org/pdf/1234.5678v1"/>
  </entry>
</feed>
"""


class ArxivParserTest(unittest.TestCase):
    def test_parse_arxiv_feed_extracts_papers(self) -> None:
        papers = parse_arxiv_feed(SAMPLE_FEED)
        self.assertEqual(len(papers), 1)
        self.assertEqual(papers[0].paper_id, "1234.5678v1")
        self.assertEqual(papers[0].authors, ["Alice", "Bob"])
        self.assertEqual(papers[0].title, "Sample Paper")
