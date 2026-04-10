"""Standardised request / response models.

These Pydantic models define the **protocol-agnostic** contract that callers
use to talk to the gateway.  Regardless of whether the underlying call goes
over SOAP/XML or JSON/REST, callers always send and receive these structures.
"""

from __future__ import annotations

import uuid
from datetime import date, datetime
from enum import Enum
from typing import Any

from pydantic import BaseModel, Field


# ── Enums ────────────────────────────────────────────────────────────────────

class Protocol(str, Enum):
    SOAP = "soap"
    JSON = "json"


class PassengerType(str, Enum):
    ADT = "ADT"   # Adult
    CHD = "CHD"   # Child
    INF = "INF"   # Infant
    CNN = "CNN"   # Child (alternate)
    INS = "INS"   # Infant with seat


class CabinClass(str, Enum):
    ECONOMY = "Economy"
    PREMIUM_ECONOMY = "PremiumEconomy"
    BUSINESS = "Business"
    FIRST = "First"


# ── Common building blocks ───────────────────────────────────────────────────

class Passenger(BaseModel):
    type: PassengerType = PassengerType.ADT
    count: int = 1


class FlightLeg(BaseModel):
    origin: str = Field(..., min_length=3, max_length=3, description="IATA airport/city code")
    destination: str = Field(..., min_length=3, max_length=3)
    departure_date: date
    cabin_class: CabinClass | None = None


class FlightSegment(BaseModel):
    carrier: str
    flight_number: str
    origin: str
    destination: str
    departure: datetime
    arrival: datetime
    cabin_class: CabinClass | None = None
    booking_class: str | None = None
    fare_basis: str | None = None
    equipment: str | None = None
    duration_minutes: int | None = None
    stops: int = 0


class PriceBreakdown(BaseModel):
    base_fare: float
    taxes: float
    total: float
    currency: str = "USD"
    passenger_type: PassengerType = PassengerType.ADT


class ContactInfo(BaseModel):
    first_name: str
    last_name: str
    phone: str | None = None
    email: str | None = None


class TravelerInfo(BaseModel):
    passenger_type: PassengerType = PassengerType.ADT
    contact: ContactInfo
    date_of_birth: date | None = None
    gender: str | None = None
    passport_number: str | None = None
    passport_country: str | None = None
    passport_expiry: date | None = None


# ── Gateway request ──────────────────────────────────────────────────────────

class GatewayRequest(BaseModel):
    """Unified gateway request — protocol-agnostic."""
    request_id: str = Field(default_factory=lambda: uuid.uuid4().hex[:16])
    operation: str = Field(..., description="e.g. air_search, air_price, air_book, air_ticket, pnr_retrieve")
    params: dict[str, Any] = Field(default_factory=dict)
    account_id: str | None = Field(None, description="Force a specific account (bypasses routing)")
    protocol: Protocol | None = Field(None, description="Force a specific protocol")
    trace_id: str | None = None


# ── Operation-specific param models ─────────────────────────────────────────

class AirSearchParams(BaseModel):
    legs: list[FlightLeg]
    passengers: list[Passenger] = Field(default_factory=lambda: [Passenger()])
    cabin_class: CabinClass | None = None
    max_results: int = 200
    direct_flights_only: bool = False
    preferred_carriers: list[str] | None = None
    origin_country: str | None = None


class AirPriceParams(BaseModel):
    segments: list[FlightSegment]
    passengers: list[Passenger] = Field(default_factory=lambda: [Passenger()])
    plating_carrier: str | None = None


class AirBookParams(BaseModel):
    segments: list[FlightSegment]
    travelers: list[TravelerInfo]
    phone: str
    email: str | None = None
    plating_carrier: str | None = None
    ticket_time_limit: datetime | None = None


class AirTicketParams(BaseModel):
    pnr_locator: str
    plating_carrier: str | None = None
    commission_percent: float | None = None


class PnrRetrieveParams(BaseModel):
    pnr_locator: str
    provider_code: str | None = None


# ── Operation-specific result models ────────────────────────────────────────

class AirSearchResultItem(BaseModel):
    itinerary_id: str | None = None
    segments: list[FlightSegment]
    prices: list[PriceBreakdown]
    total_duration_minutes: int | None = None
    stops: int = 0


class AirSearchResult(BaseModel):
    items: list[AirSearchResultItem] = Field(default_factory=list)
    total_count: int = 0


class AirPriceResult(BaseModel):
    prices: list[PriceBreakdown]
    fare_rules: list[dict[str, Any]] = Field(default_factory=list)
    ticketing_deadline: datetime | None = None


class AirBookResult(BaseModel):
    pnr_locator: str
    provider_locator: str | None = None
    status: str = "Confirmed"
    segments: list[FlightSegment] = Field(default_factory=list)
    travelers: list[dict[str, Any]] = Field(default_factory=list)


class AirTicketResult(BaseModel):
    pnr_locator: str
    ticket_numbers: list[str] = Field(default_factory=list)
    status: str = "Ticketed"


class PnrRetrieveResult(BaseModel):
    pnr_locator: str
    status: str
    segments: list[FlightSegment] = Field(default_factory=list)
    travelers: list[dict[str, Any]] = Field(default_factory=list)
    tickets: list[str] = Field(default_factory=list)
    raw: dict[str, Any] = Field(default_factory=dict)


# ── Gateway response ────────────────────────────────────────────────────────

class GatewayResponse(BaseModel):
    """Unified gateway response — protocol-agnostic."""
    request_id: str
    success: bool
    operation: str
    protocol_used: Protocol
    account_used: str
    data: dict[str, Any] = Field(default_factory=dict)
    errors: list[str] = Field(default_factory=list)
    raw_response: str | None = Field(None, description="Original protocol response for debugging")
    elapsed_ms: float | None = None
