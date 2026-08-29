# ACNC MRV Method & Data Governance (v0.1)

## Document Control

| Field | Value |
|---|---|
| **Document ID** | `ACNC-MRV-2026-001` |
| **Version** | `0.1 — DRAFT` |
| **Classification** | Internal / Technical Architecture for Partner & Methodology-Advisor Review |
| **Date** | 2026-08-29 |

---

> [!IMPORTANT]
> **Research and pilot status — not a crediting offer.**
> This documentation authorizes research, software development, partner diligence, and a controlled pilot only. ACNC does not issue, sell, market, tokenize, promise, or represent carbon credits or offsets. Any future issuance depends on an eligible project, applicable methodology, independent validation and verification, registry approval, and all applicable legal, contractual, consumer-protection, privacy, and environmental-claim requirements.

---

## 1. Non-Negotiable Data & Claims Controls

| Control | Requirement |
|---|---|
| **Baseline** | Defined before or independently from the intervention; methodology-specific. |
| **Additionality** | Determined through applicable program rules and expert review, not an app checkbox. |
| **Double counting** | Screen utility incentives, RECs, other carbon programs, grants, and prior claims. |
| **Evidence** | Hash documents for integrity, but retain auditable originals under controlled access. |
| **Privacy** | Keep personal data and bills off-chain; store only necessary hashes/identifiers on-chain. |
| **Uncertainty** | Apply conservative methodology-approved deductions, not a universal discount. |
| **Versioning** | Freeze factor, policy, methodology, software, and evidence-schema versions per report. |
| **Registry status** | Registry remains the sole authority for issued/retired serialized units. |
| **Claims** | Public statements distinguish estimated, attested, verified, issued, and retired states. |

> [!NOTE]
> **Integrity vs. Truth:** A SHA-256 hash and Merkle root establish record integrity, but they do not prove that a bill, retrofit, meter reading, or claimed reduction is true. Auditability requires secure original evidence, provenance, access controls, review procedures, and independent verification.

---

## 2. Review Status Progression Gate

```
DRAFT ──► EVIDENCE_COMPLETE ──► PENDING_INTERNAL_REVIEW ──► PENDING_INDEPENDENT_VALIDATION
  ──► VALIDATED ──► PENDING_VVB_VERIFICATION ──► VERIFIED_FOR_REGISTRY_SUBMISSION
  ──► REGISTRY_ISSUED (READ-ONLY: authorized exclusively by official registry confirmation)
```

### Role-Based Transition Authority

| Transition | Authorized Role |
|---|---|
| DRAFT → EVIDENCE_COMPLETE | Automated (schema + completeness check) |
| EVIDENCE_COMPLETE → PENDING_INTERNAL_REVIEW | Platform Operator |
| PENDING_INTERNAL_REVIEW → PENDING_INDEPENDENT_VALIDATION | Methodology Advisor |
| PENDING_INDEPENDENT_VALIDATION → VALIDATED | Methodology Advisor |
| VALIDATED → PENDING_VVB_VERIFICATION | Project Proponent |
| PENDING_VVB_VERIFICATION → VERIFIED_FOR_REGISTRY_SUBMISSION | VVB (external) |
| VERIFIED_FOR_REGISTRY_SUBMISSION → REGISTRY_ISSUED | **Carbon Registry (external; read-only ingest)** |

---

## 3. Data Classification & Privacy

1. **Pseudonymous by default:** Raw PII is hashed immediately. The system operates on `participant_id_hash: [u8; 32]`. Formal KYC is collected only where a registry, payment, anti-fraud, or benefit-sharing requirement explicitly mandates it.
2. **Off-Chain Data Security:** Utility bills, receipts, and contractor forms are retained off-chain under encrypted, access-controlled storage. Only cryptographic digests are referenced on-chain.

---

*Document prepared by UnyKorn LLC — ACNC Climate Projects Division.*
