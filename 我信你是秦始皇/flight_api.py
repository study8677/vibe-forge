"""Flight price API client using Amadeus Self-Service API."""

from __future__ import annotations

import time
from dataclasses import dataclass, field
from datetime import datetime

import requests

from airports import resolve_airport


@dataclass
class FlightOffer:
    """A single flight offer with price and segment details."""

    airline: str
    flight_number: str
    departure_time: str  # ISO 8601
    arrival_time: str
    duration: str
    price: float
    currency: str
    stops: int
    source: str = "amadeus"

    @property
    def departure_hour(self) -> int:
        return datetime.fromisoformat(self.departure_time).hour

    @property
    def departure_minute(self) -> int:
        return datetime.fromisoformat(self.departure_time).minute

    def departure_in_range(
        self, start_h: int, start_m: int, end_h: int, end_m: int
    ) -> bool:
        dep = self.departure_hour * 60 + self.departure_minute
        return (start_h * 60 + start_m) <= dep <= (end_h * 60 + end_m)

    def format_time(self, iso: str) -> str:
        return datetime.fromisoformat(iso).strftime("%H:%M")

    def __str__(self) -> str:
        dep = self.format_time(self.departure_time)
        arr = self.format_time(self.arrival_time)
        stops_label = "直飞" if self.stops == 0 else f"{self.stops}次中转"
        return (
            f"{self.airline} {self.flight_number} | "
            f"{dep}-{arr} | {self.duration} | "
            f"{stops_label} | {self.currency} {self.price:.0f}"
        )


class AmadeusClient:
    """Amadeus Flight Offers Search API client with OAuth2."""

    TOKEN_TPL = "https://{env}.api.amadeus.com/v1/security/oauth2/token"
    SEARCH_TPL = "https://{env}.api.amadeus.com/v2/shopping/flight-offers"

    def __init__(self, api_key: str, api_secret: str, env: str = "test"):
        self.api_key = api_key
        self.api_secret = api_secret
        self.env = env
        self._token: str | None = None
        self._token_expires: float = 0

    def _url(self, tpl: str) -> str:
        return tpl.format(env=self.env)

    def _authenticate(self):
        if self._token and time.time() < self._token_expires:
            return
        resp = requests.post(
            self._url(self.TOKEN_TPL),
            data={
                "grant_type": "client_credentials",
                "client_id": self.api_key,
                "client_secret": self.api_secret,
            },
            headers={"Content-Type": "application/x-www-form-urlencoded"},
            timeout=15,
        )
        resp.raise_for_status()
        data = resp.json()
        self._token = data["access_token"]
        self._token_expires = time.time() + data.get("expires_in", 1799) - 60

    def search_flights(
        self,
        origin: str,
        destination: str,
        date: str,
        adults: int = 1,
        currency: str = "CNY",
        max_results: int = 50,
    ) -> list[FlightOffer]:
        """Search flight offers for a given route and date.

        Args:
            origin: Airport name (Chinese) or IATA code.
            destination: Airport name (Chinese) or IATA code.
            date: Departure date YYYY-MM-DD.
            adults: Passenger count.
            currency: Currency code.
            max_results: Max offers to return.

        Returns:
            Sorted list of FlightOffer (cheapest first).
        """
        origin_code = resolve_airport(origin)
        dest_code = resolve_airport(destination)

        self._authenticate()

        resp = requests.get(
            self._url(self.SEARCH_TPL),
            params={
                "originLocationCode": origin_code,
                "destinationLocationCode": dest_code,
                "departureDate": date,
                "adults": adults,
                "currencyCode": currency,
                "max": max_results,
            },
            headers={"Authorization": f"Bearer {self._token}"},
            timeout=30,
        )
        resp.raise_for_status()
        return self._parse(resp.json())

    @staticmethod
    def _parse(data: dict) -> list[FlightOffer]:
        carriers = data.get("dictionaries", {}).get("carriers", {})
        offers: list[FlightOffer] = []

        for item in data.get("data", []):
            price = float(item["price"]["total"])
            currency = item["price"]["currency"]

            for itin in item.get("itineraries", []):
                segments = itin.get("segments", [])
                if not segments:
                    continue
                first, last = segments[0], segments[-1]
                carrier_code = first["carrierCode"]

                offers.append(
                    FlightOffer(
                        airline=carriers.get(carrier_code, carrier_code),
                        flight_number=f"{carrier_code}{first['number']}",
                        departure_time=first["departure"]["at"],
                        arrival_time=last["arrival"]["at"],
                        duration=itin.get("duration", ""),
                        price=price,
                        currency=currency,
                        stops=len(segments) - 1,
                    )
                )

        offers.sort(key=lambda o: o.price)
        return offers
