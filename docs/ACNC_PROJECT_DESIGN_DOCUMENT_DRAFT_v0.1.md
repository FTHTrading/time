# ACNC Project Design Document — Draft (v0.1)

## Document Control

| Field | Value |
|---|---|
| **Document ID** | `ACNC-PDD-2026-001` |
| **Version** | `0.1 — DRAFT` |
| **Classification** | Pre-Submission Draft for Partner & Methodology-Advisor Review |
| **Project Title** | ACNC Georgia Home Heat-Pump Retrofit Evidence Pilot |
| **Project Proponent** | UnyKorn LLC — ACNC Climate Projects SPV (to be incorporated) |
| **Host Country** | United States of America |
| **Host Region** | State of Georgia — Metro Atlanta & North Georgia |
| **Sectoral Scope** | Scope 3: Energy Demand (Residential Buildings) |
| **Target Registry** | Verra Verified Carbon Standard (VCS) — Grouped Project (Candidate Pathway) |
| **Date** | 2026-08-29 |

---

> [!IMPORTANT]
> **Research and pilot status — not a crediting offer.**
> This documentation authorizes research, software development, partner diligence, and a controlled pilot only. ACNC does not issue, sell, market, tokenize, promise, or represent carbon credits or offsets. Any future issuance depends on an eligible project, applicable methodology, independent validation and verification, registry approval, and all applicable legal, contractual, consumer-protection, privacy, and environmental-claim requirements.

---

## A. Project Description

### A.1 Purpose and General Description

The ACNC Georgia Home Heat-Pump Retrofit Evidence Pilot ("the Project") is an aggregated pilot project designed to evaluate documented residential heat-pump retrofits across qualifying households in the State of Georgia, United States. The Project uses a digital Measurement, Reporting, and Verification (dMRV) platform to capture standardized utility telemetry, commission qualifying interventions, and generate research-grade `ReductionCandidate` records suitable for methodology-advisor review and potential future independent verification.

The Project's operational boundary encompasses Scope 1 (on-site combustion) and Scope 2 (purchased electricity) emissions from participating residential premises. The Project does not claim Scope 3 upstream or downstream emissions reductions.

### A.2 Pilot Intervention

To ensure rigorous auditability, the first pilot focuses on a single, auditable intervention:

- **HVAC System Replacement**: Replacement of existing fossil-fuel or low-efficiency HVAC systems with ENERGY STAR-certified air-source or ground-source heat pumps meeting a minimum SEER2 rating of 16.0 and HSPF2 rating of 9.0.

---

## B. Preliminary Methodology Screening

### B.1 Preliminary Methodology Mapping

> [!WARNING]
> **Preliminary methodology-screening map, subject to eligibility analysis and written expert confirmation.**

- **Primary Candidate:** CDM AMS-II.C "Demand-side energy efficiency activities for specific technologies" (subject to written methodology advisor confirmation).

---

## C. Quantification of GHG Emission Reductions

### C.1 Baseline Emissions

$$BE_{y} = \sum_{i=1}^{n} \left( E_{elec,i,baseline,norm} \times EF_{grid,y} + E_{gas,i,baseline,norm} \times EF_{gas} \right)$$

### C.2 Project Emissions

$$PE_{y} = \sum_{i=1}^{n} \left( E_{elec,i,monitoring} \times EF_{grid,y} + E_{gas,i,monitoring} \times EF_{gas} \right)$$

### C.3 Net Emission Reductions

$$ER_{y} = \max\left(0,\ BE_{y} - PE_{y} - LE_{y}\right) \times \left(1 - \frac{UD_{bps}}{10{,}000}\right)$$

---

*Document prepared by UnyKorn LLC — ACNC Climate Projects Division.*
