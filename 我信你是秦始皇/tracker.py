"""Price history tracking and drop detection."""

from __future__ import annotations

import json
from dataclasses import dataclass
from datetime import datetime
from pathlib import Path

from flight_api import FlightOffer

HISTORY_FILE = Path(__file__).parent / "price_history.json"


@dataclass
class PriceDrop:
    """A detected price drop event."""

    origin: str
    destination: str
    date: str
    previous_price: float
    current_price: float
    drop_amount: float
    drop_percent: float
    currency: str
    flight: FlightOffer


class PriceTracker:
    """Track flight prices over time and detect significant drops."""

    def __init__(self, history_file: str | Path = HISTORY_FILE):
        self.history_file = Path(history_file)
        self.history: dict[str, list[dict]] = self._load()

    def _load(self) -> dict:
        if self.history_file.exists():
            with open(self.history_file, "r", encoding="utf-8") as f:
                return json.load(f)
        return {}

    def _save(self):
        with open(self.history_file, "w", encoding="utf-8") as f:
            json.dump(self.history, f, ensure_ascii=False, indent=2)

    @staticmethod
    def _key(origin: str, destination: str, date: str) -> str:
        return f"{origin}-{destination}-{date}"

    def check_and_update(
        self,
        origin: str,
        destination: str,
        date: str,
        offers: list[FlightOffer],
        threshold: float = 50,
    ) -> PriceDrop | None:
        """Record current prices and return PriceDrop if threshold exceeded.

        Compares the current cheapest offer against the historical minimum.
        """
        if not offers:
            return None

        cheapest = min(offers, key=lambda o: o.price)
        key = self._key(origin, destination, date)

        snapshot = {
            "timestamp": datetime.now().isoformat(),
            "lowest_price": cheapest.price,
            "currency": cheapest.currency,
            "flight_number": cheapest.flight_number,
            "airline": cheapest.airline,
        }

        previous = self.history.get(key, [])
        drop = None

        if previous:
            prev_min = min(r["lowest_price"] for r in previous)
            diff = prev_min - cheapest.price
            if diff >= threshold:
                drop = PriceDrop(
                    origin=origin,
                    destination=destination,
                    date=date,
                    previous_price=prev_min,
                    current_price=cheapest.price,
                    drop_amount=diff,
                    drop_percent=diff / prev_min * 100,
                    currency=cheapest.currency,
                    flight=cheapest,
                )

        self.history.setdefault(key, []).append(snapshot)

        # Keep last 200 snapshots per route
        if len(self.history[key]) > 200:
            self.history[key] = self.history[key][-200:]

        self._save()
        return drop

    def get_history(self, origin: str, destination: str, date: str) -> list[dict]:
        return self.history.get(self._key(origin, destination, date), [])
