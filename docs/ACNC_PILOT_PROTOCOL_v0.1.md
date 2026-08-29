# ACNC Pilot Protocol — Georgia Household Efficiency (v0.1)

## Document Control

| Field | Value |
|---|---|
| **Document ID** | `ACNC-PILOT-2026-001` |
| **Version** | `0.1 — DRAFT` |
| **Classification** | Internal / Program Design |
| **Pilot Territory** | Metro Atlanta & North Georgia |
| **Target Counties** | Fulton, Gwinnett, Cobb, DeKalb |
| **Utility Territories** | Georgia Power (Southern Company), local EMCs |
| **eGRID Subregion** | SRSO (SERC South) |
| **Date** | 2026-08-29 |

---

> [!IMPORTANT]
> **SCOPE:** This document defines the enrollment criteria, data collection procedures, intervention requirements, and success metrics for the ACNC Georgia Household Efficiency Pilot Program. It does not authorize credit issuance. All crediting decisions remain with the selected registry following VVB verification.

---

## 1. Pilot Objectives

### 1.1 Primary Objectives

1. **Validate the dMRV data pipeline** end-to-end, from utility API ingestion through deterministic Rust calculation to evidence package generation.
2. **Demonstrate calculation reproducibility** through independent blinded-sample verification.
3. **Establish baseline data quality benchmarks** for grouped-project registry submission.
4. **Test participant enrollment, consent, and engagement workflows** at small scale before expansion.

### 1.2 Secondary Objectives

1. Assess the feasibility of Green Button Connect My Data API integration with Georgia Power.
2. Identify contractor certification and commissioning workflow bottlenecks.
3. Estimate cost per tonne of monitoring and verification for household-scale interventions.
4. Generate preliminary data for methodology advisor review and registry pathway assessment.

---

## 2. Participant Enrollment

### 2.1 Eligibility Criteria

| Criterion | Requirement |
|---|---|
| **Property Type** | Single-family detached residence (owner-occupied) |
| **Location** | Within Fulton, Gwinnett, Cobb, or DeKalb County, GA |
| **Utility Provider** | Georgia Power or participating EMC with API/billing data access |
| **Baseline Data** | Minimum 12 consecutive months of electricity and/or gas consumption records |
| **Planned Intervention** | At least one qualifying HVAC replacement or thermal shell upgrade |
| **Consent** | Written informed consent for data sharing and program participation |
| **No Overlapping Claims** | Not concurrently enrolled in a utility DSM program claiming the same environmental attribute |

### 2.2 Exclusion Criteria

- Multi-family buildings (>4 units per structure).
- Rental properties without documented landlord-tenant consent.
- Premises with active solar PV systems (Phase 1 exclusion to simplify baseline).
- Premises with incomplete baseline data (<10 of 12 months).

### 2.3 Cohort Size

- **Minimum viable cohort:** 50 households.
- **Target cohort:** 100–200 households.
- **Maximum enrollment cap:** 500 households (Phase 1).

### 2.4 Enrollment Process

```
1. Participant completes online enrollment form (demographics, address, utility account).
2. Platform verifies premise eligibility (county, utility territory, property type).
3. Participant authorizes Green Button API access or uploads 12 months of billing data.
4. Platform validates baseline data completeness (≥10/12 months).
5. Participant reviews and signs digital consent and data-sharing agreement.
6. KYC hash generated (SHA-256 of name + address + DOB); raw PII discarded.
7. Enrollment confirmed; participant receives baseline assessment report.
```

---

## 3. Target Interventions

### 3.1 Intervention Category A: HVAC Replacement

| Parameter | Requirement |
|---|---|
| **Qualifying Equipment** | Air-source or ground-source heat pump |
| **Minimum Efficiency** | SEER2 ≥ 16.0, HSPF2 ≥ 9.0 |
| **Certification** | ENERGY STAR certified at time of installation |
| **Installation** | By Georgia-licensed HVAC contractor (license number documented) |
| **Documentation** | Model number, serial number, installation date, contractor license, invoice, photos |
| **Commissioning Report** | Contractor-signed form verifying proper installation, refrigerant charge, and airflow |

### 3.2 Intervention Category B: Thermal Shell Upgrade

| Parameter | Requirement |
|---|---|
| **Qualifying Measures** | Attic insulation (≥R-38), wall insulation (≥R-19), or ENERGY STAR window replacement |
| **Installation** | By licensed insulation/weatherization contractor |
| **Documentation** | Material specifications, R-value documentation, installer certification, photos |
| **Pre/Post Assessment** | Blower-door test (CFM50) before and after, if available |

### 3.3 Intervention Category C: Smart Thermostat

| Parameter | Requirement |
|---|---|
| **Qualifying Device** | ENERGY STAR-certified connected thermostat |
| **Documentation** | Device model, serial number, installation date, screenshot of programming |
| **Verification** | Contractor commissioning report or manufacturer API setback verification |

---

## 4. Data Collection Timeline

### 4.1 Phase Schedule

| Phase | Duration | Activity |
|---|---|---|
| **Phase 0: Enrollment** | Months 1–3 | Participant recruitment, consent, baseline data collection |
| **Phase 1: Baseline Period** | Months 4–15 | 12-month baseline consumption monitoring (no intervention) |
| **Phase 2: Intervention** | Months 16–18 | Qualifying intervention installed and commissioned |
| **Phase 3: Monitoring Period 1** | Months 19–30 | 12-month post-intervention monitoring |
| **Phase 4: Analysis & Reporting** | Months 31–33 | Calculation, QA, evidence package assembly |
| **Phase 5: Independent Review** | Months 34–36 | Blinded-sample verification, methodology advisor review |

### 4.2 Note on Accelerated Timeline

For participants with pre-existing 12+ months of historical utility data, Phases 0–1 may be compressed. The platform can ingest historical Green Button data retroactively, subject to data completeness validation.

---

## 5. Monitoring Parameters

### 5.1 Continuous Monitoring

| Parameter | Source | Frequency | Unit |
|---|---|---|---|
| Electricity consumption | Green Button API or monthly bill | 15-min interval or monthly | kWh |
| Natural gas consumption | Utility bill or AMR | Monthly | therms |
| Heating degree days (HDD) | NOAA GHCND — Atlanta Hartsfield (USW00013874) | Daily | °F·day (base 65) |
| Cooling degree days (CDD) | NOAA GHCND — Atlanta Hartsfield (USW00013874) | Daily | °F·day (base 65) |
| Indoor temperature setpoints | Smart thermostat API (if available) | Hourly | °F |

### 5.2 One-Time Capture (At Intervention)

| Parameter | Source | Captured |
|---|---|---|
| Pre-intervention equipment specs | Participant survey + photos | Before removal |
| Post-intervention equipment specs | Commissioning report | At installation |
| Conditioned floor area | Tax assessor records / participant | At enrollment |
| Occupancy count | Participant self-report | At enrollment + annually |
| Building vintage (year built) | Tax assessor records | At enrollment |

---

## 6. Emission Factors (Georgia-Specific)

| Factor | Value | Source | Year |
|---|---|---|---|
| SRSO Grid Electricity | 467 g CO2e / kWh | EPA eGRID SRSO subregion | 2024 |
| Natural Gas (combustion) | 5,302 g CO2e / therm | EPA GHG Emission Factors Hub | 2024 |
| Heating Oil #2 | 10,180 g CO2e / gallon | EPA GHG Emission Factors Hub | 2024 |
| Propane (LPG) | 5,740 g CO2e / gallon | EPA GHG Emission Factors Hub | 2024 |

---

## 7. Success Criteria

### 7.1 Quantitative KPIs

| KPI | Target | Measurement |
|---|---|---|
| **Data Completeness** | ≥ 90% of enrolled premises with ≥ 10/12 months complete data | Automated completeness tracker |
| **Evidence Provenance** | 100% SHA-256 evidence hashing and Merkle root anchoring | Automated hash verification |
| **No Duplicate Enrollments** | 0 duplicate premise IDs across the cohort | `AntiFraudValidator` hash check |
| **No Double-Claiming** | 0 overlapping REC or utility DSM environmental attribute claims | Anti-double-counting affidavit |
| **Weather Normalization Quality** | R² ≥ 0.75 for ≥ 80% of enrolled premises | Automated regression analysis |
| **Uncertainty Discount** | Minimum 1,000 BPS (10%) applied to all reduction calculations | Enforced by `uncertainty.rs` |
| **Blinded Reproducibility** | 100% bitwise match on 15% random sample | Independent reviewer recalculation |
| **Zero Privacy Breaches** | 0 incidents of unauthorized PII disclosure | Security audit log review |

### 7.2 Qualitative Milestones

| Milestone | Target |
|---|---|
| Methodology advisor written sign-off for registry pathway | Before Phase 5 completion |
| Formal stakeholder consultation completed | During Phase 0 |
| Participant satisfaction survey (NPS ≥ 40) | After Phase 3 completion |
| FTC Green Guides compliance review passed | Before any public marketing |

---

## 8. Risk Mitigation (Pilot-Specific)

| Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|
| Green Button API unavailable from Georgia Power | Medium | High | Fall back to monthly bill upload; manually validate 20% sample |
| Insufficient cohort enrollment (<50 households) | Medium | High | Expand to additional counties; partner with local weatherization nonprofits |
| Contractor commissioning data quality issues | Medium | Medium | Develop standardized commissioning form; train 3–5 pilot contractors |
| Participant attrition (>30% dropout) | Low–Medium | Medium | Quarterly engagement communications; progress reports |
| Data completeness below 90% threshold | Low | High | Weekly automated monitoring; proactive gap-fill outreach |
| Weather anomaly year (extreme HDD/CDD deviation) | Low | Medium | Document deviation; apply ASHRAE 14 CVRMSE threshold for model validity |

---

## 9. Budget Estimate (Pilot Phase)

| Category | Estimated Cost | Notes |
|---|---|---|
| Platform development & maintenance | In-kind (UnyKorn LLC) | Rust engine, web UI, infrastructure |
| Green Button API integration | $5,000–$15,000 | Utility data connector development |
| Contractor training & commissioning forms | $3,000–$8,000 | Standardized forms, 3–5 contractor sessions |
| Methodology advisory engagement | $15,000–$30,000 | Carbon market advisor for methodology mapping |
| Participant incentives | $5,000–$20,000 | Enrollment stipends, energy audit subsidies |
| VVB engagement (scoping) | $10,000–$25,000 | Pre-validation scoping and cost estimate |
| Legal (consent, benefit-sharing) | $5,000–$10,000 | Template agreement drafting |
| **Total Estimated Pilot Budget** | **$43,000–$108,000** | |

---

## 10. Post-Pilot Decision Gate

Upon completion of Phase 5, the following decision gate applies:

| Criterion | Threshold | Decision |
|---|---|---|
| All 7.1 quantitative KPIs met | Pass all | → Proceed to VVB engagement and registry pipeline listing |
| 5–6 of 7 quantitative KPIs met | Conditional | → Remediate gaps, extend pilot by 6 months, re-evaluate |
| <5 quantitative KPIs met | Fail | → Major redesign of data pipeline or methodology approach |

---

*Document prepared by UnyKorn LLC — ACNC Climate Projects Division.*
