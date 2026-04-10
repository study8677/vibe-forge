"""FastAPI application — HTTP entry-point for the unified gateway.

Endpoints
---------
POST /gateway             — Execute any operation (generic)
POST /gateway/air/search  — Air search (convenience)
POST /gateway/air/price   — Air pricing (convenience)
POST /gateway/air/book    — Air booking (convenience)
POST /gateway/air/ticket  — Air ticketing (convenience)
POST /gateway/pnr/{loc}   — PNR retrieve (convenience)
GET  /gateway/info        — List available operations & accounts
GET  /health              — Health check
"""

from __future__ import annotations

import logging
from contextlib import asynccontextmanager
from typing import Any

from fastapi import FastAPI, HTTPException, Query
from pydantic import BaseModel, Field

from .config import load_config
from .gateway import TravelportGateway
from .models import GatewayRequest, GatewayResponse, Protocol

logger = logging.getLogger(__name__)

# ── Application state ────────────────────────────────────────────────────────

_gateway: TravelportGateway | None = None


def get_gateway() -> TravelportGateway:
    global _gateway
    if _gateway is None:
        config = load_config()
        logging.basicConfig(level=config.gateway.log_level)
        _gateway = TravelportGateway(config)
    return _gateway


@asynccontextmanager
async def lifespan(app: FastAPI):
    get_gateway()
    logger.info("Travelport Gateway started")
    yield
    logger.info("Travelport Gateway stopped")


app = FastAPI(
    title="Travelport Unified Gateway",
    description="SOAP/XML & JSON multi-account credential routing with standardised I/O",
    version="1.0.0",
    lifespan=lifespan,
)


# ── Request / response wrappers ──────────────────────────────────────────────

class GenericRequest(BaseModel):
    operation: str
    params: dict[str, Any] = Field(default_factory=dict)
    account_id: str | None = None
    protocol: Protocol | None = None
    trace_id: str | None = None


class AirSearchBody(BaseModel):
    legs: list[dict[str, Any]]
    passengers: list[dict[str, Any]] | None = None
    cabin_class: str | None = None
    max_results: int = 200
    direct_flights_only: bool = False
    preferred_carriers: list[str] | None = None
    origin_country: str | None = None
    account_id: str | None = None
    protocol: Protocol | None = None


class AirPriceBody(BaseModel):
    segments: list[dict[str, Any]]
    passengers: list[dict[str, Any]] | None = None
    plating_carrier: str | None = None
    account_id: str | None = None
    protocol: Protocol | None = None


class AirBookBody(BaseModel):
    segments: list[dict[str, Any]]
    travelers: list[dict[str, Any]]
    phone: str
    email: str | None = None
    plating_carrier: str | None = None
    account_id: str | None = None
    protocol: Protocol | None = None


class AirTicketBody(BaseModel):
    pnr_locator: str
    plating_carrier: str | None = None
    commission_percent: float | None = None
    account_id: str | None = None
    protocol: Protocol | None = None


# ── Endpoints ────────────────────────────────────────────────────────────────

@app.get("/health")
async def health():
    return {"status": "ok"}


@app.get("/gateway/info")
async def info():
    gw = get_gateway()
    return {
        "operations": gw.available_operations(),
        "accounts": gw.available_accounts(),
        "schema_version": gw.config.schema_version,
    }


@app.post("/gateway", response_model=GatewayResponse)
async def generic_execute(body: GenericRequest):
    gw = get_gateway()
    req = GatewayRequest(
        operation=body.operation,
        params=body.params,
        account_id=body.account_id,
        protocol=body.protocol,
        trace_id=body.trace_id,
    )
    return await gw.execute(req)


@app.post("/gateway/air/search", response_model=GatewayResponse)
async def air_search(body: AirSearchBody):
    gw = get_gateway()
    params = body.model_dump(exclude={"account_id", "protocol"}, exclude_none=True)
    return await gw.air_search(params, account_id=body.account_id, protocol=body.protocol)


@app.post("/gateway/air/price", response_model=GatewayResponse)
async def air_price(body: AirPriceBody):
    gw = get_gateway()
    params = body.model_dump(exclude={"account_id", "protocol"}, exclude_none=True)
    return await gw.air_price(params, account_id=body.account_id, protocol=body.protocol)


@app.post("/gateway/air/book", response_model=GatewayResponse)
async def air_book(body: AirBookBody):
    gw = get_gateway()
    params = body.model_dump(exclude={"account_id", "protocol"}, exclude_none=True)
    return await gw.air_book(params, account_id=body.account_id, protocol=body.protocol)


@app.post("/gateway/air/ticket", response_model=GatewayResponse)
async def air_ticket(body: AirTicketBody):
    gw = get_gateway()
    params = body.model_dump(exclude={"account_id", "protocol"}, exclude_none=True)
    return await gw.air_ticket(params, account_id=body.account_id, protocol=body.protocol)


@app.post("/gateway/pnr/{locator}", response_model=GatewayResponse)
async def pnr_retrieve(
    locator: str,
    provider_code: str | None = Query(None),
    account_id: str | None = Query(None),
    protocol: Protocol | None = Query(None),
):
    gw = get_gateway()
    params: dict[str, Any] = {"pnr_locator": locator}
    if provider_code:
        params["provider_code"] = provider_code
    return await gw.pnr_retrieve(params, account_id=account_id, protocol=protocol)
