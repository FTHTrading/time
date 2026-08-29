# ACNC Carbon Program Research Proposal (v0.1)

## Document Control

| Field | Value |
|---|---|
| **Document ID** | `ACNC-RP-2026-001` |
| **Version** | `0.1 — DRAFT` |
| **Classification** | Internal / Draft Documentation for Partner & Methodology-Advisor Review |
| **Prepared by** | UnyKorn LLC — ACNC Climate Projects Division |
| **Date** | 2026-08-29 |

---

> [!IMPORTANT]
> **Research and pilot status — not a crediting offer.**
> This documentation authorizes research, software development, partner diligence, and a controlled pilot only. ACNC does not issue, sell, market, tokenize, promise, or represent carbon credits or offsets. Any future issuance depends on an eligible project, applicable methodology, independent validation and verification, registry approval, and all applicable legal, contractual, consumer-protection, privacy, and environmental-claim requirements.

---

## 1. Executive Summary

### 1.1 The Context

Household-scale climate interventions can face material aggregation, data, validation, and transaction-cost barriers. Project design, validation, and verification costs require validation through written quotes from project developers, VVBs, registries, and legal advisors during pilot diligence. Historically, these friction points have concentrated carbon crediting in large industrial, forestry, and land-use projects. Simultaneously, unsubstantiated environmental claims from consumer applications may create consumer-protection and greenwashing risk. ACNC will make only claims supported by evidence and applicable review.

### 1.2 The ACNC Aggregation Approach

ACNC is developing research and pilot infrastructure for privacy-preserving measurement, reporting, verification, and participant engagement. ACNC does not currently issue, sell, market, tokenize, or represent carbon credits or offsets. Any future credit issuance requires an eligible project, applicable methodology, independent validation and verification, registry approval, and compliance with applicable law. The platform's role is limited to:

1. **Evidence capture and canonicalization** — ingesting utility interval data, IoT sensor readings, commissioning records, and partner attestations into deterministic, hash-anchored records.
2. **Deterministic quantification** — computing conservative emission reductions using integer arithmetic, published emission factors, and methodology-aligned uncertainty discounts.
3. **Anti-duplication and governance** — enforcing serial-level double-counting guards, role-based review gates, and append-only audit trails.
4. **Audit-ready packaging** — generating monitoring reports and evidence bundles structured for potential future VVB verification and registry review, subject to project eligibility.

### 1.3 Canonical Product Claim

> **ACNC produces evidence-grade impact records and registry-ready carbon-accounting packages. Carbon credits are issued only by an authorized registry after third-party validation and verification.**

---

## 2. Theory of Change

```
[ Participant Enrollment & Pseudonymous ID Hash ]
                  │
                  ▼
[ 12–24 Month Baseline Utility Data Ingestion ]
                  │
                  ▼
[ Documented Qualifying Intervention (Commissioning & Serial Registration) ]
                  │
                  ▼
[ Continuous Monitoring & Cryptographic Evidence Ingestion (SHA-256 / Merkle Root) ]
                  │
                  ▼
[ Deterministic Rust Engine: Baseline vs. Observed Calculation - Leakage - Uncertainty Discount ]
                  │
                  ▼
[ Reduction Candidate Generated (Status: DRAFT -> EVIDENCE_COMPLETE -> PENDING_INTERNAL_REVIEW) ]
                  │
                  ▼
[ Independent QA & Methodology Compliance Review (Status: VALIDATED) ]
                  │
                  ▼
[ Potential Future VVB Verification & Registry Review, Subject to Project Eligibility (PENDING_VVB_VERIFICATION) ]
                  │
                  ▼
[ Registry May Issue Serialized Units Only After It Approves a Verified Project Submission (REGISTRY_ISSUED - Read Only) ]
                  │
                  ▼
[ Project-Level Benefit Sharing & Buffer Pool Allocation ]
                  │
                  ▼
[ Separate, Non-Transferable ACNC Utility Reward (VTIME) Allocated to Enrolled Participants ]
```

---

## 3. Preliminary Methodology Screening

### 3.1 Preliminary Methodology-Screening Map

> [!WARNING]
> **Preliminary methodology-screening map, subject to eligibility analysis and written expert confirmation.** No methodology is confirmed as applicable until a qualified carbon-market methodology advisor verifies project fit, current eligibility, geography, version, and registry acceptance.

| Intervention Category | Preliminary Candidate Methodology | Target Registry (if applicable) | Notes |
|---|---|---|---|
| Residential HVAC upgrade (heat pump) | CDM AMS-II.C (demand-side efficiency) — *to be confirmed by advisor* | Verra VCS | Requires metered baseline & post-installation data |
| Residential building thermal shell | CDM AMS-II.E (building efficiency) — *to be confirmed by advisor* | Verra VCS / Gold Standard | Applicable to insulation, windows, weatherization |
| Smart thermostat load management | VM0038 or AMS-II.C variant — *to be confirmed by advisor* | Verra VCS | Requires demonstration of additionality beyond utility programs |
| Residential appliance replacement | CDM AMS-II.J or similar — *to be confirmed by advisor* | Verra VCS | Need ENERGY STAR certification documentation |
| Public transit mode shift | CDM AMS-III.C variant — *to be confirmed by advisor* | Gold Standard | Requires verifiable trip documentation & transit partner data |
| Landfill waste diversion | CDM AMS-III.E — *to be confirmed by advisor* | Verra VCS / Gold Standard | Requires certified scale tickets and MRF manifest data |
| Community distributed solar | VM0038 or AMS-I.F — *to be confirmed by advisor* | Verra VCS | Requires revenue-grade sub-metering and grid displacement proof |

---

## 4. Status Vocabulary

The platform strictly enforces the following standardized 9-status taxonomy across all surfaces:

| Status | Meaning |
|---|---|
| `LOCAL RECORD` | User-created record stored locally or in the ACNC evidence system. |
| `ESTIMATED` | Calculated from declared inputs and a disclosed factor/model. |
| `RECEIPT BACKED` | Supported by a submitted invoice, receipt, or statement; not yet independently verified. |
| `METERED` | Supported by an approved meter, utility, facility, or partner data source. |
| `ATTESTED` | Confirmed by an authorized third party under documented rules. |
| `VALIDATED` | Reviewed for ACNC program completeness; not a registry-issued unit. |
| `PENDING VVB VERIFICATION` | Included in a project monitoring package awaiting independent verification. |
| `REGISTRY ISSUED` | Confirmed by the applicable registry through official issued-unit records (read-only). |
| `REGISTRY RETIRED` | Confirmed by the applicable registry as retired and unavailable for further use (read-only). |

---

*Document prepared by UnyKorn LLC — ACNC Climate Projects Division.*
