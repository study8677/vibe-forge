"""Operation-based routing engine.

Evaluates routing rules **top-down** against the incoming request params.
First matching rule wins; if nothing matches the ``default_account`` is used.
"""

from __future__ import annotations

import logging
from typing import Any

from .config import GatewayConfig, RoutingCondition, RoutingRule

logger = logging.getLogger(__name__)


class OperationRouter:
    """Route an operation + params to the appropriate account ID."""

    def __init__(self, config: GatewayConfig):
        self._rules = config.routing.rules
        self._default = config.routing.default_account

    def resolve(self, operation: str, params: dict[str, Any]) -> str:
        """Return the ``account_id`` that should serve this request."""
        for rule in self._rules:
            if rule.operation != operation:
                continue
            if self._match_conditions(rule.conditions, params):
                logger.debug(
                    "Routing %s → %s (rule matched)", operation, rule.account_id
                )
                return rule.account_id

        logger.debug("Routing %s → %s (default)", operation, self._default)
        return self._default

    # ── Condition evaluation ─────────────────────────────────────────────

    @staticmethod
    def _match_conditions(conditions: list[RoutingCondition], params: dict[str, Any]) -> bool:
        """All conditions must pass (AND logic)."""
        if not conditions:
            return True
        return all(
            OperationRouter._eval_condition(c, params) for c in conditions
        )

    @staticmethod
    def _eval_condition(cond: RoutingCondition, params: dict[str, Any]) -> bool:
        actual = params.get(cond.field)
        if actual is None:
            return cond.operator in ("neq", "not_in")

        expected = cond.value
        op = cond.operator

        if op == "eq":
            return str(actual) == str(expected)
        if op == "neq":
            return str(actual) != str(expected)
        if op == "in":
            if isinstance(expected, list):
                return str(actual) in [str(v) for v in expected]
            return str(actual) in str(expected)
        if op == "not_in":
            if isinstance(expected, list):
                return str(actual) not in [str(v) for v in expected]
            return str(actual) not in str(expected)

        logger.warning("Unknown operator %r in routing condition", op)
        return False
