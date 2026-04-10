"""Abstract operation handler.

Each concrete handler knows how to translate between the gateway's
standardised param models and the two wire formats (SOAP XML / REST JSON).
"""

from __future__ import annotations

from abc import ABC, abstractmethod
from dataclasses import dataclass
from typing import Any

from ..config import AccountConfig


@dataclass
class SoapRequestInfo:
    """Everything the SOAP adapter needs to fire a request."""
    service: str          # e.g. "AirService"
    xml_body: str         # inner XML (goes inside <soapenv:Body>)
    soap_action: str = "" # optional SOAPAction header


@dataclass
class RestRequestInfo:
    """Everything the REST adapter needs to fire a request."""
    path: str             # URL path appended to base_url
    method: str = "POST"
    body: dict[str, Any] | None = None


class OperationHandler(ABC):
    """Bidirectional transformer between standardised params and wire formats."""

    name: str = ""       # e.g. "air_search"
    soap_service: str = "AirService"

    @abstractmethod
    def build_soap(self, params: dict[str, Any], account: AccountConfig, schema_ver: str) -> SoapRequestInfo:
        """Build SOAP/XML request from standardised params."""
        ...

    @abstractmethod
    def build_rest(self, params: dict[str, Any], account: AccountConfig) -> RestRequestInfo:
        """Build REST/JSON request from standardised params."""
        ...

    @abstractmethod
    def parse_soap(self, xml_text: str) -> dict[str, Any]:
        """Parse SOAP response XML into standardised result dict."""
        ...

    @abstractmethod
    def parse_rest(self, json_data: dict[str, Any]) -> dict[str, Any]:
        """Parse REST response JSON into standardised result dict."""
        ...
