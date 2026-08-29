# ACNC Project Design Document — Draft (v0.1)

## Document Control

| Field | Value |
|---|---|
| **Document ID** | `ACNC-PDD-2026-001` |
| **Version** | `0.1 — DRAFT` |
| **Classification** | Pre-Submission / Internal Review |
| **Project Title** | ACNC Aggregated Household Energy Efficiency & Retrofit Program |
| **Project Proponent** | UnyKorn LLC — ACNC Climate Projects SPV (to be incorporated) |
| **Host Country** | United States of America |
| **Host Region** | State of Georgia — Metro Atlanta & North Georgia |
| **Sectoral Scope** | Scope 3: Energy Demand (Residential Buildings) |
| **Target Registry** | Verra Verified Carbon Standard (VCS) — Grouped Project |
| **Date** | 2026-08-29 |

---

> [!IMPORTANT]
> **STATUTORY DISCLAIMER:** This is a draft Project Design Document prepared for internal review and pre-submission stakeholder engagement. It does not constitute a registered project, a validated methodology application, or an approved credit-issuance authorization. Registration, validation, verification, and credit issuance are exclusively within the authority of the selected carbon registry following independent VVB audit.

---

## A. Project Description

### A.1 Purpose and General Description

The ACNC Aggregated Household Energy Efficiency & Retrofit Program ("the Project") is a grouped project designed to aggregate documented residential energy-efficiency interventions across qualifying households in the State of Georgia, United States. The Project uses a digital Measurement, Reporting, and Verification (dMRV) platform to capture standardized utility telemetry, commission qualifying interventions, and generate monitoring reports suitable for independent verification.

The Project's operational boundary encompasses Scope 1 (on-site combustion) and Scope 2 (purchased electricity) emissions from participating residential premises. The Project does not claim Scope 3 upstream or downstream emissions reductions.

### A.2 Project Activity

The Project encompasses the following qualifying interventions:

1. **HVAC System Replacement**: Replacement of existing fossil-fuel or low-efficiency HVAC systems with ENERGY STAR-certified air-source or ground-source heat pumps meeting a minimum SEER2 rating of 16.0 and HSPF2 rating of 9.0.

2. **Thermal Shell Upgrade**: Installation of additional wall insulation (minimum R-19), attic insulation (minimum R-38), or ENERGY STAR-certified window replacement achieving documented improvements in the building thermal envelope.

3. **Smart Thermostat Installation**: Installation of ENERGY STAR-certified smart thermostats with documented demand-response or setback programming verified by contractor commissioning report.

### A.3 Technology/Measures

All qualifying equipment must:
- Be commercially available, new (not refurbished), and installed by a licensed contractor.
- Meet or exceed ENERGY STAR specifications at the time of commissioning.
- Be documented with model number, serial number, installation date, installer license number, and photographic evidence.
- Be covered by a manufacturer warranty of at least 5 years.

---

## B. Application of Methodology

### B.1 Selected Methodology

**Primary:** CDM AMS-II.C "Demand-side energy efficiency activities for specific technologies" (Version 15.0 or latest).

**Secondary (thermal shell):** CDM AMS-II.E "Energy efficiency and fuel switching measures in buildings" (Version 12.0 or latest).

Both methodologies are approved for use under the Verra VCS Program as per the VCS Methodology Requirements (v4.4).

### B.2 Applicability Conditions

| Condition | Compliance |
|---|---|
| Activity replaces existing, less-efficient equipment | ✅ Documented pre-existing equipment survey |
| Equipment is commercially available | ✅ ENERGY STAR certification required |
| Aggregate small-scale threshold met | ✅ Grouped project <60 GWh annual savings |
| No double-counting with utility DSM claims | ✅ Anti-double-counting affidavit required |

### B.3 Project Boundary

The spatial boundary is the conditioned floor area of each enrolled residential premise, metered at the utility point of delivery. The temporal boundary is the crediting period defined by the selected methodology (typically 10 years, renewable).

```
┌──────────────────────────────────────────────┐
│  PROJECT BOUNDARY (per premise)              │
│                                              │
│  ┌──────────────────────────────────────┐    │
│  │  Metered Electricity (kWh)           │    │
│  │  Grid connection at utility meter    │    │
│  └──────────────────────────────────────┘    │
│                                              │
│  ┌──────────────────────────────────────┐    │
│  │  Metered Natural Gas (therms)        │    │
│  │  Utility meter or invoice            │    │
│  └──────────────────────────────────────┘    │
│                                              │
│  ┌──────────────────────────────────────┐    │
│  │  On-site Combustion (fuel oil/LPG)   │    │
│  │  Delivery invoices                   │    │
│  └──────────────────────────────────────┘    │
│                                              │
│  EXCLUDED: Scope 3, transportation,          │
│  embodied carbon, appliances not in scope    │
└──────────────────────────────────────────────┘
```

---

## C. Quantification of GHG Emission Reductions

### C.1 Baseline Emissions

$$BE_{y} = \sum_{i=1}^{n} \left( E_{elec,i,baseline,norm} \times EF_{grid,y} + E_{gas,i,baseline,norm} \times EF_{gas} \right)$$

Where:
- $BE_y$ = Baseline emissions in monitoring year $y$ (tCO2e)
- $E_{elec,i,baseline,norm}$ = Weather-normalized baseline electricity consumption for premise $i$ (kWh)
- $EF_{grid,y}$ = EPA eGRID subregion emission factor for year $y$ (tCO2e/kWh)
- $E_{gas,i,baseline,norm}$ = Weather-normalized baseline natural gas consumption for premise $i$ (therms)
- $EF_{gas}$ = Natural gas combustion emission factor (tCO2e/therm)
- $n$ = Number of enrolled premises

### C.2 Project Emissions

$$PE_{y} = \sum_{i=1}^{n} \left( E_{elec,i,monitoring} \times EF_{grid,y} + E_{gas,i,monitoring} \times EF_{gas} \right)$$

### C.3 Leakage

$$LE_{y} = (BE_y - PE_y) \times LF_{rebound} + LE_{upstream}$$

Where:
- $LF_{rebound}$ = Rebound effect leakage factor (default 5.0% = 500 bps)
- $LE_{upstream}$ = Upstream manufacturing lifecycle leakage (default 2.0% = 200 bps)

### C.4 Net Emission Reductions

$$ER_{y} = \max\left(0,\ BE_{y} - PE_{y} - LE_{y}\right) \times \left(1 - \frac{UD_{bps}}{10{,}000}\right)$$

Where $UD_{bps}$ is the uncertainty discount in basis points, determined by the evidence tier of the monitoring data.

---

## D. Monitoring Plan

### D.1 Monitoring Parameters

| Parameter | Source | Frequency | Responsible Party |
|---|---|---|---|
| Electricity consumption (kWh) | Green Button API / Utility bill | Monthly or 15-min interval | Platform + Utility |
| Natural gas consumption (therms) | Utility bill / AMR data | Monthly | Platform + Utility |
| Heating Degree Days (HDD) | NOAA GHCND station | Daily | Platform (automated) |
| Cooling Degree Days (CDD) | NOAA GHCND station | Daily | Platform (automated) |
| Equipment model/serial/efficiency | Commissioning report | At installation | Licensed contractor |
| Occupancy count | Participant self-report | Annual | Participant |
| Premise conditioned area (sq ft) | Tax assessor / participant | At enrollment | Platform |

### D.2 Data Management & Quality Assurance

- All monitoring data is canonicalized to JSON, hashed with SHA-256, and stored with a Merkle root evidence chain in the ACNC Rust engine.
- Data completeness is tracked per-premise per-month. Premises with <90% data completeness in any monitoring period are flagged for manual review.
- Automated anomaly detection flags consumption changes >3σ from historical patterns for manual investigation.

### D.3 Monitoring Report Frequency

- **Internal aggregation:** Quarterly (90-day monitoring periods).
- **VVB submission:** Annually or semi-annually, aligned with crediting period verification schedule.

---

## E. Additionality Demonstration

### E.1 Methodology: VCS Tool VT0001 (Tool for the Demonstration and Assessment of Additionality)

**Step 1 — Regulatory Surplus:** Georgia Building Code (IECC 2015 as adopted) does not mandate heat pump installation or thermal shell upgrades for existing residential buildings. The interventions are not legally required and therefore satisfy regulatory surplus.

**Step 2 — Investment Analysis:** Participants demonstrate that the incremental cost of qualifying equipment exceeds the cost of conventional replacement. Carbon revenue contribution (via project credit proceeds and VTIME utility incentives) is shown to reduce the payback period below the decision threshold.

**Step 3 — Barrier Analysis:** Information barriers (lack of energy audit data), access barriers (contractor availability in underserved communities), and institutional barriers (split-incentive in rental properties) are documented.

**Step 4 — Common Practice:** Heat pump adoption rate in the target geography is documented below 30% of comparable housing stock (per Census ACS and ENERGY STAR Market Reports).

---

## F. Environmental & Social Safeguards

- No involuntary resettlement.
- No destruction of critical natural habitat.
- No disproportionate burden on vulnerable communities.
- Free, prior, informed consent (FPIC) documented for all enrolled participants.
- Data privacy protections per ACNC Privacy & Data Governance Policy.
- Ethical boundaries per `ACNC_ETHICAL_BOUNDARIES_v0.1.md`: no biological human functions scored or penalized.

---

## G. Stakeholder Consultation

A local stakeholder consultation will be conducted in the pilot counties (Fulton, Gwinnett, Cobb, DeKalb) including:

1. Public notice and comment period (minimum 30 days).
2. Community meeting(s) in accessible venues.
3. Written response to all substantive comments.
4. Documentation of consultation process and outcomes in the final PDD.

---

## H. Crediting Period

- **Start Date:** To be determined upon project registration.
- **Duration:** 10 years (renewable for up to 2 additional periods of 7 years each, per VCS rules).
- **First Monitoring Period:** 12 months post-registration.

---

## I. Project Governance

| Role | Entity | Responsibility |
|---|---|---|
| Project Proponent | ACNC Climate Projects SPV | Registry account holder, VVB contracting, legal chain of title |
| Platform Operator | UnyKorn LLC | dMRV software, data ingestion, cryptographic evidence |
| Methodology Advisor | TBD (external) | Methodology selection, baseline study, quantification review |
| Data Attester | Licensed HVAC / weatherization contractors | Commissioning reports, equipment verification |
| VVB | TBD (ANAB/JAS-ANZ accredited) | Independent validation and verification (ISO 14064-3) |
| Carbon Registry | Verra (target) | Project registration, serial tracking, credit issuance |

---

*This document will be revised following methodology advisor review and formal registry pipeline listing.*
