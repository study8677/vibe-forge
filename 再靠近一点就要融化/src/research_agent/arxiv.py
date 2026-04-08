from __future__ import annotations

import urllib.parse
import urllib.request
import xml.etree.ElementTree as ET

from research_agent.models import Paper

ATOM_NS = {"atom": "http://www.w3.org/2005/Atom"}


def build_arxiv_query(goal: str, limit: int) -> str:
    params = {
        "search_query": f"all:{goal}",
        "start": 0,
        "max_results": limit,
        "sortBy": "relevance",
        "sortOrder": "descending",
    }
    return urllib.parse.urlencode(params)


def parse_arxiv_feed(xml_text: str) -> list[Paper]:
    root = ET.fromstring(xml_text)
    papers: list[Paper] = []
    for entry in root.findall("atom:entry", ATOM_NS):
        paper_id = entry.findtext("atom:id", default="", namespaces=ATOM_NS).rsplit("/", 1)[-1]
        title = " ".join(entry.findtext("atom:title", default="", namespaces=ATOM_NS).split())
        summary = " ".join(entry.findtext("atom:summary", default="", namespaces=ATOM_NS).split())
        authors = [
            author.findtext("atom:name", default="", namespaces=ATOM_NS)
            for author in entry.findall("atom:author", ATOM_NS)
        ]
        pdf_url = ""
        for link in entry.findall("atom:link", ATOM_NS):
            if link.attrib.get("title") == "pdf":
                pdf_url = link.attrib.get("href", "")
                break
        papers.append(
            Paper(
                paper_id=paper_id,
                title=title,
                summary=summary,
                authors=authors,
                pdf_url=pdf_url,
                published=entry.findtext("atom:published", default="", namespaces=ATOM_NS),
            )
        )
    return papers


class ArxivClient:
    def __init__(
        self,
        base_url: str = "https://export.arxiv.org/api/query",
        timeout_seconds: int = 30,
    ):
        self.base_url = base_url
        self.timeout_seconds = timeout_seconds

    def search(self, goal: str, limit: int) -> list[Paper]:
        query = build_arxiv_query(goal=goal, limit=limit)
        url = f"{self.base_url}?{query}"
        with urllib.request.urlopen(url, timeout=self.timeout_seconds) as response:
            payload = response.read().decode("utf-8")
        return parse_arxiv_feed(payload)
