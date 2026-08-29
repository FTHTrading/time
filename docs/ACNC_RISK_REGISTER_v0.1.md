# ACNC Risk Register (v0.1)

## Document Control

| Field | Value |
|---|---|
| **Document ID** | `ACNC-RISK-2026-001` |
| **Version** | `0.1 — DRAFT` |
| **Classification** | Internal / Governance |
| **Date** | 2026-08-29 |

---

## 1. Risk Assessment Methodology

### 1.1 Scoring Framework

Each risk is assessed on two dimensions:

**Likelihood:**
| Score | Level | Description |
|---|---|---|
| 1 | Rare | <5% probability in the crediting period |
| 2 | Unlikely | 5–20% probability |
| 3 | Possible | 20–50% probability |
| 4 | Likely | 50–80% probability |
| 5 | Almost Certain | >80% probability |

**Impact:**
| Score | Level | Description |
|---|---|---|
| 1 | Negligible | Minor delay or <$5,000 cost |
| 2 | Minor | Partial data quality issue or $5K–$25K cost |
| 3 | Moderate | Monitoring period invalidation or $25K–$100K cost |
| 4 | Major | Project registration failure or $100K–$500K cost |
| 5 | Critical | Regulatory action, reputational destruction, or >$500K cost |

**Risk Score = Likelihood × Impact**. Scores ≥12 require mandatory mitigation. Scores ≥16 require executive escalation.

---

## 2. Technical & Data Risks

### R-001: Data Pipeline Failure

| Attribute | Detail |
|---|---|
| **Risk** | Green Button API, utility data feeds, or IoT integrations fail or become unavailable |
| **Likelihood** | 3 (Possible) |
| **Impact** | 4 (Major) |
| **Score** | **12** |
| **Owner** | Platform Operator |
| **Mitigation** | Multi-source fallback (API → monthly bill upload → manual entry with higher uncertainty discount). Maintain backup ingestion pathways for all data sources. |
| **Residual Risk** | Data quality degrades to RECEIPT_BACKED or USER_ENTERED tier, increasing uncertainty discounts by 10–45 percentage points. |

### R-002: Calculation Engine Error

| Attribute | Detail |
|---|---|
| **Risk** | Bug in Rust integer arithmetic, emission factor registry, or reduction calculation logic |
| **Likelihood** | 2 (Unlikely) |
| **Impact** | 4 (Major) |
| **Score** | **8** |
| **Owner** | Platform Operator |
| **Mitigation** | Comprehensive unit test suite. 15% blinded-sample independent verification. All calculations use checked arithmetic (`checked_mul`, `saturating_sub`) to prevent silent overflow. |
| **Residual Risk** | Undiscovered edge cases in complex multi-factor calculations. Mitigated by mandatory code review and test-vector publication. |

### R-003: Evidence Hash Collision or Tampering

| Attribute | Detail |
|---|---|
| **Risk** | SHA-256 hash collision exploited to substitute fraudulent evidence |
| **Likelihood** | 1 (Rare) |
| **Impact** | 5 (Critical) |
| **Score** | **5** |
| **Owner** | Platform Operator |
| **Mitigation** | SHA-256 collision resistance is computationally infeasible with current technology. Merkle tree root anchoring on-chain provides additional tamper evidence. All evidence records include timestamp and attester signature. |

### R-004: Data Completeness Below Threshold

| Attribute | Detail |
|---|---|
| **Risk** | Participant premises fall below 90% data completeness requirement |
| **Likelihood** | 3 (Possible) |
| **Impact** | 3 (Moderate) |
| **Score** | **9** |
| **Owner** | Platform Operator + Participant |
| **Mitigation** | Weekly automated completeness monitoring. Proactive outreach to participants with gaps. Allow billing-data backfill for months with missing interval data. |

---

## 3. Methodology & Crediting Risks

### R-005: Methodology Inapplicability

| Attribute | Detail |
|---|---|
| **Risk** | Selected CDM/VCS methodology deemed inapplicable to aggregated household interventions by VVB or registry |
| **Likelihood** | 3 (Possible) |
| **Impact** | 5 (Critical) |
| **Score** | **15** |
| **Owner** | Methodology Advisor + Project Proponent |
| **Mitigation** | Engage methodology advisor early in pilot phase for formal applicability assessment. Maintain alternative methodology mapping (AMS-II.C, AMS-II.E, VM0038). Consider new methodology development pathway if no existing methodology is applicable. |
| **Residual Risk** | New methodology development costs $100K–$300K and takes 12–24 months. |

### R-006: Additionality Challenge

| Attribute | Detail |
|---|---|
| **Risk** | VVB or registry determines that interventions fail the additionality test (common practice exceeds 50%, no financial barrier) |
| **Likelihood** | 3 (Possible) |
| **Impact** | 5 (Critical) |
| **Score** | **15** |
| **Owner** | Methodology Advisor |
| **Mitigation** | Pre-validate additionality with census data, ENERGY STAR market reports, and Georgia Power DSM enrollment statistics. Document financial and information barriers with participant surveys. Target underserved communities where adoption rates are demonstrably lower. |

### R-007: Baseline Manipulation or Gaming

| Attribute | Detail |
|---|---|
| **Risk** | Participants artificially inflate baseline consumption to maximize reduction calculations |
| **Likelihood** | 2 (Unlikely) |
| **Impact** | 4 (Major) |
| **Score** | **8** |
| **Owner** | Platform Operator |
| **Mitigation** | Baselines sourced from utility-provided interval or billing data (not self-reported). Automated anomaly detection flags consumption >3σ from peer cohort. Weather normalization removes seasonal variability. |

### R-008: Leakage Underestimation

| Attribute | Detail |
|---|---|
| **Risk** | Actual rebound effect or upstream displacement exceeds default leakage factors |
| **Likelihood** | 2 (Unlikely) |
| **Impact** | 3 (Moderate) |
| **Score** | **6** |
| **Owner** | Methodology Advisor |
| **Mitigation** | Conservative default leakage factors (5% rebound + 2% upstream). Literature-based justification documented in methodology. VVB may adjust upward during verification. |

---

## 4. Regulatory & Legal Risks

### R-009: FTC Green Guides Enforcement Action

| Attribute | Detail |
|---|---|
| **Risk** | Platform marketing language construed as unsubstantiated environmental benefit claims |
| **Likelihood** | 2 (Unlikely) |
| **Impact** | 5 (Critical) |
| **Score** | **10** |
| **Owner** | Project Proponent + Legal Counsel |
| **Mitigation** | All public-facing copy uses the canonical product claim: "ACNC produces evidence-grade impact records and registry-ready carbon-accounting packages. Carbon credits are issued only by an authorized registry after third-party validation and verification." No claim of personal carbon credits, offsets, or environmental benefits until registry-verified. FTC compliance review before any public marketing. |

### R-010: Securities Law Classification of VTIME

| Attribute | Detail |
|---|---|
| **Risk** | VTIME is classified as a security, investment contract, or commodity by US SEC/CFTC |
| **Likelihood** | 1 (Rare) |
| **Impact** | 5 (Critical) |
| **Score** | **5** |
| **Owner** | Legal Counsel |
| **Mitigation** | VTIME is explicitly designed as a non-transferable, non-redeemable, closed-loop utility credit with no monetary value, no exchange listing, and no investment return expectation. Howey test analysis documented. Legal opinion obtained before public launch. |

### R-011: Data Privacy Breach

| Attribute | Detail |
|---|---|
| **Risk** | Unauthorized disclosure of participant PII, utility consumption data, or address information |
| **Likelihood** | 2 (Unlikely) |
| **Impact** | 4 (Major) |
| **Score** | **8** |
| **Owner** | Platform Operator |
| **Mitigation** | Hash-first architecture: raw PII replaced with SHA-256 `subject_id_hash` at ingestion. No plain-text addresses stored after enrollment verification. Secret scanning workflow (Gitleaks) in CI/CD pipeline. Minimum-privilege access controls on data stores. |

---

## 5. Market & Commercial Risks

### R-012: Credit Price Volatility

| Attribute | Detail |
|---|---|
| **Risk** | VCM credit prices decline below project break-even, making verification costs unrecoverable |
| **Likelihood** | 3 (Possible) |
| **Impact** | 3 (Moderate) |
| **Score** | **9** |
| **Owner** | Project Proponent |
| **Mitigation** | Do not pre-sell or forward-commit credits. Pilot budget funded independently of credit revenue. VTIME utility rewards are decoupled from credit prices. Long-term crediting period (10+ years) provides price averaging. |

### R-013: Registry Policy Change

| Attribute | Detail |
|---|---|
| **Risk** | Verra or Gold Standard changes methodology requirements, grouped-project rules, or fee structures mid-program |
| **Likelihood** | 3 (Possible) |
| **Impact** | 3 (Moderate) |
| **Score** | **9** |
| **Owner** | Project Proponent + Methodology Advisor |
| **Mitigation** | Monitor registry policy updates. Maintain methodology flexibility (multiple candidate methodologies mapped). Budget for methodology revision costs. |

### R-014: Insufficient Participant Enrollment

| Attribute | Detail |
|---|---|
| **Risk** | Pilot fails to reach minimum viable cohort of 50 households |
| **Likelihood** | 3 (Possible) |
| **Impact** | 3 (Moderate) |
| **Score** | **9** |
| **Owner** | Platform Operator |
| **Mitigation** | Partner with local weatherization nonprofits, community organizations, and utility rebate programs. Offer energy audit subsidies as enrollment incentive. Expand geographic scope to additional Georgia counties if needed. |

---

## 6. Reversal & Permanence Risks

### R-015: Participant Reversal (Equipment Removal or Degradation)

| Attribute | Detail |
|---|---|
| **Risk** | Enrolled participant removes qualifying equipment, sells property, or equipment degrades below efficiency threshold |
| **Likelihood** | 2 (Unlikely) |
| **Impact** | 3 (Moderate) |
| **Score** | **6** |
| **Owner** | Project Proponent |
| **Mitigation** | Participant agreement includes minimum equipment retention period (5 years). Buffer pool allocation (5–15% of gross reductions) covers reversal risk. Annual monitoring detects consumption reversion. Property sale triggers re-enrollment or pro-rated credit adjustment. |

### R-016: Grid Decarbonization Erosion

| Attribute | Detail |
|---|---|
| **Risk** | Utility grid emission factor declines over crediting period, reducing the marginal value of electricity-efficiency interventions |
| **Likelihood** | 4 (Likely) |
| **Impact** | 2 (Minor) |
| **Score** | **8** |
| **Owner** | Methodology Advisor |
| **Mitigation** | Emission factors are updated annually per the latest EPA eGRID data. Reduction calculations use the monitoring-period emission factor (not the baseline-period factor), accurately reflecting the marginal emission impact. This is a feature of conservative quantification, not a risk to be eliminated. |

---

## 7. Risk Heat Map Summary

| Score Range | Count | Category |
|---|---|---|
| **15–25 (Critical)** | 2 | R-005 (Methodology), R-006 (Additionality) |
| **12–14 (High)** | 1 | R-001 (Data Pipeline) |
| **8–11 (Medium)** | 7 | R-002, R-004, R-007, R-009, R-011, R-012, R-013, R-014, R-016 |
| **1–7 (Low)** | 4 | R-003, R-008, R-010, R-015 |

### Priority Actions

1. **Engage methodology advisor immediately** to de-risk R-005 and R-006 before significant pilot investment.
2. **Secure Green Button API access with Georgia Power** to de-risk R-001.
3. **Obtain FTC compliance and securities law opinions** to de-risk R-009 and R-010.
4. **Complete Rust test-vector suite** to further de-risk R-002.

---

## 8. Risk Review Schedule

| Review Event | Frequency | Responsible |
|---|---|---|
| Full risk register review | Quarterly | Project Proponent + Platform Operator |
| Methodology risk assessment | At each VVB engagement milestone | Methodology Advisor |
| Data quality risk assessment | Monthly (during monitoring) | Platform Operator |
| Regulatory landscape scan | Semi-annually | Legal Counsel |
| Post-incident review | Within 48 hours of any materialized risk | Affected risk owner |

---

*Document prepared by UnyKorn LLC — ACNC Climate Projects Division.*
