# Smart Contract Architecture & EIP-712 Claims

## 1. Overview
Smart contracts in this repository are designed for deterministic verification on EVM networks (Polygon Amoy Testnet, Chain ID 80002).

---

## 2. Contracts Inventory

### `TimeImpactLedger.sol`
- **Purpose**: On-chain anchor for cryptographic focus and impact receipt hashes.
- **Verification**: Tracks SHA-256 evidence digests and prevents duplicate claim replay.

### `VTIMERewardVault.sol`
- **Purpose**: Non-custodial utility vault validating signed EIP-712 reward claims.
- **Enforcement**: Enforces activity, daily, and account caps directly in Solidity integer math.

### `OffsetRetirementReceipt.sol`
- **Purpose**: Soulbound NFT recognition for validated third-party registry retirements.

---

## 3. EIP-712 Verifiable Claim Schema
```solidity
struct RewardClaim {
    address recipient;
    uint256 amount;
    bytes32 receiptId;
    bytes32 evidenceHash;
    uint256 nonce;
    uint256 deadline;
}
```
Claims require a cryptographic signature from an authorized off-chain attester before contract execution.
