"""Tests for routing, credential management, operation builders, and SOAP parsing."""

from __future__ import annotations

import textwrap

import pytest

from gateway.config import GatewayConfig
from gateway.credentials import CredentialManager
from gateway.models import Protocol
from gateway.operations.air import (
    AirBookHandler,
    AirPriceHandler,
    AirSearchHandler,
    AirTicketHandler,
    PnrRetrieveHandler,
    _money,
)
from gateway.operations.registry import build_default_registry
from gateway.router import OperationRouter


# ── Routing tests ────────────────────────────────────────────────────────────

class TestOperationRouter:
    def test_route_domestic(self, router: OperationRouter):
        acct = router.resolve("air_search", {"origin_country": "CN"})
        assert acct == "soap_a"

    def test_route_international(self, router: OperationRouter):
        acct = router.resolve("air_search", {"origin_country": "US"})
        assert acct == "soap_b"

    def test_route_no_country_falls_through(self, router: OperationRouter):
        # No origin_country → first "neq CN" rule matches (since None != CN)
        acct = router.resolve("air_search", {})
        assert acct == "soap_b"

    def test_route_default(self, router: OperationRouter):
        acct = router.resolve("some_unknown_op", {})
        assert acct == "soap_a"  # default_account

    def test_route_air_price(self, router: OperationRouter):
        acct = router.resolve("air_price", {})
        assert acct == "soap_a"


# ── Credential manager tests ────────────────────────────────────────────────

class TestCredentialManager:
    def test_get_account(self, cred_manager: CredentialManager):
        acct = cred_manager.get_account("soap_a")
        assert acct.pcc == "AAA"
        assert acct.protocol == Protocol.SOAP

    def test_unknown_account(self, cred_manager: CredentialManager):
        with pytest.raises(ValueError, match="Unknown account"):
            cred_manager.get_account("nonexistent")

    def test_soap_auth_header(self, cred_manager: CredentialManager):
        header = cred_manager.soap_auth_header("soap_a")
        assert "Authorization" in header
        assert header["Authorization"].startswith("Basic ")

    def test_soap_endpoint(self, cred_manager: CredentialManager):
        url = cred_manager.soap_endpoint("soap_a", "AirService")
        assert "AirService" in url

    def test_protocol_for(self, cred_manager: CredentialManager):
        assert cred_manager.protocol_for("soap_a") == Protocol.SOAP
        assert cred_manager.protocol_for("json_c") == Protocol.JSON


# ── Operation registry tests ────────────────────────────────────────────────

class TestOperationRegistry:
    def test_all_operations_registered(self):
        reg = build_default_registry()
        ops = reg.operations
        assert "air_search" in ops
        assert "air_price" in ops
        assert "air_book" in ops
        assert "air_ticket" in ops
        assert "pnr_retrieve" in ops

    def test_unknown_operation(self):
        reg = build_default_registry()
        with pytest.raises(ValueError, match="Unknown operation"):
            reg.get("nonexistent")


# ── SOAP builder tests ──────────────────────────────────────────────────────

class TestAirSearchSoapBuilder:
    def test_builds_valid_xml(self, cred_manager: CredentialManager):
        handler = AirSearchHandler()
        account = cred_manager.get_account("soap_a")
        params = {
            "legs": [
                {"origin": "PEK", "destination": "SHA", "departure_date": "2025-06-15"},
            ],
            "passengers": [{"type": "ADT", "count": 1}],
            "max_results": 50,
        }
        result = handler.build_soap(params, account, "v52_0")
        assert result.service == "AirService"
        assert "LowFareSearchReq" in result.xml_body
        assert "PEK" in result.xml_body
        assert "SHA" in result.xml_body
        assert 'TargetBranch="P001"' in result.xml_body
        assert 'MaxSolutions="50"' in result.xml_body

    def test_multi_leg(self, cred_manager: CredentialManager):
        handler = AirSearchHandler()
        account = cred_manager.get_account("soap_a")
        params = {
            "legs": [
                {"origin": "PEK", "destination": "SHA", "departure_date": "2025-06-15"},
                {"origin": "SHA", "destination": "PEK", "departure_date": "2025-06-20"},
            ],
        }
        result = handler.build_soap(params, account, "v52_0")
        assert result.xml_body.count("SearchAirLeg") == 4  # 2 open + 2 close

    def test_preferred_carriers(self, cred_manager: CredentialManager):
        handler = AirSearchHandler()
        account = cred_manager.get_account("soap_a")
        params = {
            "legs": [{"origin": "PEK", "destination": "SHA", "departure_date": "2025-06-15"}],
            "preferred_carriers": ["CA", "MU"],
        }
        result = handler.build_soap(params, account, "v52_0")
        assert "CA" in result.xml_body
        assert "MU" in result.xml_body


class TestAirPriceSoapBuilder:
    def test_builds_valid_xml(self, cred_manager: CredentialManager):
        handler = AirPriceHandler()
        account = cred_manager.get_account("soap_a")
        params = {
            "segments": [{
                "carrier": "CA",
                "flight_number": "1234",
                "origin": "PEK",
                "destination": "SHA",
                "departure": "2025-06-15T08:00:00",
                "arrival": "2025-06-15T10:30:00",
            }],
        }
        result = handler.build_soap(params, account, "v52_0")
        assert "AirPriceReq" in result.xml_body
        assert "CA" in result.xml_body


class TestAirBookSoapBuilder:
    def test_builds_valid_xml(self, cred_manager: CredentialManager):
        handler = AirBookHandler()
        account = cred_manager.get_account("soap_a")
        params = {
            "segments": [{
                "carrier": "CA",
                "flight_number": "1234",
                "origin": "PEK",
                "destination": "SHA",
                "departure": "2025-06-15T08:00:00",
                "arrival": "2025-06-15T10:30:00",
            }],
            "travelers": [{
                "passenger_type": "ADT",
                "contact": {"first_name": "JOHN", "last_name": "DOE"},
            }],
            "phone": "13800138000",
        }
        result = handler.build_soap(params, account, "v52_0")
        assert "AirCreateReservationReq" in result.xml_body
        assert "JOHN" in result.xml_body
        assert "DOE" in result.xml_body


# ── SOAP response parser tests ──────────────────────────────────────────────

class TestAirSearchSoapParser:
    SAMPLE_RESPONSE = textwrap.dedent("""\
        <air:LowFareSearchRsp xmlns:air="http://www.travelport.com/schema/air_v52_0"
                              xmlns:com="http://www.travelport.com/schema/common_v52_0">
          <air:AirPricingSolution Key="sol_1" TotalPrice="CNY1520.00">
            <air:AirSegment Carrier="CA" FlightNumber="1234"
                            Origin="PEK" Destination="SHA"
                            DepartureTime="2025-06-15T08:00:00.000+08:00"
                            ArrivalTime="2025-06-15T10:30:00.000+08:00"
                            Equipment="738" NumberOfStops="0"/>
            <air:AirPricingInfo BasePrice="CNY1200.00" Taxes="CNY320.00" TotalPrice="CNY1520.00">
              <air:PassengerType Code="ADT"/>
            </air:AirPricingInfo>
          </air:AirPricingSolution>
          <air:AirPricingSolution Key="sol_2" TotalPrice="CNY980.00">
            <air:AirSegment Carrier="MU" FlightNumber="5678"
                            Origin="PEK" Destination="SHA"
                            DepartureTime="2025-06-15T14:00:00.000+08:00"
                            ArrivalTime="2025-06-15T16:20:00.000+08:00"
                            Equipment="320" NumberOfStops="0"/>
            <air:AirPricingInfo BasePrice="CNY700.00" Taxes="CNY280.00" TotalPrice="CNY980.00">
              <air:PassengerType Code="ADT"/>
            </air:AirPricingInfo>
          </air:AirPricingSolution>
        </air:LowFareSearchRsp>
    """)

    def test_parse_two_solutions(self):
        handler = AirSearchHandler()
        result = handler.parse_soap(self.SAMPLE_RESPONSE)
        assert result["total_count"] == 2
        assert len(result["items"]) == 2

        item0 = result["items"][0]
        assert item0["segments"][0]["carrier"] == "CA"
        assert item0["segments"][0]["origin"] == "PEK"
        assert item0["prices"][0]["total"] == 1520.0
        assert item0["prices"][0]["currency"] == "CNY"

        item1 = result["items"][1]
        assert item1["segments"][0]["carrier"] == "MU"
        assert item1["prices"][0]["total"] == 980.0


class TestAirPriceSoapParser:
    SAMPLE = textwrap.dedent("""\
        <air:AirPriceRsp xmlns:air="http://www.travelport.com/schema/air_v52_0"
                         xmlns:com="http://www.travelport.com/schema/common_v52_0">
          <air:AirPricingInfo BasePrice="USD450.00" Taxes="USD120.00" TotalPrice="USD570.00">
            <air:PassengerType Code="ADT"/>
          </air:AirPricingInfo>
        </air:AirPriceRsp>
    """)

    def test_parse_price(self):
        handler = AirPriceHandler()
        result = handler.parse_soap(self.SAMPLE)
        assert len(result["prices"]) == 1
        assert result["prices"][0]["base_fare"] == 450.0
        assert result["prices"][0]["taxes"] == 120.0
        assert result["prices"][0]["total"] == 570.0


class TestPnrRetrieveSoapParser:
    SAMPLE = textwrap.dedent("""\
        <univ:UniversalRecordRetrieveRsp xmlns:univ="http://www.travelport.com/schema/universal_v52_0"
                                          xmlns:air="http://www.travelport.com/schema/air_v52_0">
          <univ:UniversalRecord LocatorCode="ABC123" Status="Active">
            <air:AirSegment Carrier="CA" FlightNumber="1234"
                            Origin="PEK" Destination="SHA"
                            DepartureTime="2025-06-15T08:00:00" ArrivalTime="2025-06-15T10:30:00"
                            NumberOfStops="0"/>
          </univ:UniversalRecord>
        </univ:UniversalRecordRetrieveRsp>
    """)

    def test_parse_pnr(self):
        handler = PnrRetrieveHandler()
        result = handler.parse_soap(self.SAMPLE)
        assert result["pnr_locator"] == "ABC123"
        assert result["status"] == "Active"
        assert len(result["segments"]) == 1
        assert result["segments"][0]["carrier"] == "CA"


# ── REST builder tests ───────────────────────────────────────────────────────

class TestAirSearchRestBuilder:
    def test_builds_catalog_request(self, cred_manager: CredentialManager):
        handler = AirSearchHandler()
        account = cred_manager.get_account("json_c")
        params = {
            "legs": [{"origin": "PEK", "destination": "LAX", "departure_date": "2025-07-01"}],
            "passengers": [{"type": "ADT", "count": 2}],
        }
        result = handler.build_rest(params, account)
        assert result.path == "/search/catalogofferings"
        body = result.body
        req = body["CatalogOfferingsQueryRequest"]["CatalogOfferingsRequest"][0]
        assert req["@type"] == "CatalogOfferingsRequestAir"
        assert req["PassengerCriteria"][0]["number"] == 2
        assert req["SearchCriteriaFlight"][0]["From"]["value"] == "PEK"


class TestAirSearchRestParser:
    def test_parse_offerings(self):
        handler = AirSearchHandler()
        json_data = {
            "CatalogOfferings": {
                "CatalogOffering": [
                    {
                        "id": "offer_1",
                        "ProductOptions": [{
                            "Product": [{
                                "FlightSegment": [{
                                    "carrier": "AA",
                                    "number": "100",
                                    "Departure": {"location": "PEK", "dateTime": "2025-07-01T10:00:00"},
                                    "Arrival": {"location": "LAX", "dateTime": "2025-07-01T14:00:00"},
                                    "stops": 0,
                                }],
                            }],
                        }],
                        "Price": {"Base": 800, "TotalTaxes": 200, "TotalPrice": 1000, "currencyCode": "USD"},
                    }
                ]
            }
        }
        result = handler.parse_rest(json_data)
        assert result["total_count"] == 1
        assert result["items"][0]["prices"][0]["total"] == 1000.0
        assert result["items"][0]["segments"][0]["carrier"] == "AA"


# ── Money parser tests ───────────────────────────────────────────────────────

class TestMoneyParser:
    def test_usd(self):
        assert _money("USD1234.56") == 1234.56

    def test_cny(self):
        assert _money("CNY980.00") == 980.0

    def test_empty(self):
        assert _money("") == 0.0

    def test_plain_number(self):
        assert _money("450.00") == 450.0
