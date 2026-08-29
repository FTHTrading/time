# ACNC MRV Method & Data Governance (v0.1)

## Document Control

| Field | Value |
|---|---|
| **Document ID** | `ACNC-MRV-2026-001` |
| **Version** | `0.1 — DRAFT` |
| **Classification** | Internal / Technical Architecture |
| **Date** | 2026-08-29 |

---

> [!IMPORTANT]
> **SCOPE:** This document defines the technical architecture, data governance policies, and quality-assurance procedures for ACNC's digital Measurement, Reporting, and Verification (dMRV) system. It does not authorize credit issuance.

---

## 1. System Architecture Overview

### 1.1 Component Map

```
┌─────────────────────────────────────────────────────────────────────┐
│  DATA INGESTION LAYER                                               │
│                                                                     │
│  Green Button API ──┐                                               │
│  Smart Meter AMI ───┤                                               │
│  Utility Invoice ───┤──► Canonical JSON Parser ──► SHA-256 Hasher   │
│  IoT Sensor Feed ───┤                                               │
│  Contractor Report ─┘                                               │
└───────────────────────────────┬─────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────────┐
│  EVIDENCE LAYER (acnc-evidence)                                     │
│                                                                     │
│  canonical_json.rs: Deterministic key-sorted JSON bytes             │
│  hash.rs: SHA-256 content-addressed evidence digests                │
│  receipts.rs: Sealed evidence receipts with timestamp               │
│  signatures.rs: EIP-712 structured claim payloads                   │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────┐        │
│  │  Merkle Tree                                            │        │
│  │  Leaf: SHA-256(canonical_json(ActivityRecord))          │        │
│  │  Root: Evidence root hash for monitoring period         │        │
│  └─────────────────────────────────────────────────────────┘        │
└───────────────────────────────┬─────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────────┐
│  CALCULATION LAYER (acnc-carbon-core)                               │
│                                                                     │
│  factors.rs: Published emission factor registry (EPA, eGRID, IPCC)  │
│  footprint.rs: Activity-to-CO2e integer conversions                 │
│  baseline.rs: Historical window & weather normalization             │
│  reduction.rs: Conservative reduction = max(0, B-O-L) * (1-U/10k)  │
│  uncertainty.rs: Evidence-tier uncertainty deduction schedule        │
│  caps.rs: Daily and category issuance limits                        │
└───────────────────────────────┬─────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────────┐
│  GOVERNANCE LAYER (acnc-policy + acnc-project-mrv)                  │
│                                                                     │
│  eligibility.rs: Evidence-tier reward eligibility gates              │
│  rewards.rs: VTIME integer calculation with account caps            │
│  anti_fraud.rs: Duplicate evidence hash detection                   │
│  audit_log.rs: Append-only policy decision records                  │
│  double_counting.rs: Serial and premise anti-duplication            │
│  additionality.rs: Checklist compilation (not automated ruling)     │
│  monitoring.rs: Period aggregation and reporting                    │
│  permanence.rs: Reversal risk buffer pool deductions                │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 2. Data Classification & Sensitivity

### 2.1 Data Categories

| Category | Examples | Sensitivity | Retention |
|---|---|---|---|
| **Participant Identity** | Name, address, utility account | HIGH — PII | Hashed; raw data deleted after enrollment verification |
| **Premise Identifier** | Utility premise ID, tax parcel | HIGH — PII | Hashed; used only for de-duplication |
| **Consumption Data** | kWh, therms, interval readings | MEDIUM — proprietary | Retained for crediting period + 7 years |
| **Intervention Records** | Equipment serial, contractor ID | MEDIUM | Retained for crediting period + 7 years |
| **Evidence Hashes** | SHA-256 digests, Merkle roots | LOW | Retained indefinitely (content-addressed) |
| **Emission Factors** | EPA eGRID factors, versions | PUBLIC | Published openly |
| **Calculation Outputs** | FootprintEstimate, ReductionCandidate | MEDIUM | Retained for crediting period + 7 years |

### 2.2 Data Minimization Principles

1. **Hash-first architecture:** Raw PII is immediately hashed at ingestion. The ACNC platform stores `subject_id_hash: [u8; 32]`, never plain-text names or addresses.
2. **Purpose limitation:** Consumption data is used exclusively for emission quantification and monitoring report generation.
3. **No secondary monetization:** Participant data is never sold, shared for advertising, or disclosed to third parties without explicit consent.

---

## 3. Ingestion Pipeline Specifications

### 3.1 Supported Data Sources

| Source | Protocol | Authentication | Data Format |
|---|---|---|---|
| Green Button Connect My Data (CMD) | OAuth 2.0 / REST API | Utility-issued token | ESPI XML (Atom) |
| Green Button Download My Data (DMD) | File upload | N/A | ESPI XML or CSV |
| Utility Monthly Bill | Manual entry / OCR | N/A | Structured form |
| Smart Thermostat API | OAuth 2.0 | Manufacturer token | JSON |
| Contractor Commissioning | Structured form + photo | Digital signature | JSON + JPEG |
| Certified Scale Ticket | Structured form + scan | Weighmaster signature | JSON + PDF |

### 3.2 Ingestion Validation Rules

Each ingested record must pass:

1. **Schema Validation:** Conforms to the applicable JSON schema in `packages/schemas/`.
2. **Temporal Bounds:** `period_start < period_end` and period falls within a valid baseline or monitoring window.
3. **Unit Validation:** Quantity and unit are consistent with the stated category (e.g., kWh for electricity, therms for gas).
4. **Duplicate Detection:** SHA-256 hash of `(premise_id_hash, category, period_start, period_end, quantity)` must not match any existing record in the `AntiFraudValidator`.
5. **Evidence Status Assignment:** Based on the data source provenance:

| Source | Assigned Evidence Status |
|---|---|
| Green Button API (interval) | `METERED` |
| Smart meter AMI feed | `METERED` |
| Utility monthly bill | `RECEIPT_BACKED` |
| Contractor commissioning | `ATTESTED` |
| Certified scale ticket | `ATTESTED` |
| Manual entry / survey | `USER_ENTERED` |
| Model / inference | `ESTIMATED` |

---

## 4. Calculation Reproducibility

### 4.1 Deterministic Integer Arithmetic

All calculations use `i64` integer types (GramsCo2e, BasisPoints). No floating-point operations are used in the quantification pipeline. This ensures:

- **Bit-exact reproducibility** across platforms, architectures, and compiler versions.
- **No rounding drift** across aggregation of thousands of participant records.
- **Auditor reproducibility** given the same inputs, factor version, and methodology version.

### 4.2 Calculation Trace

Every `FootprintEstimate` and `ReductionCandidate` output includes:

```json
{
  "factor_id": "electricity_us_grid_srso",
  "factor_version": "2026.1",
  "methodology_version": "AMS-II.C-v15",
  "evidence_status": "METERED",
  "uncertainty_discount_bps": 500,
  "calculation_engine": "acnc-carbon-core v0.1.0",
  "evidence_hash": "sha256:a1b2c3..."
}
```

### 4.3 Blinded Sample Verification

For each monitoring period, a minimum 15% random blinded sample of participant records is selected for independent recalculation. The independent reviewer must achieve 100% bitwise match with the ACNC engine output to confirm reproducibility.

---

## 5. Review Status Lifecycle

### 5.1 State Machine

```
DRAFT
  │ (participant submits monitoring data)
  ▼
EVIDENCE_COMPLETE
  │ (data completeness >= 90%, schema validated)
  ▼
PENDING_INTERNAL_REVIEW
  │ (ACNC methodology team reviews calculation, flags anomalies)
  ▼
PENDING_INDEPENDENT_VALIDATION
  │ (independent methodology advisor sign-off)
  ▼
VALIDATED
  │ (aggregated into monitoring report)
  ▼
PENDING_VVB_VERIFICATION
  │ (submitted to accredited VVB)
  ▼
VERIFIED_FOR_REGISTRY_SUBMISSION
  │ (VVB issues positive verification report)
  ▼
REGISTRY_ISSUED
  │ (registry approves and issues serialized credits)
  ▼
[PROJECT-LEVEL BENEFIT SHARING]
```

### 5.2 Role-Based Transition Authority

| Transition | Authorized Role |
|---|---|
| DRAFT → EVIDENCE_COMPLETE | Automated (schema + completeness check) |
| EVIDENCE_COMPLETE → PENDING_INTERNAL_REVIEW | Platform Operator |
| PENDING_INTERNAL_REVIEW → PENDING_INDEPENDENT_VALIDATION | Methodology Advisor |
| PENDING_INDEPENDENT_VALIDATION → VALIDATED | Methodology Advisor |
| VALIDATED → PENDING_VVB_VERIFICATION | Project Proponent |
| PENDING_VVB_VERIFICATION → VERIFIED_FOR_REGISTRY_SUBMISSION | VVB (external) |
| VERIFIED_FOR_REGISTRY_SUBMISSION → REGISTRY_ISSUED | Carbon Registry (external) |

### 5.3 Rejection & Remediation

At any stage, a record may transition to `REJECTED` with a documented reason code. Rejected records may be remediated (additional evidence, corrected data) and re-submitted starting from `DRAFT`.

---

## 6. Audit Trail & Record Retention

### 6.1 Append-Only Audit Log

Every state transition, calculation invocation, and policy decision is recorded in the `PolicyDecisionLog` with:
- `decision_id`: Unique identifier
- `target_id`: Record being acted upon
- `rule_name`: Policy rule or gate that triggered the decision
- `status`: `APPROVED`, `REJECTED`, `FLAGGED`
- `timestamp`: Unix timestamp

### 6.2 Retention Schedule

| Record Type | Minimum Retention |
|---|---|
| Participant consent records | Crediting period + 10 years |
| Monitoring data & evidence hashes | Crediting period + 7 years |
| Calculation outputs | Crediting period + 7 years |
| Audit logs | Crediting period + 10 years |
| VVB reports | Crediting period + 10 years |

### 6.3 Disaster Recovery

- Evidence hashes are anchored on-chain via `ImpactReceiptRegistry.sol` (Polygon Amoy, then mainnet).
- Complete data backups are maintained in geographically separated storage.
- Merkle roots provide tamper-evidence verification even if primary storage is compromised.

---

## 7. Data Subject Rights

Enrolled participants retain the following rights under ACNC's data governance policy:

1. **Right to access:** Request a complete export of all stored data associated with their `subject_id_hash`.
2. **Right to correction:** Request correction of erroneous consumption data (triggers re-calculation).
3. **Right to withdrawal:** Withdraw consent and request deletion of un-hashed data. Note: SHA-256 evidence hashes that have been submitted to a VVB or registry cannot be retroactively removed from the verification record.
4. **Right to data portability:** Export all records in standardized JSON format.

---

*Document prepared by UnyKorn LLC — ACNC Climate Projects Division.*
