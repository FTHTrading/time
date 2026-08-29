# Evidence & Attestation Service

The Attester Service validates submitted evidence documents, confirms registry retirement serials, and generates EIP-712 cryptographic signatures for testnet claims.

## Key Responsibilities
1. **Deduplication**: Ensures receipt IDs, evidence hashes, and registry serials are never processed twice.
2. **Registry Adapters**: Connects to public registry APIs (Gold Standard, Verra, Puro) to confirm active retirement status.
3. **EIP-712 Signer**: Signs structured claim payloads using a secure HSM/KMS key.
