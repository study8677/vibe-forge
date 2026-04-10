"""Abstract protocol adapter interface."""

from __future__ import annotations

from abc import ABC, abstractmethod
from typing import Any

from ..credentials import CredentialManager


class ProtocolAdapter(ABC):
    """Send a pre-built payload over a specific transport and return the raw response."""

    def __init__(self, credential_manager: CredentialManager, timeout: int = 30):
        self.creds = credential_manager
        self.timeout = timeout

    @abstractmethod
    async def send(
        self,
        account_id: str,
        service: str,
        payload: str | dict,
        *,
        extra_headers: dict[str, str] | None = None,
    ) -> tuple[int, str | dict]:
        """Send payload and return ``(status_code, raw_response)``."""
        ...
