"""JSON / REST protocol adapter for Travelport+ API.

Responsibilities:
 - Obtain and cache OAuth2 bearer tokens via the credential manager
 - Send JSON requests to Travelport+ REST endpoints
 - Return parsed JSON (or raw text on error)
"""

from __future__ import annotations

import logging
from typing import Any

import httpx

from ..credentials import CredentialManager
from .base import ProtocolAdapter

logger = logging.getLogger(__name__)


class RestAdapter(ProtocolAdapter):
    """Send JSON/REST requests to Travelport+ endpoints."""

    def __init__(self, credential_manager: CredentialManager, timeout: int = 30):
        super().__init__(credential_manager, timeout)

    async def send(
        self,
        account_id: str,
        service: str,
        payload: str | dict,
        *,
        extra_headers: dict[str, str] | None = None,
        method: str = "POST",
        path: str = "",
    ) -> tuple[int, str | dict]:
        """
        Parameters
        ----------
        account_id : Account to use for credentials + base URL.
        service    : Ignored for REST (kept for interface parity).
        payload    : JSON-serialisable dict (or raw JSON string).
        method     : HTTP method (default POST).
        path       : Path appended to the account's ``base_url``.

        Returns
        -------
        (http_status, response_body)  — body is a dict on success, str on error.
        """
        base_url = self.creds.json_base_url(account_id)
        url = f"{base_url.rstrip('/')}/{path.lstrip('/')}" if path else base_url
        auth_headers = await self.creds.json_auth_header(account_id)

        headers = {
            "Content-Type": "application/json",
            "Accept": "application/json",
            **auth_headers,
        }
        if extra_headers:
            headers.update(extra_headers)

        logger.debug("REST %s → %s", method, url)

        async with httpx.AsyncClient(timeout=self.timeout) as client:
            if method.upper() == "GET":
                resp = await client.get(url, headers=headers, params=payload if isinstance(payload, dict) else None)
            else:
                resp = await client.request(
                    method,
                    url,
                    json=payload if isinstance(payload, dict) else None,
                    content=payload if isinstance(payload, str) else None,
                    headers=headers,
                )

        if resp.status_code >= 400:
            logger.error("REST error: HTTP %d from %s\n%s", resp.status_code, url, resp.text[:2000])
            return resp.status_code, resp.text

        try:
            return resp.status_code, resp.json()
        except Exception:
            return resp.status_code, resp.text
