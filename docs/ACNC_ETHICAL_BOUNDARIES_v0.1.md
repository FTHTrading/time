# ACNC Ethical Boundaries (v0.1)

## Document Control

| Field | Value |
|---|---|
| **Document ID** | `ACNC-ETH-2026-001` |
| **Version** | `0.1 — DRAFT` |
| **Classification** | Public / Governance |
| **Date** | 2026-08-29 |

---

> [!CAUTION]
> **INVIOLABLE ETHICAL PRINCIPLE:** No human being may ever be charged, scored, penalized, taxed, metered, or negatively assessed for the biological, metabolic, or physiological processes required for life. This principle is algorithmically enforced and cannot be overridden by configuration, policy amendment, or administrative action.

---

## 1. Purpose

This document establishes the non-negotiable ethical boundaries for the ACNC Human Footprint Reduction & Verified Climate Action Program. These boundaries define what the platform will never measure, score, or penalize, regardless of methodological applicability, market demand, or regulatory change.

---

## 2. Absolutely Excluded Domains

The following categories are permanently excluded from all measurement, scoring, reward calculation, penalty assessment, or carbon quantification within the ACNC system:

### 2.1 Biological Human Functions

| Excluded Category | Rationale |
|---|---|
| **Respiration / breathing** | Biological necessity; CO2 exhaled is biogenic and part of the short carbon cycle |
| **Metabolic rate / basal metabolism** | Involuntary physiological process; varies by age, sex, genetics, and health |
| **Oxygen consumption** | Fundamental to human life; cannot be voluntarily reduced |
| **Sleep and rest** | Essential biological recovery; not a discretionary emission source |
| **Body temperature regulation** | Involuntary thermoregulation; not controllable by individual decision |
| **Cardiac and circulatory function** | Involuntary life-sustaining process |

### 2.2 Health, Disability, and Medical Needs

| Excluded Category | Rationale |
|---|---|
| **Medical conditions and treatments** | Health needs are involuntary and protected by law |
| **Disability-related energy use** | Adaptive equipment (motorized wheelchairs, oxygen concentrators, ventilators) is medically necessary |
| **Medication and pharmaceutical needs** | Health requirements are not discretionary emissions |
| **Hospitalization and medical facility use** | Medical care decisions are outside individual emission control |
| **Mental health conditions** | Behavioral patterns associated with mental health are not scorable |

### 2.3 Basic Subsistence Needs

| Excluded Category | Rationale |
|---|---|
| **Basic caloric intake / dietary subsistence** | Minimum nutrition is a fundamental human right (UDHR Art. 25) |
| **Potable water consumption** | Drinking water is a fundamental human right (UN GA Res. 64/292) |
| **Basic shelter and heating/cooling for health** | Maintaining survivable indoor temperatures is not discretionary |
| **Sanitation and hygiene** | Basic hygiene is a public health requirement |

### 2.4 Social Scoring and Behavioral Surveillance

| Excluded Category | Rationale |
|---|---|
| **Unverified "good behavior" surveys** | Subjective self-assessment has no evidential weight |
| **Social media activity or online presence** | Not correlated with measurable emission reductions |
| **Lifestyle scoring or personal habit tracking** | Creates social-credit-system dynamics; violates privacy |
| **Family size, reproductive choices** | Deeply personal decisions protected by fundamental rights |
| **Political beliefs or civic participation** | Constitutional protections; irrelevant to emission quantification |
| **Religious or cultural practices** | Protected under freedom of religion and expression |

---

## 3. Algorithmic Enforcement

### 3.1 Category Rejection

The ACNC ingestion pipeline includes a hard-coded category validator that rejects any `ActivityRecord` submitted with a category matching an excluded domain. This validator cannot be disabled by configuration:

```
EXCLUDED_CATEGORIES = [
    "respiration", "breathing", "metabolic_rate", "oxygen_consumption",
    "sleep", "body_temperature", "cardiac_function", "medical_treatment",
    "disability_equipment", "medication", "hospitalization", "mental_health",
    "caloric_intake", "drinking_water", "basic_shelter", "sanitation",
    "social_behavior", "social_media", "lifestyle_score", "family_size",
    "reproductive_choice", "political_belief", "religious_practice"
]
```

### 3.2 Audit Trail

Every rejected submission is logged in the `PolicyDecisionLog` with:
- Rule name: `ETHICAL_BOUNDARY_VIOLATION`
- Status: `REJECTED`
- Reason: Specific excluded category identified

### 3.3 Immutability

This exclusion list may only be expanded (new categories added to the excluded set). Categories may never be removed from the exclusion list. This constraint is documented in the CONTRIBUTING.md governance rules.

---

## 4. Eligible vs. Ineligible — Decision Framework

```
┌───────────────────────────────────────────────────────────┐
│  IS THE ACTIVITY VOLUNTARY AND CONTROLLABLE?              │
│                                                           │
│  YES ──► Is there a measurable, documented emission       │
│          reduction against a defensible baseline?         │
│          │                                                │
│          YES ──► ELIGIBLE for evidence capture and        │
│                  conservative quantification              │
│          │                                                │
│          NO ──► NOT ELIGIBLE (no measurable reduction)    │
│                                                           │
│  NO ──► ABSOLUTELY EXCLUDED                               │
│         (biological, medical, subsistence, or social)     │
└───────────────────────────────────────────────────────────┘
```

---

## 5. Relationship to Carbon Crediting Standards

The ICVCM Core Carbon Principles require that credited emission reductions be:

1. **Real** — Representing actual measured changes, not biological processes.
2. **Additional** — Beyond what would have occurred without the project intervention.
3. **Measurable** — Quantifiable against a defensible baseline.
4. **Permanent or reversal-managed** — Not subject to involuntary reversal.

Human biological functions fail criteria 1, 2, 3, and 4. They are not controllable interventions, cannot be additional to a baseline, are not meaningfully measurable as "reductions," and are inherently non-permanent (they continue as long as the person lives). Excluding them is not merely an ethical choice — it is a technical and methodological requirement.

---

## 6. Regulatory Alignment

### 6.1 FTC Green Guides (16 CFR Part 260)

The FTC's Guides for the Use of Environmental Marketing Claims prohibit:
- Unqualified environmental benefit claims that are not substantiated by competent and reliable evidence.
- Claims that overstate the environmental benefit of a product or service.
- Carbon offset claims that do not meet requirements for third-party certification and verification.

ACNC's ethical boundaries ensure compliance by preventing any claim that could be interpreted as scoring or crediting involuntary biological processes as "emission reductions."

### 6.2 Universal Declaration of Human Rights

ACNC's exclusions align with:
- **Article 3:** Right to life, liberty, and security of person.
- **Article 25:** Right to a standard of living adequate for health and well-being, including food, clothing, housing, and medical care.

---

## 7. Amendment Procedure

1. This document may be amended only by unanimous written approval of the ACNC Governance Board.
2. Amendments may only expand the list of excluded categories, never reduce it.
3. All amendments must include a written ethical justification.
4. The amendment history is maintained in the repository version control system.

---

*Document prepared by UnyKorn LLC — ACNC Climate Projects Division.*
