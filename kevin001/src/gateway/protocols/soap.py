"""SOAP/XML protocol adapter for Travelport UAPI.

Responsibilities:
 - Wrap an XML body in a SOAP envelope
 - Send via HTTP POST with Basic-Auth + SOAPAction header
 - Detect SOAP faults and extract error information
 - Return the raw XML response text
"""

from __future__ import annotations

import logging
from typing import Any
from xml.etree.ElementTree import Element, SubElement, tostring, fromstring

import httpx

from ..credentials import CredentialManager
from .base import ProtocolAdapter

logger = logging.getLogger(__name__)

# Travelport UAPI SOAP namespace
SOAP_NS = "http://schemas.xmlsoap.org/soap/envelope/"


class SoapAdapter(ProtocolAdapter):
    """Send SOAP/XML requests to Travelport UAPI endpoints."""

    def __init__(self, credential_manager: CredentialManager, timeout: int = 30):
        super().__init__(credential_manager, timeout)

    # ── public API ───────────────────────────────────────────────────────

    async def send(
        self,
        account_id: str,
        service: str,
        payload: str | dict,
        *,
        extra_headers: dict[str, str] | None = None,
    ) -> tuple[int, str]:
        """
        Parameters
        ----------
        account_id : Account to use for credentials + endpoint.
        service    : Travelport service name (``AirService``, ``HotelService``, …).
        payload    : The **inner** XML body (everything inside ``<soapenv:Body>``).
                     Can be a raw XML string.
        extra_headers : Additional HTTP headers (e.g. ``SOAPAction``).

        Returns
        -------
        (http_status, response_xml_string)
        """
        endpoint = self.creds.soap_endpoint(account_id, service)
        auth_headers = self.creds.soap_auth_header(account_id)

        if isinstance(payload, str):
            body_xml = payload
        else:
            raise TypeError("SOAP payload must be an XML string")

        envelope = self._wrap_envelope(body_xml)

        headers = {
            "Content-Type": "text/xml; charset=utf-8",
            **auth_headers,
        }
        if extra_headers:
            headers.update(extra_headers)

        logger.debug("SOAP → %s [%s]", endpoint, service)

        async with httpx.AsyncClient(timeout=self.timeout) as client:
            resp = await client.post(endpoint, content=envelope, headers=headers)

        if resp.status_code >= 400:
            logger.error(
                "SOAP fault: HTTP %d from %s\n%s",
                resp.status_code,
                endpoint,
                resp.text[:2000],
            )

        return resp.status_code, resp.text

    # ── SOAP envelope helpers ────────────────────────────────────────────

    @staticmethod
    def _wrap_envelope(body_xml: str) -> str:
        return (
            '<?xml version="1.0" encoding="UTF-8"?>'
            '<soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/">'
            "<soapenv:Header/>"
            "<soapenv:Body>"
            f"{body_xml}"
            "</soapenv:Body>"
            "</soapenv:Envelope>"
        )

    # ── Response helpers ─────────────────────────────────────────────────

    @staticmethod
    def extract_body(xml_text: str) -> str:
        """Return the inner content of ``<soap:Body>`` as a string."""
        try:
            root = fromstring(xml_text)
        except Exception:
            return xml_text
        ns = {"soap": SOAP_NS}
        body = root.find("soap:Body", ns)
        if body is None:
            # Try without namespace prefix
            for child in root:
                tag = child.tag.split("}")[-1] if "}" in child.tag else child.tag
                if tag == "Body":
                    body = child
                    break
        if body is None:
            return xml_text
        # Re-serialise body children
        parts = []
        for child in body:
            parts.append(tostring(child, encoding="unicode"))
        return "".join(parts)

    @staticmethod
    def is_fault(xml_text: str) -> bool:
        return "<faultcode>" in xml_text or "<soap:Fault" in xml_text or "<soapenv:Fault" in xml_text

    @staticmethod
    def extract_fault(xml_text: str) -> str:
        """Best-effort extraction of the fault string from a SOAP fault."""
        try:
            root = fromstring(xml_text)
        except Exception:
            return xml_text[:500]
        # Search for faultstring anywhere in the tree
        for elem in root.iter():
            tag = elem.tag.split("}")[-1] if "}" in elem.tag else elem.tag
            if tag == "faultstring" and elem.text:
                return elem.text
        return xml_text[:500]
