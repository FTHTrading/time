# Product Status & Environment Verification

This document outlines the exact operational readiness and environment status of each subsystem in **ALL COUCH NO CAGE**.

---

## 1. Status Taxonomy

| Color | Status | Definition |
|---|---|---|
| 🟢 Emerald (`#22C55E`) | `LIVE` | Operating publicly in its stated production environment |
| 🔵 Cyan (`#22D3EE`) | `LOCAL` | Client-only; runs in browser JavaScript / localStorage |
| 🔷 Blue (`#3B82F6`) | `TESTNET` | Deployed and testable on Polygon Amoy (Chain ID 80002) |
| 🟣 Violet (`#8B5CF6`) | `DEMO` | Prototype simulation; no authoritative state |
| 🟡 Amber (`#F59E0B`) | `PENDING` | Specification complete; waiting for service integration |
| ⚪ Slate (`#64748B`) | `NO DATA` | Default honest zero-state |
| 🔴 Red (`#EF4444`) | `REJECTED` | Ineligible or fraudulent claim |
| 🟡 Gold (`#D4AF37`) | `REGISTRY VERIFIED` | Matched to authoritative carbon registry certificate |

---

## 2. Component Inventory

| Component | Target Environment | Status | Verification Reference |
|---|---|---|---|
| **Overview & Assessment Starter** | GitHub Pages (`/`) | `LIVE` | Client router & zero-state engine |
| **Consumption & Waste Forms** | Browser (`/measure.html`) | `LOCAL` | Disclosed factor model & SHA-256 |
| **Reduction Comparison Engine** | Browser (`/reduce.html`) | `LOCAL` | Baseline delta math |
| **Registry Offset Validator** | Backend Integration (`/offsets.html`) | `PENDING` | Gold Standard & Verra adapters |
| **Reward Policy Engine** | Browser (`/rewards.html`) | `LOCAL` | Deterministic equation calculator |
| **Cryptographic Receipt Vault** | Browser / EVM (`/vault.html`) | `LOCAL` / `TESTNET` | JSON export & Amoy contract specs |
| **Milestone Relic Forge** | Browser (`/relics.html`) | `DEMO` | Salvador Dali artwork renderer |
| **Protocol & Schemas** | Static Documentation (`/protocol.html`) | `LIVE` | Published JSON schemas |
| **Smart Contract Stack** | Polygon Amoy Testnet | `TESTNET` | `0x4E574939D460d284B5D990646D4aeaEF2D49Fa13` |
