"""Multi-account credential manager.

Holds all configured accounts and provides lookup by ID.
For JSON/OAuth2 accounts it also manages token lifecycle.
"""

from __future__ import annotations

import logging
import time
from typing import Any

import httpx

from .config import AccountConfig, GatewayConfig
from .models import Protocol

logger = logging.getLogger(__name__)


class OAuthToken:
    """Cached OAuth2 bearer token with expiry tracking."""

    def __init__(self, access_token: str, expires_at: float):
        self.access_token = access_token
        self.expires_at = expires_at

    @property
    def expired(self) -> bool:
        return time.time() >= self.expires_at - 30  # 30s safety margin


class CredentialManager:
    """Resolve account configs and manage OAuth2 tokens for JSON accounts."""

    def __init__(self, config: GatewayConfig):
        self._accounts: dict[str, AccountConfig] = {a.id: a for a in config.accounts}
        self._tokens: dict[str, OAuthToken] = {}

    @property
    def account_ids(self) -> list[str]:
        return list(self._accounts)

    def get_account(self, account_id: str) -> AccountConfig:
        try:
            return self._accounts[account_id]
        except KeyError:
            raise ValueError(f"Unknown account: {account_id}")

    # ── SOAP helpers ─────────────────────────────────────────────────────

    def soap_auth_header(self, account_id: str) -> dict[str, str]:
        acct = self.get_account(account_id)
        cred = acct.credentials
        if cred.username is None or cred.password is None:
            raise ValueError(f"Account {account_id} missing SOAP username/password")
        # Travelport UAPI uses HTTP Basic Auth
        import base64
        token = base64.b64encode(f"{cred.username}:{cred.password}".encode()).decode()
        return {"Authorization": f"Basic {token}"}

    def soap_endpoint(self, account_id: str, service: str) -> str:
        acct = self.get_account(account_id)
        try:
            return acct.endpoints[service]
        except KeyError:
            raise ValueError(f"Account {account_id} has no endpoint for service {service}")

    # ── JSON / OAuth2 helpers ────────────────────────────────────────────

    async def json_bearer_token(self, account_id: str) -> str:
        """Return a valid bearer token, refreshing if needed."""
        cached = self._tokens.get(account_id)
        if cached and not cached.expired:
            return cached.access_token

        acct = self.get_account(account_id)
        cred = acct.credentials
        if not cred.client_id or not cred.client_secret or not cred.token_endpoint:
            raise ValueError(f"Account {account_id} missing OAuth2 credentials")

        logger.info("Refreshing OAuth2 token for account %s", account_id)
        async with httpx.AsyncClient() as client:
            resp = await client.post(
                cred.token_endpoint,
                data={
                    "grant_type": "client_credentials",
                    "client_id": cred.client_id,
                    "client_secret": cred.client_secret,
                },
                headers={"Content-Type": "application/x-www-form-urlencoded"},
            )
            resp.raise_for_status()
            body = resp.json()

        access_token = body["access_token"]
        expires_in = int(body.get("expires_in", 3600))
        self._tokens[account_id] = OAuthToken(access_token, time.time() + expires_in)
        return access_token

    async def json_auth_header(self, account_id: str) -> dict[str, str]:
        token = await self.json_bearer_token(account_id)
        return {"Authorization": f"Bearer {token}"}

    def json_base_url(self, account_id: str) -> str:
        acct = self.get_account(account_id)
        url = acct.endpoints.get("base_url", "")
        if not url:
            raise ValueError(f"Account {account_id} has no base_url endpoint")
        return url

    # ── Generic helpers ──────────────────────────────────────────────────

    def protocol_for(self, account_id: str) -> Protocol:
        return self.get_account(account_id).protocol

    def target_branch(self, account_id: str) -> str:
        acct = self.get_account(account_id)
        tb = acct.credentials.target_branch
        if not tb:
            raise ValueError(f"Account {account_id} missing target_branch")
        return tb

    def pcc(self, account_id: str) -> str:
        return self.get_account(account_id).pcc

    def provider(self, account_id: str) -> str:
        return self.get_account(account_id).provider
