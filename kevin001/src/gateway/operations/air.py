"""Air operation handlers — SOAP/XML ↔ standardised ↔ REST/JSON transformers.

Covers: air_search, air_price, air_book, air_ticket, pnr_retrieve.
"""

from __future__ import annotations

import re
import uuid
from datetime import datetime
from typing import Any
from xml.etree.ElementTree import fromstring, Element

from ..config import AccountConfig
from .base import OperationHandler, RestRequestInfo, SoapRequestInfo

# ── XML namespace helpers ────────────────────────────────────────────────────

def _ns(schema_ver: str) -> dict[str, str]:
    """Build Travelport UAPI namespace map for a given schema version."""
    return {
        "air": f"http://www.travelport.com/schema/air_{schema_ver}",
        "com": f"http://www.travelport.com/schema/common_{schema_ver}",
        "univ": f"http://www.travelport.com/schema/universal_{schema_ver}",
    }


def _tag(ns_prefix: str, local: str, nss: dict[str, str]) -> str:
    """Produce a Clark-notation tag ``{namespace}local``."""
    return f"{{{nss[ns_prefix]}}}{local}"


def _find_all_deep(root: Element, local_name: str) -> list[Element]:
    """Recursively find all elements whose local tag equals *local_name*."""
    results: list[Element] = []
    for elem in root.iter():
        tag = elem.tag.split("}")[-1] if "}" in elem.tag else elem.tag
        if tag == local_name:
            results.append(elem)
    return results


def _attr(elem: Element, name: str, default: str = "") -> str:
    return elem.attrib.get(name, default)


def _parse_dt(s: str) -> datetime | None:
    if not s:
        return None
    for fmt in ("%Y-%m-%dT%H:%M:%S.%f%z", "%Y-%m-%dT%H:%M:%S%z", "%Y-%m-%dT%H:%M:%S", "%Y-%m-%d"):
        try:
            return datetime.strptime(s, fmt)
        except ValueError:
            continue
    return None


# ═════════════════════════════════════════════════════════════════════════════
# 1. AIR SEARCH
# ═════════════════════════════════════════════════════════════════════════════

class AirSearchHandler(OperationHandler):
    name = "air_search"
    soap_service = "AirService"

    # ── SOAP builder ─────────────────────────────────────────────────────

    def build_soap(self, params: dict[str, Any], account: AccountConfig, schema_ver: str) -> SoapRequestInfo:
        nss = _ns(schema_ver)
        trace_id = params.get("trace_id") or uuid.uuid4().hex[:12]

        legs_xml = []
        for leg in params.get("legs", []):
            cabin_attr = ""
            cabin = leg.get("cabin_class") or params.get("cabin_class")
            if cabin:
                cabin_attr = f' CabinClass="{cabin}"'

            legs_xml.append(
                f'<air:SearchAirLeg xmlns:air="{nss["air"]}" xmlns:com="{nss["com"]}">'
                f'  <air:SearchOrigin>'
                f'    <com:CityOrAirport Code="{leg["origin"]}"/>'
                f'  </air:SearchOrigin>'
                f'  <air:SearchDestination>'
                f'    <com:CityOrAirport Code="{leg["destination"]}"/>'
                f'  </air:SearchDestination>'
                f'  <air:SearchDepTime PreferredTime="{leg["departure_date"]}"/>'
                f'  {f"<air:AirLegModifiers{cabin_attr}/>" if cabin_attr else ""}'
                f'</air:SearchAirLeg>'
            )

        pax_xml = []
        ref_idx = 1
        for pax in params.get("passengers", [{"type": "ADT", "count": 1}]):
            for _ in range(pax.get("count", 1)):
                pax_xml.append(
                    f'<com:SearchPassenger xmlns:com="{nss["com"]}" '
                    f'Code="{pax["type"]}" BookingTravelerRef="trav_{ref_idx}"/>'
                )
                ref_idx += 1

        modifiers = ""
        preferred = params.get("preferred_carriers")
        direct_only = params.get("direct_flights_only", False)
        max_results = params.get("max_results", 200)

        mod_parts = []
        if preferred:
            prov = "".join(f'<com:Provider Code="{account.provider}"/>' for _ in [1])
            carriers = "".join(f'<air:PermittedCarriers xmlns:air="{nss["air"]}"><com:Carrier Code="{c}"/></air:PermittedCarriers>' for c in preferred)
            mod_parts.append(carriers)
        if direct_only:
            mod_parts.append(f'<air:FlightType xmlns:air="{nss["air"]}" NonStopDirects="true"/>')

        if mod_parts:
            modifiers = (
                f'<air:AirSearchModifiers xmlns:air="{nss["air"]}" xmlns:com="{nss["com"]}" MaxSolutions="{max_results}">'
                f'  <air:PreferredProviders><com:Provider Code="{account.provider}"/></air:PreferredProviders>'
                f'  {"".join(mod_parts)}'
                f'</air:AirSearchModifiers>'
            )
        else:
            modifiers = (
                f'<air:AirSearchModifiers xmlns:air="{nss["air"]}" xmlns:com="{nss["com"]}" MaxSolutions="{max_results}">'
                f'  <air:PreferredProviders><com:Provider Code="{account.provider}"/></air:PreferredProviders>'
                f'</air:AirSearchModifiers>'
            )

        body = (
            f'<air:LowFareSearchReq xmlns:air="{nss["air"]}" xmlns:com="{nss["com"]}" '
            f'AuthorizedBy="user" TargetBranch="{account.credentials.target_branch}" '
            f'TraceId="{trace_id}">'
            f'  <com:BillingPointOfSaleInfo OriginApplication="UAPI"/>'
            f'  {"".join(legs_xml)}'
            f'  {modifiers}'
            f'  {"".join(pax_xml)}'
            f'</air:LowFareSearchReq>'
        )

        return SoapRequestInfo(service="AirService", xml_body=body)

    # ── REST builder ─────────────────────────────────────────────────────

    def build_rest(self, params: dict[str, Any], account: AccountConfig) -> RestRequestInfo:
        search_criteria = []
        for leg in params.get("legs", []):
            sc: dict[str, Any] = {
                "@type": "SearchCriteriaFlight",
                "departureDate": str(leg["departure_date"]),
                "From": {"value": leg["origin"]},
                "To": {"value": leg["destination"]},
            }
            cabin = leg.get("cabin_class") or params.get("cabin_class")
            if cabin:
                sc["cabinPreference"] = cabin
            search_criteria.append(sc)

        pax_criteria = []
        for pax in params.get("passengers", [{"type": "ADT", "count": 1}]):
            pax_criteria.append({
                "number": pax.get("count", 1),
                "passengerTypeCode": pax["type"],
            })

        modifiers: dict[str, Any] = {"@type": "SearchModifiersAir"}
        if params.get("preferred_carriers"):
            modifiers["carriers"] = [{"Carrier": {"id": c}} for c in params["preferred_carriers"]]
        if params.get("direct_flights_only"):
            modifiers["prohibitChangeOfAirport"] = True

        body = {
            "CatalogOfferingsQueryRequest": {
                "CatalogOfferingsRequest": [{
                    "@type": "CatalogOfferingsRequestAir",
                    "offersPerPage": params.get("max_results", 200),
                    "PassengerCriteria": pax_criteria,
                    "SearchCriteriaFlight": search_criteria,
                    "SearchModifiersAir": modifiers,
                }]
            }
        }

        return RestRequestInfo(path="/search/catalogofferings", body=body)

    # ── SOAP response parser ────────────────────────────────────────────

    def parse_soap(self, xml_text: str) -> dict[str, Any]:
        root = fromstring(xml_text)
        items = []

        # Find all AirPricingSolution or AirItinerarySolution elements
        solutions = _find_all_deep(root, "AirPricingSolution")
        if not solutions:
            solutions = _find_all_deep(root, "AirItinerarySolution")

        for sol in solutions:
            segments = []
            for seg_elem in _find_all_deep(sol, "AirSegment"):
                segments.append({
                    "carrier": _attr(seg_elem, "Carrier"),
                    "flight_number": _attr(seg_elem, "FlightNumber"),
                    "origin": _attr(seg_elem, "Origin"),
                    "destination": _attr(seg_elem, "Destination"),
                    "departure": _attr(seg_elem, "DepartureTime"),
                    "arrival": _attr(seg_elem, "ArrivalTime"),
                    "equipment": _attr(seg_elem, "Equipment"),
                    "cabin_class": _attr(seg_elem, "CabinClass") or None,
                    "booking_class": _attr(seg_elem, "ClassOfService") or None,
                    "stops": int(_attr(seg_elem, "NumberOfStops", "0")),
                })

            prices = []
            for info in _find_all_deep(sol, "AirPricingInfo"):
                base = _attr(info, "BasePrice", "0")
                taxes = _attr(info, "Taxes", "0")
                total = _attr(info, "TotalPrice", "0")
                currency = "".join(c for c in total if c.isalpha()) or "USD"
                prices.append({
                    "base_fare": _money(base),
                    "taxes": _money(taxes),
                    "total": _money(total),
                    "currency": currency,
                })

            items.append({
                "itinerary_id": _attr(sol, "Key") or None,
                "segments": segments,
                "prices": prices,
                "stops": max((s.get("stops", 0) for s in segments), default=0),
            })

        return {"items": items, "total_count": len(items)}

    # ── REST response parser ─────────────────────────────────────────────

    def parse_rest(self, json_data: dict[str, Any]) -> dict[str, Any]:
        items = []
        offerings = json_data.get("CatalogOfferings", {}).get("CatalogOffering", [])
        if isinstance(offerings, dict):
            offerings = [offerings]

        for offering in offerings:
            segments = []
            products = offering.get("ProductOptions", [])
            for po in (products if isinstance(products, list) else [products]):
                for product in (po.get("Product", []) if isinstance(po.get("Product"), list) else [po.get("Product", {})]):
                    for flight in (product.get("FlightSegment", []) if isinstance(product.get("FlightSegment"), list) else [product.get("FlightSegment", {})]):
                        if not flight:
                            continue
                        dep = flight.get("Departure", {})
                        arr = flight.get("Arrival", {})
                        segments.append({
                            "carrier": flight.get("carrier", ""),
                            "flight_number": flight.get("number", ""),
                            "origin": dep.get("location", ""),
                            "destination": arr.get("location", ""),
                            "departure": dep.get("dateTime", ""),
                            "arrival": arr.get("dateTime", ""),
                            "equipment": flight.get("equipment", "") or None,
                            "stops": flight.get("stops", 0),
                        })

            prices = []
            price_node = offering.get("Price", {})
            if price_node:
                prices.append({
                    "base_fare": float(price_node.get("Base", 0)),
                    "taxes": float(price_node.get("TotalTaxes", 0)),
                    "total": float(price_node.get("TotalPrice", 0)),
                    "currency": price_node.get("currencyCode", "USD"),
                })

            items.append({
                "itinerary_id": offering.get("id") or None,
                "segments": segments,
                "prices": prices,
                "stops": max((s.get("stops", 0) for s in segments), default=0),
            })

        return {"items": items, "total_count": len(items)}


# ═════════════════════════════════════════════════════════════════════════════
# 2. AIR PRICE
# ═════════════════════════════════════════════════════════════════════════════

class AirPriceHandler(OperationHandler):
    name = "air_price"
    soap_service = "AirService"

    def build_soap(self, params: dict[str, Any], account: AccountConfig, schema_ver: str) -> SoapRequestInfo:
        nss = _ns(schema_ver)
        trace_id = params.get("trace_id") or uuid.uuid4().hex[:12]

        seg_xml = []
        for seg in params.get("segments", []):
            seg_xml.append(
                f'<air:AirSegment xmlns:air="{nss["air"]}" xmlns:com="{nss["com"]}" '
                f'Carrier="{seg["carrier"]}" FlightNumber="{seg["flight_number"]}" '
                f'Origin="{seg["origin"]}" Destination="{seg["destination"]}" '
                f'DepartureTime="{seg["departure"]}" ArrivalTime="{seg["arrival"]}" '
                f'ProviderCode="{account.provider}" '
                f'Key="seg_{seg["carrier"]}{seg["flight_number"]}">'
                f'</air:AirSegment>'
            )

        pax_xml = []
        ref = 1
        for pax in params.get("passengers", [{"type": "ADT", "count": 1}]):
            for _ in range(pax.get("count", 1)):
                pax_xml.append(
                    f'<com:SearchPassenger xmlns:com="{nss["com"]}" '
                    f'Code="{pax["type"]}" BookingTravelerRef="trav_{ref}"/>'
                )
                ref += 1

        plating = ""
        if params.get("plating_carrier"):
            plating = f' PlatingCarrier="{params["plating_carrier"]}"'

        body = (
            f'<air:AirPriceReq xmlns:air="{nss["air"]}" xmlns:com="{nss["com"]}" '
            f'AuthorizedBy="user" TargetBranch="{account.credentials.target_branch}" '
            f'TraceId="{trace_id}">'
            f'  <com:BillingPointOfSaleInfo OriginApplication="UAPI"/>'
            f'  <air:AirItinerary>'
            f'    {"".join(seg_xml)}'
            f'  </air:AirItinerary>'
            f'  <air:AirPricingModifiers{plating}/>'
            f'  {"".join(pax_xml)}'
            f'  <air:AirPricingCommand CabinClass="Economy"/>'
            f'</air:AirPriceReq>'
        )

        return SoapRequestInfo(service="AirService", xml_body=body)

    def build_rest(self, params: dict[str, Any], account: AccountConfig) -> RestRequestInfo:
        segments = []
        for seg in params.get("segments", []):
            segments.append({
                "@type": "FlightSegment",
                "carrier": seg["carrier"],
                "number": seg["flight_number"],
                "Departure": {"location": seg["origin"], "dateTime": str(seg["departure"])},
                "Arrival": {"location": seg["destination"], "dateTime": str(seg["arrival"])},
            })

        pax = []
        for p in params.get("passengers", [{"type": "ADT", "count": 1}]):
            pax.append({"number": p.get("count", 1), "passengerTypeCode": p["type"]})

        body = {
            "OfferQueryBuildFromProducts": {
                "BuildFromProductsRequest": {
                    "@type": "BuildFromProductsRequestAir",
                    "PassengerCriteria": pax,
                    "Product": [{"FlightSegment": segments}],
                }
            }
        }

        return RestRequestInfo(path="/price/offer", body=body)

    def parse_soap(self, xml_text: str) -> dict[str, Any]:
        root = fromstring(xml_text)
        prices = []
        for info in _find_all_deep(root, "AirPricingInfo"):
            base = _attr(info, "BasePrice", "0")
            taxes = _attr(info, "Taxes", "0")
            total = _attr(info, "TotalPrice", "0")
            currency = "".join(c for c in total if c.isalpha()) or "USD"
            pax_type = "ADT"
            for bk in _find_all_deep(info, "BookingInfo"):
                pass  # Could extract cabin class here
            for pt in _find_all_deep(info, "PassengerType"):
                pax_type = _attr(pt, "Code", "ADT")
            prices.append({
                "base_fare": _money(base),
                "taxes": _money(taxes),
                "total": _money(total),
                "currency": currency,
                "passenger_type": pax_type,
            })

        deadline = None
        for tl in _find_all_deep(root, "TicketingModifiers"):
            dl = _attr(tl, "TicketByDate")
            if dl:
                deadline = dl

        return {"prices": prices, "fare_rules": [], "ticketing_deadline": deadline}

    def parse_rest(self, json_data: dict[str, Any]) -> dict[str, Any]:
        prices = []
        offer = json_data.get("Offer", {})
        price_node = offer.get("Price", {})
        if price_node:
            prices.append({
                "base_fare": float(price_node.get("Base", 0)),
                "taxes": float(price_node.get("TotalTaxes", 0)),
                "total": float(price_node.get("TotalPrice", 0)),
                "currency": price_node.get("currencyCode", "USD"),
            })
        return {"prices": prices, "fare_rules": [], "ticketing_deadline": None}


# ═════════════════════════════════════════════════════════════════════════════
# 3. AIR BOOK
# ═════════════════════════════════════════════════════════════════════════════

class AirBookHandler(OperationHandler):
    name = "air_book"
    soap_service = "AirService"

    def build_soap(self, params: dict[str, Any], account: AccountConfig, schema_ver: str) -> SoapRequestInfo:
        nss = _ns(schema_ver)
        trace_id = params.get("trace_id") or uuid.uuid4().hex[:12]

        seg_xml = []
        for seg in params.get("segments", []):
            seg_xml.append(
                f'<air:AirSegment xmlns:air="{nss["air"]}" xmlns:com="{nss["com"]}" '
                f'Carrier="{seg["carrier"]}" FlightNumber="{seg["flight_number"]}" '
                f'Origin="{seg["origin"]}" Destination="{seg["destination"]}" '
                f'DepartureTime="{seg["departure"]}" ArrivalTime="{seg["arrival"]}" '
                f'ProviderCode="{account.provider}" '
                f'Key="seg_{seg["carrier"]}{seg["flight_number"]}" '
                f'Group="0">'
                f'</air:AirSegment>'
            )

        traveler_xml = []
        for idx, tvl in enumerate(params.get("travelers", []), 1):
            contact = tvl.get("contact", {})
            phone = tvl.get("phone") or params.get("phone", "")
            email = tvl.get("email") or params.get("email", "")
            dob = tvl.get("date_of_birth", "")
            gender = tvl.get("gender", "")

            phone_xml = f'<com:PhoneNumber AreaCode="" CountryCode="1" Location="city" Number="{phone}" Type="Other"/>' if phone else ""
            email_xml = f'<com:Email EmailID="{email}" Type="Other"/>' if email else ""
            dob_attr = f' DOB="{dob}"' if dob else ""
            gender_attr = f' Gender="{gender}"' if gender else ""

            traveler_xml.append(
                f'<com:BookingTraveler xmlns:com="{nss["com"]}" '
                f'Key="trav_{idx}" TravelerType="{tvl.get("passenger_type", "ADT")}"{dob_attr}{gender_attr}>'
                f'  <com:BookingTravelerName First="{contact.get("first_name", "")}" '
                f'Last="{contact.get("last_name", "")}" Prefix="Mr"/>'
                f'  {phone_xml}'
                f'  {email_xml}'
                f'</com:BookingTraveler>'
            )

        pricing_xml = (
            f'<air:AirPricingModifiers xmlns:air="{nss["air"]}" xmlns:com="{nss["com"]}"'
            f'{f" PlatingCarrier={chr(34)}{params['plating_carrier']}{chr(34)}" if params.get("plating_carrier") else ""}'
            f'/>'
        )

        body = (
            f'<air:AirCreateReservationReq xmlns:air="{nss["air"]}" xmlns:com="{nss["com"]}" '
            f'AuthorizedBy="user" TargetBranch="{account.credentials.target_branch}" '
            f'TraceId="{trace_id}" ProviderCode="{account.provider}">'
            f'  <com:BillingPointOfSaleInfo OriginApplication="UAPI"/>'
            f'  {"".join(traveler_xml)}'
            f'  <air:AirItinerary>'
            f'    {"".join(seg_xml)}'
            f'  </air:AirItinerary>'
            f'  {pricing_xml}'
            f'</air:AirCreateReservationReq>'
        )

        return SoapRequestInfo(service="AirService", xml_body=body)

    def build_rest(self, params: dict[str, Any], account: AccountConfig) -> RestRequestInfo:
        travelers = []
        for idx, tvl in enumerate(params.get("travelers", []), 1):
            contact = tvl.get("contact", {})
            t: dict[str, Any] = {
                "@type": "Traveler",
                "personName": {
                    "given": contact.get("first_name", ""),
                    "surname": contact.get("last_name", ""),
                },
                "passengerTypeCode": tvl.get("passenger_type", "ADT"),
                "Telephone": [{"number": params.get("phone", "")}],
            }
            if params.get("email"):
                t["Email"] = [{"value": params["email"]}]
            travelers.append(t)

        segments = []
        for seg in params.get("segments", []):
            segments.append({
                "@type": "FlightSegment",
                "carrier": seg["carrier"],
                "number": seg["flight_number"],
                "Departure": {"location": seg["origin"], "dateTime": str(seg["departure"])},
                "Arrival": {"location": seg["destination"], "dateTime": str(seg["arrival"])},
            })

        body = {
            "ReservationQueryBuild": {
                "@type": "ReservationQueryBuildAir",
                "Traveler": travelers,
                "Product": [{"FlightSegment": segments}],
            }
        }

        return RestRequestInfo(path="/reserve/build", body=body)

    def parse_soap(self, xml_text: str) -> dict[str, Any]:
        root = fromstring(xml_text)
        locator = ""
        provider_loc = ""
        for ur in _find_all_deep(root, "UniversalRecord"):
            locator = _attr(ur, "LocatorCode") or locator
            for prl in _find_all_deep(ur, "ProviderReservationInfo"):
                provider_loc = _attr(prl, "LocatorCode") or provider_loc

        if not locator:
            for loc in _find_all_deep(root, "LocatorCode"):
                if loc.text:
                    locator = loc.text
                    break

        segments = _extract_segments_from_xml(root)

        return {
            "pnr_locator": locator,
            "provider_locator": provider_loc or None,
            "status": "Confirmed",
            "segments": segments,
            "travelers": [],
        }

    def parse_rest(self, json_data: dict[str, Any]) -> dict[str, Any]:
        res = json_data.get("Reservation", json_data)
        return {
            "pnr_locator": res.get("locatorCode", ""),
            "provider_locator": None,
            "status": "Confirmed",
            "segments": [],
            "travelers": [],
        }


# ═════════════════════════════════════════════════════════════════════════════
# 4. AIR TICKET
# ═════════════════════════════════════════════════════════════════════════════

class AirTicketHandler(OperationHandler):
    name = "air_ticket"
    soap_service = "AirService"

    def build_soap(self, params: dict[str, Any], account: AccountConfig, schema_ver: str) -> SoapRequestInfo:
        nss = _ns(schema_ver)
        trace_id = params.get("trace_id") or uuid.uuid4().hex[:12]
        pnr = params["pnr_locator"]

        commission = ""
        if params.get("commission_percent") is not None:
            commission = f'<air:Commission Level="Fare" Type="PercentBase" Percentage="{params["commission_percent"]}"/>'

        plating = ""
        if params.get("plating_carrier"):
            plating = f' PlatingCarrier="{params["plating_carrier"]}"'

        body = (
            f'<air:AirTicketingReq xmlns:air="{nss["air"]}" xmlns:com="{nss["com"]}" '
            f'AuthorizedBy="user" TargetBranch="{account.credentials.target_branch}" '
            f'TraceId="{trace_id}">'
            f'  <com:BillingPointOfSaleInfo OriginApplication="UAPI"/>'
            f'  <air:AirReservationLocatorCode>{pnr}</air:AirReservationLocatorCode>'
            f'  <air:AirTicketingModifiers{plating}>'
            f'    {commission}'
            f'  </air:AirTicketingModifiers>'
            f'</air:AirTicketingReq>'
        )

        return SoapRequestInfo(service="AirService", xml_body=body)

    def build_rest(self, params: dict[str, Any], account: AccountConfig) -> RestRequestInfo:
        body = {
            "TicketQueryIssue": {
                "@type": "TicketQueryIssueAir",
                "locatorCode": params["pnr_locator"],
            }
        }
        if params.get("plating_carrier"):
            body["TicketQueryIssue"]["platingCarrier"] = params["plating_carrier"]

        return RestRequestInfo(path="/ticket/issue", body=body)

    def parse_soap(self, xml_text: str) -> dict[str, Any]:
        root = fromstring(xml_text)
        pnr = ""
        tickets: list[str] = []

        for loc in _find_all_deep(root, "AirReservationLocatorCode"):
            if loc.text:
                pnr = loc.text
                break

        for tkt in _find_all_deep(root, "eTicketNumber"):
            if tkt.text:
                tickets.append(tkt.text)

        if not tickets:
            for tkt in _find_all_deep(root, "TicketNumber"):
                val = _attr(tkt, "Number") or (tkt.text or "")
                if val:
                    tickets.append(val)

        return {"pnr_locator": pnr, "ticket_numbers": tickets, "status": "Ticketed"}

    def parse_rest(self, json_data: dict[str, Any]) -> dict[str, Any]:
        ticket = json_data.get("Ticket", json_data)
        numbers = []
        if isinstance(ticket, dict):
            num = ticket.get("ticketNumber", "")
            if num:
                numbers.append(num)
        return {
            "pnr_locator": json_data.get("locatorCode", ""),
            "ticket_numbers": numbers,
            "status": "Ticketed",
        }


# ═════════════════════════════════════════════════════════════════════════════
# 5. PNR RETRIEVE
# ═════════════════════════════════════════════════════════════════════════════

class PnrRetrieveHandler(OperationHandler):
    name = "pnr_retrieve"
    soap_service = "UniversalRecordService"

    def build_soap(self, params: dict[str, Any], account: AccountConfig, schema_ver: str) -> SoapRequestInfo:
        nss = _ns(schema_ver)
        trace_id = params.get("trace_id") or uuid.uuid4().hex[:12]
        pnr = params["pnr_locator"]

        provider_code = params.get("provider_code") or account.provider

        body = (
            f'<univ:UniversalRecordRetrieveReq xmlns:univ="{nss["univ"]}" xmlns:com="{nss["com"]}" '
            f'AuthorizedBy="user" TargetBranch="{account.credentials.target_branch}" '
            f'TraceId="{trace_id}">'
            f'  <com:BillingPointOfSaleInfo OriginApplication="UAPI"/>'
            f'  <univ:ProviderReservationInfo ProviderCode="{provider_code}" '
            f'ProviderLocatorCode="{pnr}"/>'
            f'</univ:UniversalRecordRetrieveReq>'
        )

        return SoapRequestInfo(service="UniversalRecordService", xml_body=body)

    def build_rest(self, params: dict[str, Any], account: AccountConfig) -> RestRequestInfo:
        pnr = params["pnr_locator"]
        return RestRequestInfo(path=f"/retrieve/{pnr}", method="GET", body=None)

    def parse_soap(self, xml_text: str) -> dict[str, Any]:
        root = fromstring(xml_text)
        pnr = ""
        status = "Unknown"

        for ur in _find_all_deep(root, "UniversalRecord"):
            pnr = _attr(ur, "LocatorCode") or pnr
            status = _attr(ur, "Status") or "Active"

        segments = _extract_segments_from_xml(root)

        tickets: list[str] = []
        for tkt in _find_all_deep(root, "eTicketNumber"):
            if tkt.text:
                tickets.append(tkt.text)

        return {
            "pnr_locator": pnr,
            "status": status,
            "segments": segments,
            "travelers": [],
            "tickets": tickets,
            "raw": {},
        }

    def parse_rest(self, json_data: dict[str, Any]) -> dict[str, Any]:
        res = json_data.get("Reservation", json_data)
        return {
            "pnr_locator": res.get("locatorCode", ""),
            "status": res.get("status", "Active"),
            "segments": [],
            "travelers": [],
            "tickets": [],
            "raw": json_data,
        }


# ── Shared helpers ───────────────────────────────────────────────────────────

_MONEY_RE = re.compile(r"[A-Z]{3}([\d.]+)")


def _money(value: str) -> float:
    """Parse Travelport money strings like ``USD1234.56`` → 1234.56."""
    if not value:
        return 0.0
    m = _MONEY_RE.search(value)
    if m:
        return float(m.group(1))
    try:
        return float("".join(c for c in value if c.isdigit() or c == ".") or "0")
    except ValueError:
        return 0.0


def _extract_segments_from_xml(root: Element) -> list[dict[str, Any]]:
    segments: list[dict[str, Any]] = []
    for seg_elem in _find_all_deep(root, "AirSegment"):
        segments.append({
            "carrier": _attr(seg_elem, "Carrier"),
            "flight_number": _attr(seg_elem, "FlightNumber"),
            "origin": _attr(seg_elem, "Origin"),
            "destination": _attr(seg_elem, "Destination"),
            "departure": _attr(seg_elem, "DepartureTime"),
            "arrival": _attr(seg_elem, "ArrivalTime"),
            "equipment": _attr(seg_elem, "Equipment") or None,
            "stops": int(_attr(seg_elem, "NumberOfStops", "0")),
        })
    return segments
