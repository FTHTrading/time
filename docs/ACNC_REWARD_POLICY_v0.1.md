# ACNC Reward Policy (v0.1)

## Document Control

| Field | Value |
|---|---|
| **Document ID** | `ACNC-RWD-2026-001` |
| **Version** | `0.1 — DRAFT` |
| **Classification** | Internal / Platform Economics |
| **Date** | 2026-08-29 |

---

> [!IMPORTANT]
> **STATUTORY NOTICE:** VTIME is a closed-loop, non-transferable internal platform utility credit. It is not a carbon credit, carbon offset, security, commodity, investment contract, or guaranteed payout. VTIME does not represent ownership of, or claim to, any environmental attribute, serialized carbon unit, or registry-issued credit. Carbon credits are issued exclusively by authorized registries after independent validation and verification.

---

## 1. Reward Philosophy

### 1.1 Core Principles

1. **Evidence precedes reward.** No VTIME is issued without a validated evidence record meeting minimum provenance thresholds.
2. **Rewards are not carbon credits.** VTIME utility is strictly separated from the carbon-crediting lifecycle. Participants who receive VTIME for validated efficiency improvements are not receiving carbon credits.
3. **Conservative by default.** Uncertainty discounts and category caps prevent over-rewarding. The system errs on the side of under-crediting.
4. **Transparent calculation.** Every VTIME issuance includes a full calculation trace with evidence hash, factor version, methodology version, and multiplier breakdown.

### 1.2 What VTIME Can Be Used For

- Access to advanced energy-usage analytics and personalized reduction recommendations.
- Partner hardware and service discounts (e.g., ENERGY STAR appliance rebate aggregation).
- Priority access to future ACNC program features.
- Community leaderboard participation (anonymized and opt-in only).

### 1.3 What VTIME Cannot Be Used For

- Redemption for cash, cryptocurrency, or any monetary equivalent.
- Transfer to another user or external wallet.
- Representation as a carbon credit, offset, or environmental attribute.
- Collateralization, lending, or use as an investment instrument.

---

## 2. Event-to-Reward Matrix

### 2.1 Evidence Tier Multipliers

| Evidence / Verification Event | Evidence Status | Multiplier (BPS) | Effective Rate |
|---|---|---|---|
| Unverified submission | `UNVERIFIED` | 0 | 0% — No reward |
| Self-reported activity log | `USER_ENTERED` | 2,000 | 20% |
| Invoice or receipt attached | `RECEIPT_BACKED` | 8,000 | 80% |
| Utility meter data confirmed | `METERED` | 10,000 | 100% |
| Licensed contractor attestation | `ATTESTED` | 10,000 | 100% |
| Registry-verified retirement | `REGISTRY_VERIFIED` | 10,000 | 100% |

### 2.2 VTIME Calculation Formula

$$\text{VTIME (micro-units)} = \min\left(\frac{\text{Reduction (grams CO2e)}}{1{,}000} \times 250{,}000 \times \frac{\text{Multiplier BPS}}{10{,}000},\ \text{Category Cap},\ \text{Daily Cap} - \text{Daily Issued}\right)$$

Where:
- 1 kg CO2e reduction at 100% multiplier = 250,000 micro-VTIME (0.25 VTIME)
- 1,000,000 micro-VTIME = 1.00 VTIME

### 2.3 Worked Examples

**Example 1: Metered Electricity Reduction**
- Reduction: 500 kg CO2e (metered via Green Button API)
- Evidence Status: `METERED` (multiplier = 10,000 bps = 100%)
- Base reward: 500 × 250,000 = 125,000,000 micro-VTIME (125.00 VTIME)
- Daily cap check: 125.00 < 200.00 → Issued: **125.00 VTIME**

**Example 2: Self-Reported Transit Shift**
- Reduction: 100 kg CO2e (user-entered trip log)
- Evidence Status: `USER_ENTERED` (multiplier = 2,000 bps = 20%)
- Base reward: 100 × 250,000 × 2,000 / 10,000 = 5,000,000 micro-VTIME (5.00 VTIME)
- Issued: **5.00 VTIME** (educational/provisional only)

**Example 3: Cap-Limited High-Volume Day**
- Reduction: 2,000 kg CO2e (attested heat pump retrofit)
- Evidence Status: `ATTESTED` (multiplier = 10,000 bps = 100%)
- Base reward: 2,000 × 250,000 = 500,000,000 micro-VTIME (500.00 VTIME)
- Daily cap: 200,000,000 micro-VTIME (200.00 VTIME)
- Issued: **200.00 VTIME** (cap applied)

---

## 3. Issuance Caps

### 3.1 Cap Schedule

| Cap Type | Limit (micro-VTIME) | Limit (VTIME) | Scope |
|---|---|---|---|
| Daily Account Cap | 200,000,000 | 200.00 | Per participant per calendar day |
| Daily Focus Cap | 50,000,000 | 50.00 | Focus-session category only |
| Daily Impact Cap | 50,000,000 | 50.00 | Impact-activity category only |
| Single-Event Cap | 500,000,000 | 500.00 | Any single evidence record |
| Monthly Account Cap | 3,000,000,000 | 3,000.00 | Per participant per calendar month |

### 3.2 Cap Rationale

Caps prevent gaming, bot-driven evidence flooding, and over-concentration of utility rewards. They are calibrated to be generous enough for legitimate high-activity participants while preventing abuse.

---

## 4. Reward Lifecycle & Clawback

### 4.1 Issuance States

| State | Description |
|---|---|
| `PROVISIONAL` | Issued upon initial evidence validation. May be adjusted if evidence is later downgraded or rejected. |
| `CONFIRMED` | Locked after independent review passes and monitoring period closes. |
| `CLAWED_BACK` | Reversed due to evidence rejection, duplicate detection, or policy violation. |

### 4.2 Clawback Triggers

- Evidence record transitions to `REJECTED` status.
- Duplicate evidence hash detected by `AntiFraudValidator`.
- Participant enrolled in overlapping program making the same environmental attribute claim.
- Calculation error discovered during blinded-sample audit.

### 4.3 Clawback Procedure

1. Affected record is flagged and linked to the clawback decision in the `PolicyDecisionLog`.
2. VTIME balance is debited by the originally credited amount.
3. Participant is notified with the reason code and evidence reference.
4. Participant may contest by submitting corrected evidence, restarting at `DRAFT`.

---

## 5. Relationship to Carbon Credits

### 5.1 Strict Separation

```
┌──────────────────────────────────┐     ┌──────────────────────────────────┐
│  VTIME UTILITY REWARD TRACK      │     │  CARBON CREDIT TRACK             │
│                                  │     │                                  │
│  Issued by: ACNC Platform        │     │  Issued by: Authorized Registry  │
│  Authority: Platform policy      │     │  Authority: VVB + Registry rules │
│  Nature: Internal utility credit │     │  Nature: Serialized carbon unit   │
│  Transferable: No                │     │  Transferable: Per registry rules │
│  Environmental claim: None       │     │  Environmental claim: Yes         │
│  Backed by: Evidence records     │     │  Backed by: Verified reductions   │
│                                  │     │                                  │
│  THESE ARE SEPARATE SYSTEMS.     │     │  THESE ARE SEPARATE SYSTEMS.     │
│  VTIME ≠ Carbon Credit.         │     │  Credits ≠ VTIME.               │
└──────────────────────────────────┘     └──────────────────────────────────┘
```

### 5.2 Benefit Sharing (Post-Crediting)

If the aggregated project successfully receives registry-issued credits, a separate benefit-sharing agreement governs the distribution of credit proceeds to enrolled participants. This agreement is:
- Executed as a standalone legal document between the Project Proponent SPV and each participant.
- Independent of and in addition to any VTIME utility rewards.
- Subject to the credit sale proceeds actually received, net of VVB fees, registry fees, and project administration costs.

---

## 6. Governance & Amendment

- The Reward Policy may be amended by the Platform Operator with 30 days' advance notice to all enrolled participants.
- No amendment may retroactively reduce VTIME balances that have already transitioned to `CONFIRMED` status.
- Policy version history is maintained in the repository at `packages/policy/reward-policy.v1.json`.

---

*Document prepared by UnyKorn LLC — ACNC Climate Projects Division.*
