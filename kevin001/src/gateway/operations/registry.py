"""Operation registry — maps operation names to their handlers."""

from __future__ import annotations

from .base import OperationHandler


class OperationRegistry:
    """Singleton-ish registry of operation handlers."""

    def __init__(self) -> None:
        self._handlers: dict[str, OperationHandler] = {}

    def register(self, handler: OperationHandler) -> None:
        self._handlers[handler.name] = handler

    def get(self, operation: str) -> OperationHandler:
        try:
            return self._handlers[operation]
        except KeyError:
            raise ValueError(
                f"Unknown operation: {operation!r}.  "
                f"Available: {', '.join(sorted(self._handlers))}"
            )

    @property
    def operations(self) -> list[str]:
        return sorted(self._handlers)


def build_default_registry() -> OperationRegistry:
    """Create a registry pre-loaded with all built-in operation handlers."""
    from .air import (
        AirSearchHandler,
        AirPriceHandler,
        AirBookHandler,
        AirTicketHandler,
        PnrRetrieveHandler,
    )

    reg = OperationRegistry()
    reg.register(AirSearchHandler())
    reg.register(AirPriceHandler())
    reg.register(AirBookHandler())
    reg.register(AirTicketHandler())
    reg.register(PnrRetrieveHandler())
    return reg
