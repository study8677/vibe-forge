"""Core gateway orchestrator.

Request flow:
  1. Receive standardised ``GatewayRequest``
  2. Route to account (via ``OperationRouter``)
  3. Look up operation handler (via ``OperationRegistry``)
  4. Build protocol-specific payload (SOAP or REST)
  5. Send via the appropriate ``ProtocolAdapter``
  6. Parse response back to standardised form
  7. Return ``GatewayResponse``
"""

from __future__ import annotations

import logging
import time
from typing import Any

from .config import GatewayConfig, load_config
from .credentials import CredentialManager
from .models import GatewayRequest, GatewayResponse, Protocol
from .operations.registry import OperationRegistry, build_default_registry
from .protocols.base import ProtocolAdapter
from .protocols.rest import RestAdapter
from .protocols.soap import SoapAdapter
from .router import OperationRouter

logger = logging.getLogger(__name__)


class TravelportGateway:
    """Unified Travelport gateway — the single entry-point for all callers."""

    def __init__(
        self,
        config: GatewayConfig | None = None,
        registry: OperationRegistry | None = None,
    ):
        self.config = config or load_config()
        self.creds = CredentialManager(self.config)
        self.router = OperationRouter(self.config)
        self.registry = registry or build_default_registry()

        timeout = self.config.gateway.default_timeout
        self._adapters: dict[Protocol, ProtocolAdapter] = {
            Protocol.SOAP: SoapAdapter(self.creds, timeout=timeout),
            Protocol.JSON: RestAdapter(self.creds, timeout=timeout),
        }

    # ── main entry-point ─────────────────────────────────────────────────

    async def execute(self, request: GatewayRequest) -> GatewayResponse:
        t0 = time.monotonic()
        operation = request.operation
        params = request.params

        try:
            # 1. Resolve account
            account_id = request.account_id or self.router.resolve(operation, params)
            account = self.creds.get_account(account_id)

            # 2. Determine protocol
            protocol = request.protocol or account.protocol

            # 3. Get operation handler
            handler = self.registry.get(operation)

            # 4. Build protocol-specific payload
            if protocol == Protocol.SOAP:
                req_info = handler.build_soap(params, account, self.config.schema_version)
                service = req_info.service
                payload: str | dict = req_info.xml_body
            else:
                req_info_rest = handler.build_rest(params, account)
                service = ""
                payload = req_info_rest.body or {}

            # 5. Send
            adapter = self._adapters[protocol]
            if protocol == Protocol.SOAP:
                status, raw = await adapter.send(account_id, service, payload)
            else:
                status, raw = await adapter.send(
                    account_id,
                    service,
                    payload,
                    path=req_info_rest.path,
                    method=req_info_rest.method,
                )

            # 6. Parse response
            if status >= 400:
                error_detail = raw if isinstance(raw, str) else str(raw)
                if protocol == Protocol.SOAP and isinstance(raw, str) and SoapAdapter.is_fault(raw):
                    error_detail = SoapAdapter.extract_fault(raw)
                return GatewayResponse(
                    request_id=request.request_id,
                    success=False,
                    operation=operation,
                    protocol_used=protocol,
                    account_used=account_id,
                    errors=[f"HTTP {status}: {error_detail[:500]}"],
                    raw_response=raw if isinstance(raw, str) else None,
                    elapsed_ms=_elapsed(t0),
                )

            if protocol == Protocol.SOAP:
                body_xml = SoapAdapter.extract_body(raw) if isinstance(raw, str) else str(raw)
                data = handler.parse_soap(body_xml)
            else:
                data = handler.parse_rest(raw if isinstance(raw, dict) else {})

            return GatewayResponse(
                request_id=request.request_id,
                success=True,
                operation=operation,
                protocol_used=protocol,
                account_used=account_id,
                data=data,
                raw_response=raw if isinstance(raw, str) else None,
                elapsed_ms=_elapsed(t0),
            )

        except Exception as exc:
            logger.exception("Gateway error for %s", operation)
            return GatewayResponse(
                request_id=request.request_id,
                success=False,
                operation=operation,
                protocol_used=request.protocol or Protocol.SOAP,
                account_used=request.account_id or "unknown",
                errors=[str(exc)],
                elapsed_ms=_elapsed(t0),
            )

    # ── convenience wrappers ─────────────────────────────────────────────

    async def air_search(self, params: dict[str, Any], **kw: Any) -> GatewayResponse:
        return await self.execute(GatewayRequest(operation="air_search", params=params, **kw))

    async def air_price(self, params: dict[str, Any], **kw: Any) -> GatewayResponse:
        return await self.execute(GatewayRequest(operation="air_price", params=params, **kw))

    async def air_book(self, params: dict[str, Any], **kw: Any) -> GatewayResponse:
        return await self.execute(GatewayRequest(operation="air_book", params=params, **kw))

    async def air_ticket(self, params: dict[str, Any], **kw: Any) -> GatewayResponse:
        return await self.execute(GatewayRequest(operation="air_ticket", params=params, **kw))

    async def pnr_retrieve(self, params: dict[str, Any], **kw: Any) -> GatewayResponse:
        return await self.execute(GatewayRequest(operation="pnr_retrieve", params=params, **kw))

    # ── introspection ────────────────────────────────────────────────────

    def available_operations(self) -> list[str]:
        return self.registry.operations

    def available_accounts(self) -> list[dict[str, str]]:
        return [
            {"id": a.id, "name": a.name, "protocol": a.protocol.value, "pcc": a.pcc}
            for a in self.config.accounts
        ]


def _elapsed(t0: float) -> float:
    return round((time.monotonic() - t0) * 1000, 2)
