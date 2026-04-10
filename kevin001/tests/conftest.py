"""Shared fixtures for gateway tests."""

from __future__ import annotations

import textwrap
from pathlib import Path

import pytest
import yaml

from gateway.config import GatewayConfig, load_config
from gateway.credentials import CredentialManager
from gateway.gateway import TravelportGateway
from gateway.operations.registry import build_default_registry
from gateway.router import OperationRouter


@pytest.fixture()
def sample_config(tmp_path: Path) -> GatewayConfig:
    """Write a minimal config YAML and load it."""
    cfg = {
        "gateway": {"name": "test-gw", "version": "0.1.0", "default_timeout": 5, "log_level": "DEBUG"},
        "schema_version": "v52_0",
        "accounts": [
            {
                "id": "soap_a",
                "name": "SOAP Account A",
                "protocol": "soap",
                "pcc": "AAA",
                "provider": "1G",
                "credentials": {
                    "target_branch": "P001",
                    "username": "user_a",
                    "password": "pass_a",
                },
                "endpoints": {
                    "AirService": "https://mock.travelport.test/AirService",
                    "UniversalRecordService": "https://mock.travelport.test/UniversalRecordService",
                },
            },
            {
                "id": "soap_b",
                "name": "SOAP Account B",
                "protocol": "soap",
                "pcc": "BBB",
                "provider": "1G",
                "credentials": {
                    "target_branch": "P002",
                    "username": "user_b",
                    "password": "pass_b",
                },
                "endpoints": {
                    "AirService": "https://mock-intl.travelport.test/AirService",
                    "UniversalRecordService": "https://mock-intl.travelport.test/UniversalRecordService",
                },
            },
            {
                "id": "json_c",
                "name": "JSON Account C",
                "protocol": "json",
                "pcc": "CCC",
                "provider": "TP",
                "credentials": {
                    "client_id": "cid",
                    "client_secret": "csecret",
                    "token_endpoint": "https://mock.oauth.test/token",
                },
                "endpoints": {
                    "base_url": "https://mock.travelport.test/api",
                },
            },
        ],
        "routing": {
            "default_account": "soap_a",
            "rules": [
                {"operation": "air_search", "conditions": [{"field": "origin_country", "operator": "eq", "value": "CN"}], "account_id": "soap_a"},
                {"operation": "air_search", "conditions": [{"field": "origin_country", "operator": "neq", "value": "CN"}], "account_id": "soap_b"},
                {"operation": "air_price", "account_id": "soap_a"},
                {"operation": "air_book", "account_id": "soap_a"},
                {"operation": "air_ticket", "account_id": "soap_a"},
                {"operation": "pnr_retrieve", "account_id": "soap_a"},
            ],
        },
    }
    cfg_path = tmp_path / "gateway.yaml"
    cfg_path.write_text(yaml.dump(cfg), encoding="utf-8")
    return load_config(cfg_path)


@pytest.fixture()
def cred_manager(sample_config: GatewayConfig) -> CredentialManager:
    return CredentialManager(sample_config)


@pytest.fixture()
def router(sample_config: GatewayConfig) -> OperationRouter:
    return OperationRouter(sample_config)
