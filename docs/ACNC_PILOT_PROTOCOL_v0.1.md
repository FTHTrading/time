# ACNC Pilot Protocol — Georgia Home Heat-Pump Retrofit Evidence Pilot (v0.1)

## Document Control

| Field | Value |
|---|---|
| **Document ID** | `ACNC-PILOT-2026-001` |
| **Version** | `0.1 — DRAFT` |
| **Classification** | Internal / Program Design for Partner & Methodology-Advisor Review |
| **Pilot Territory** | Defined Georgia Utility / EMC Territory (North Georgia / Metro Atlanta) |
| **Cohort Size** | 50–100 voluntary single-family households |
| **Intervention** | Documented qualifying SEER2 16+ heat-pump retrofit |
| **Date** | 2026-08-29 |

---

> [!IMPORTANT]
> **Research and pilot status — not a crediting offer.**
> This documentation authorizes research, software development, partner diligence, and a controlled pilot only. ACNC does not issue, sell, market, tokenize, promise, or represent carbon credits or offsets. Any future issuance depends on an eligible project, applicable methodology, independent validation and verification, registry approval, and all applicable legal, contractual, consumer-protection, privacy, and environmental-claim requirements.

---

## 1. Focused Pilot Architecture

To establish audit-grade data quality before considering broader intervention types, the pilot is deliberately constrained:

| Parameter | Specification |
|---|---|
| **Program Name** | ACNC Georgia Home Heat-Pump Retrofit Evidence Pilot |
| **Participant Cohort** | 50–100 voluntary single-family households |
| **Territory** | One defined utility or EMC service area in Georgia (SRSO subregion) |
| **Intervention** | Documented SEER2 16+ / HSPF2 9.0+ heat-pump installation |
| **Baseline** | Pre-install 12-month utility consumption and weather data |
| **Monitoring** | Post-install utility interval or billing data |
| **Evidence Package** | Utility data where available, contractor invoice, model/serial number, installation date, commissioning record, participant consent, duplicate-incentive screening |
| **Output** | Research-grade `ReductionCandidate` records |
| **Not an Output** | Carbon credits, offsets, tradable tokens, guaranteed payment, or environmental claim |

---

## 2. Partner Readiness Gates

Before approaching a registry or VVB for formal project listing, the following gates must be completed:

1. **Green CI Run & Reproducible Fixtures:** Validated Rust engine with integer precision.
2. **Emissions-Factor Manifest:** Cited, versioned, geography-specific factors (eGRID SRSO 2024).
3. **Written Methodology-Fit Opinion:** Formal memo from a qualified carbon-market advisor.
4. **Signed Partner Letters:** Pilot letters with installer, utility/data provider, or program administrator.
5. **Data & Privacy Package:** Participant consent, data-processing agreement, retention policies.
6. **Chain of Title:** Defined credit-rights and benefit-sharing structure.
7. **Double-Counting Screening:** Screen utility incentives, RECs, grants, and other claims.
8. **Independent Security & Privacy Review:** Code, hash, and data store audit.
9. **Blinded Sample Verification:** Independent recalculation of a 15% sample.
10. **Legal Review:** Consumer claims, rewards, utility terms, and privacy obligations.

---

*Document prepared by UnyKorn LLC — ACNC Climate Projects Division.*
