# ALL COUCH NO CAGE — Human Time & Impact Protocol

> **Measure what you use. Reduce what you can. Verify what you offset. Earn utility only from eligible, evidence-backed action.**

[![Status: FOUNDATION](https://img.shields.io/badge/Status-FOUNDATION-64748B.svg)](#system-status-indicators)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Deployment: GitHub_Pages](https://img.shields.io/badge/Deployment-GitHub_Pages-22D3EE.svg)](https://fthtrading.github.io/time/)

Evidence-first human time, resource-use, reduction, and verified-offset ledger. Private by default, built for transparent reward eligibility and optional on-chain utility claims.

---

## System Status Indicators

We maintain a strict, standardized status system across all documentation, interfaces, and contracts:

| Color | Hex | Status Label | Meaning & Operational Scope |
|---|---|---|---|
| 🟢 **Emerald** | `#22C55E` | `LIVE` | Publicly usable and verified in its stated environment |
| 🔵 **Cyan** | `#22D3EE` | `LOCAL` | Runs only in the user’s browser / client device |
| 🔷 **Blue** | `#3B82F6` | `TESTNET` | Deployed and usable on a non-production blockchain (Polygon Amoy) |
| 🟣 **Violet** | `#8B5CF6` | `DEMO` | Demonstration experience; no authoritative result |
| 🟡 **Amber** | `#F59E0B` | `PENDING` | Designed or queued but not operational |
| ⚪ **Slate** | `#64748B` | `NO DATA` | No user record or verified evidence exists yet |
| 🔴 **Red** | `#EF4444` | `REJECTED` | Failed validation, duplicate claim, expired proof, or policy violation |
| 🟡 **Gold** | `#D4AF37` | `REGISTRY VERIFIED` | Linked to an independently verifiable offset-retirement record |

> [!IMPORTANT]
> No metric or badge in this repository will ever display an "active", "pulsing", "minted", or "verified" state unless backed by a retrievable proof, cryptographic receipt, or on-chain transaction.

---

## Core Product Areas & Architecture

```
time/
├── apps/
│   └── web/                   # Static multi-page assessment & ledger application
│       ├── index.html         # Overview: Personal Footprint & Value Assessment
│       ├── measure.html       # Measure: Enter utility, transit, and positive activity
│       ├── reduce.html        # Reduce: Action plans & comparative baseline reduction logs
│       ├── offsets.html       # Offsets: Authoritative registry retirement validation
│       ├── rewards.html       # Rewards: Focus Points, Impact Points & VTIME calculation
│       ├── vault.html         # Vault: Web3 claims & cryptographic JSON audit receipts
│       ├── relics.html        # Relics: Milestone-backed sovereign achievement artifacts
│       ├── protocol.html      # Protocol: Published methodology & standards
│       └── assets/            # Modular CSS, JS modules, images, data
├── packages/
│   ├── schemas/               # Canonical JSON validation schemas
│   └── policy/                # Versioned reward & emissions factor models (v2026.1)
├── contracts/                 # Deterministic EVM smart contracts (Polygon Amoy)
├── services/
│   └── attester/              # Off-chain evidence & signature attestation service specs
└── docs/                      # Architectural, ethical, and deployment specifications
```

---

## Page Status Matrix

| Page | Launch Status | Primary Action |
|---|---|---|
| **Overview** (`/`) | `LIVE` | Begin Personal Impact Assessment |
| **Measure** (`/measure.html`) | `LOCAL` | Add a supported consumption or activity record |
| **Reduce** (`/reduce.html`) | `LOCAL` | Record verified reduction evidence against a baseline |
| **Offsets** (`/offsets.html`) | `PENDING` | Validate third-party registry retirement certificate |
| **Rewards** (`/rewards.html`) | `LOCAL` | Review eligible activity & claim calculations |
| **Vault** (`/vault.html`) | `LOCAL` | Export full JSON ledger / connect Web3 wallet |
| **Relics** (`/relics.html`) | `DEMO` | Preview milestone achievement artifacts |
| **Protocol** (`/protocol.html`) | `LIVE` | Inspect disclosed methodology and schemas |

---

## Reward Issuance Equation

Utility credits ($VTIME) are issued strictly after a record satisfies evidence thresholds:

$$\text{VTIME Issued} = \min( \text{base reward} \times \text{evidence multiplier} \times \text{policy multiplier}, \text{category cap}, \text{daily account cap} )$$

### Evidence Multipliers
- `UNVERIFIED`: `0.0×` (Personal record only)
- `USER_ENTERED`: `0.2×` (Provisional estimate)
- `ESTIMATED`: `0.3×` (Provisional factor calculation)
- `RECEIPT_BACKED`: `0.8×` (Statement / document attached)
- `METERED`: `1.0×` (Connected utility / provider integration)
- `ATTESTED`: `1.0×` (Partner / employer verified)
- `REGISTRY_VERIFIED`: `1.0×` (Third-party carbon retirement certificate)

---

## Ethical Ledger Boundary

Human existence is a positive source of value—**never a biological liability**.
As detailed in [`docs/ETHICAL_LEDGER.md`](docs/ETHICAL_LEDGER.md), no user is ever scored, charged, or penalized for breathing, respiration, metabolism, basal nourishment, sleep, bodily existence, health conditions, or disability.

---

## License & Security
- Code licensed under [MIT License](LICENSE).
- Security disclosures governed by [SECURITY.md](SECURITY.md).
