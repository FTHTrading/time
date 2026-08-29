# ACNC Carbon Program Research Proposal (v0.1)

## Document Control

| Field | Value |
|---|---|
| **Document ID** | `ACNC-RP-2026-001` |
| **Version** | `0.1 — DRAFT` |
| **Classification** | Internal / Pre-Submission Research |
| **Prepared by** | UnyKorn LLC — ACNC Climate Projects Division |
| **Date** | 2026-08-29 |

---

> [!IMPORTANT]
> **DECISION GATE & STATUTORY DISCLAIMER (v0.1):**
> This document authorizes research, technical architecture, and pilot program design only. It does not authorize the issuance, sale, tokenization, marketing, forward delivery, or representation of carbon credits or carbon offsets. All crediting, serialized unit creation, and environmental attribute issuance remain strictly within the sole jurisdiction of recognized, accredited independent standards bodies and registries (e.g., Verra VCS, Gold Standard) following independent Validation and Verification Body (VVB) audit and official registry approval.

---

## 1. Executive Summary

### 1.1 The Problem

The Voluntary Carbon Market (VCM) is structurally inaccessible to household and community-scale climate interventions. Transaction costs for project design, validation, and verification ($50,000–$200,000+ per project cycle) restrict crediting mechanisms to large-scale industrial, forestry, and land-use projects. Simultaneously, consumer-facing applications claiming to "mint personal carbon credits" from unverified lifestyle logging violate additionality requirements, baseline rigor, and consumer protection standards including the US FTC Green Guides (16 CFR Part 260).

### 1.2 The ACNC Aggregation Thesis

ACNC proposes a digital Measurement, Reporting, and Verification (dMRV), cryptographic evidence, and participant engagement layer that aggregates standardized, tamper-evident household telemetry into grouped project records. Credit-issuance authority remains exclusively under accredited carbon registries. The platform's role is limited to:

1. **Evidence capture and canonicalization** — ingesting utility interval data, IoT sensor readings, commissioning records, and partner attestations into deterministic, hash-anchored records.
2. **Deterministic quantification** — computing conservative emission reductions using integer arithmetic, published emission factors, and methodology-aligned uncertainty discounts.
3. **Anti-duplication and governance** — enforcing serial-level double-counting guards, role-based review gates, and append-only audit trails.
4. **Registry-ready packaging** — generating monitoring reports and evidence bundles structured for VVB review and registry submission.

### 1.3 Canonical Product Claim

> **ACNC produces evidence-grade impact records and registry-ready carbon-accounting packages. Carbon credits are issued only by an authorized registry after third-party validation and verification.**

---

## 2. Theory of Change

```
┌─────────────────────────────────────────────────────────────────────┐
│  INPUT LAYER                                                        │
│                                                                     │
│  Participant enrollment → KYC hash → Consent & data-sharing agreement│
│  12–24 month baseline utility/activity ingestion                    │
│  Qualifying intervention documentation (commissioning, serial no.)  │
└──────────────────────────┬──────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────────┐
│  MEASUREMENT LAYER (Rust acnc-carbon-core)                          │
│                                                                     │
│  Continuous monitoring & cryptographic evidence ingestion            │
│  SHA-256 / Merkle root evidence hashing                             │
│  Deterministic baseline vs. observed calculation                    │
│  Leakage deduction + uncertainty discount                           │
│  ReductionCandidate generation (DRAFT → EVIDENCE_COMPLETE)          │
└──────────────────────────┬──────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────────┐
│  REVIEW & VERIFICATION LAYER                                        │
│                                                                     │
│  Internal QA & methodology compliance review (→ VALIDATED)          │
│  Aggregated monitoring report to accredited VVB                     │
│  Independent VVB verification (ISO 14064-3)                         │
│  Registry certification & serial-numbered credit issuance           │
└──────────────────────────┬──────────────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────────────┐
│  DISTRIBUTION LAYER                                                 │
│                                                                     │
│  Project-level benefit sharing & buffer pool allocation              │
│  Non-transferable VTIME utility reward to enrolled participants      │
│  Registry retirement anchoring on-chain (RegistryRetirementAnchor)  │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 3. Applicable Methodology Mapping

### 3.1 Phase 1 Intervention Categories and Candidate Methodologies

| Intervention Category | Candidate Methodology | Registry | Notes |
|---|---|---|---|
| Residential HVAC upgrade (heat pump) | CDM AMS-II.C "Demand-side energy efficiency activities for specific technologies" | Verra VCS (via CDM methodology) | Requires metered baseline & post-installation data |
| Residential building thermal shell | CDM AMS-II.E "Energy efficiency and fuel switching measures in buildings" | Verra VCS / Gold Standard | Applicable to insulation, windows, weatherization |
| Smart thermostat load management | VM0038 or aggregated AMS-II.C variant | Verra VCS | Requires demonstration of additionality beyond utility programs |
| Residential appliance replacement | CDM AMS-II.J "Demand-side activities for efficient lighting technologies" (by analogy) | Verra VCS | Need ENERGY STAR certification documentation |
| Public transit mode shift | CDM AMS-III.C "Emission reductions by electric and hybrid vehicles" (variant) | Gold Standard | Requires verifiable trip documentation & transit partner data |
| Landfill waste diversion | CDM AMS-III.E "Avoidance of methane production from decay of biomass" | Verra VCS / Gold Standard | Requires certified scale tickets and MRF manifest data |
| Community distributed solar | VM0038 or CDM AMS-I.F "Renewable electricity generation" | Verra VCS | Requires revenue-grade sub-metering and grid displacement proof |

### 3.2 Methodology Selection Criteria

A candidate methodology must satisfy all of the following:

1. **Recognized by an ICROA/ICVCM-endorsed registry** (Verra, Gold Standard, ACR, CAR, Puro.earth).
2. **Applicable to the specific intervention and geography** with documented baseline and monitoring procedures.
3. **Compatible with aggregated grouped-project design** allowing multiple participants under a single project registration.
4. **Includes conservative quantification provisions** for leakage, uncertainty, and non-permanence risk.
5. **Implemented or implementable** in the ACNC deterministic Rust engine using integer arithmetic and disclosed emission factors.

---

## 4. Additionality Framework

### 4.1 Regulatory Surplus Test

All project interventions must exceed the requirements of applicable local, state, and federal law. For the Georgia pilot:

- Interventions must exceed current Georgia Building Code (International Energy Conservation Code 2015 as adopted) and any Georgia Power / EMC demand-side management rebate requirements.
- Equipment efficiency ratings must exceed ENERGY STAR minimum thresholds by a documented margin.

### 4.2 Investment/Barrier Analysis

Documentation must demonstrate that the carbon revenue contribution was a material factor in the investment decision, or that a non-financial barrier (information, institutional, technological) was overcome through program participation.

### 4.3 Common Practice Analysis

The intervention penetration rate in the relevant jurisdiction must be below 50% of comparable premises (documented via Census ACS housing data, ENERGY STAR Market & Industry Scoping Reports, or Georgia Power program enrollment data).

---

## 5. Baseline Determination & Weather Normalization

### 5.1 Baseline Period

A minimum of 12 consecutive months of pre-intervention utility consumption data, sourced via:

- **Preferred:** Green Button Connect (CMD/DMD) or utility API interval data (15-minute or hourly resolution).
- **Acceptable:** Monthly billing data with weather normalization via heating degree days (HDD) and cooling degree days (CDD) regression.

### 5.2 Weather Normalization Model

Baseline consumption is normalized using a piecewise linear regression model:

$$E_{baseline,norm} = \beta_0 + \beta_{HDD} \cdot HDD_{period} + \beta_{CDD} \cdot CDD_{period}$$

Where:
- $E_{baseline,norm}$ is the weather-normalized baseline consumption (kWh or therms).
- $HDD_{period}$ and $CDD_{period}$ are NOAA station-observed degree days for the monitoring period.
- $\beta$ coefficients are derived from the baseline period regression.

**Minimum regression quality:** $R^2 \ge 0.75$ for model acceptance.

### 5.3 Baseline Adjustments

Baselines are adjusted for documented non-routine events including:
- Change in occupancy count (±20% threshold).
- Structural additions or removals (>10% conditioned floor area change).
- Extended vacancy periods (>30 consecutive days).

---

## 6. Emission Factor Governance

### 6.1 Factor Sources

| Domain | Source | Version | Update Frequency |
|---|---|---|---|
| US Grid Electricity | EPA eGRID | 2024 (latest available) | Annual |
| Subregional Grid | EPA eGRID Subregion (SRSO for Georgia) | 2024 | Annual |
| Natural Gas | EPA GHG Emission Factors Hub | 2024 | Annual |
| Fuel Oil / Propane | EPA GHG Emission Factors Hub | 2024 | Annual |
| Vehicle Miles | EPA GHG Hub + DOT NHTS | 2024 | Annual |
| Transit | DOT FTA National Transit Database | 2024 | Annual |
| Landfill Methane Avoidance | EPA WARM v15 | 2024 | Periodic |

### 6.2 Factor Versioning & Reproducibility

All emission factors used in calculations are:
- Stored with `factor_id`, `geography`, `source_uri`, `version`, `valid_from`, and `valid_to` in the Rust `FactorRegistry`.
- Immutably recorded in each `FootprintEstimate` and `ReductionCandidate` output.
- Reproducible by any third-party auditor with access to the stated source and version.

---

## 7. Quantification & Conservative Discounting

### 7.1 Core Formula

$$\text{Conservative Reduction (grams CO2e)} = \max\left(0,\ \text{Baseline} - \text{Observed} - \text{Leakage}\right) \times \frac{10{,}000 - \text{Uncertainty BPS}}{10{,}000}$$

### 7.2 Leakage Assessment

| Leakage Source | Default Deduction | Justification |
|---|---|---|
| Rebound effect (energy efficiency) | 500 bps (5.0%) | Behavioral rebound per Sorrell et al. (2009) |
| Fuel-switching displacement | 300 bps (3.0%) | Upstream fuel-cycle displacement |
| Materials lifecycle | 200 bps (2.0%) | Embodied carbon of replacement equipment |

### 7.3 Uncertainty Discount Schedule

| Evidence Tier | Uncertainty Discount (bps) | Effective Discount |
|---|---|---|
| `METERED` (15-min interval data) | 500 | 5% |
| `ATTESTED` (certified contractor) | 500 | 5% |
| `RECEIPT_BACKED` (invoice/bill) | 1,500 | 15% |
| `ESTIMATED` (modeled/inferred) | 3,500 | 35% |
| `USER_ENTERED` (self-reported) | 5,000 | 50% |
| `UNVERIFIED` | 10,000 | 100% (no credit) |

---

## 8. Anti-Double-Counting Protocol

### 8.1 Scope

ACNC enforces three dimensions of double-counting prevention as defined by the ICVCM Core Carbon Principles:

1. **Double Issuance:** No two registries may issue credits for the same intervention. ACNC checks the Verra Registry System, Gold Standard Impact Registry, and all applicable state and utility REC tracking systems before project registration.
2. **Double Claiming:** No participant premise may be enrolled in overlapping emission-reduction programs (utility DSM rebate programs, state clean-energy incentive programs) claiming the same environmental attribute without documented consent and attribute allocation.
3. **Double Use:** Each registry serial number is marked as consumed exactly once in the `DoubleCountingGuard` and `RegistryRetirementAnchor.sol`.

### 8.2 Implementation

- SHA-256 hash of each `(premise_id, intervention_type, monitoring_period)` tuple is stored in the `AntiFraudValidator`.
- Duplicate hash submissions are algorithmically rejected.
- All serialized credit retirements are anchored on-chain with single-use serial guards.

---

## 9. Registry Integration Strategy

### 9.1 Phase 1: Retirement Verification (Current)

ACNC validates third-party registry retirement records from Gold Standard, Verra VCS, Puro.earth, CAR, and ACR via the `CarbonRegistryAdapter` trait. This allows the platform to confirm and anchor existing retirements without generating new credits.

### 9.2 Phase 2: Aggregated Project Submission (Target)

ACNC aggregates household monitoring data into a grouped-project submission to an accredited registry. The submission includes:
- Project Design Document (PDD)
- Baseline study and additionality demonstration
- Monitoring report with SHA-256 evidence roots
- Anti-double-counting affidavit
- Stakeholder consultation records

### 9.3 Phase 3: Continuous Issuance Pipeline (Future)

After successful pilot and initial crediting period, establish a continuous monitoring-period verification cycle with a contracted VVB, targeting quarterly or semi-annual verification rounds.

---

## 10. Intellectual Property & Licensing

- ACNC platform software: MIT License (open-source).
- Methodology specifications: Creative Commons Attribution-ShareAlike 4.0 International (CC BY-SA 4.0).
- Participant data: Governed by the ACNC Privacy & Data Governance Policy; never sold, shared, or disclosed without explicit consent.
- Carbon credit ownership: Retained by the Project Proponent SPV pending formal benefit-sharing agreement execution with enrolled participants.

---

## 11. Next Steps

1. Finalize Georgia pilot cohort enrollment criteria and IRB-equivalent data governance review.
2. Engage a carbon-market methodology advisor for formal VCS/GS methodology mapping.
3. Contract an accredited VVB for pilot validation scope definition and cost estimation.
4. Complete Rust test-vector suite for all Phase 1 intervention categories.
5. Execute data-sharing agreements with Georgia Power / local EMCs for Green Button API access.
6. Publish the Project Concept Note (PCN) to Verra's Pipeline Listing system.

---

*Document prepared by UnyKorn LLC — ACNC Climate Projects Division. For questions or collaboration inquiries, contact the project team via the repository at [github.com/FTHTrading/time](https://github.com/FTHTrading/time).*
