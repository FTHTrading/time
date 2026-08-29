# Privacy & Client-First Data Architecture

## 1. Zero Tracking & Local Persistence
- **Client Storage**: In the current Foundation phase, all activity records, reduction comparisons, and focus sessions are stored strictly in the user's browser `localStorage`.
- **No Remote Telemetry**: The web application does not transmit personally identifiable information (PII), biometric signals, or utility statements to centralized advertising networks or surveillance analytics.
- **SHA-256 Hashes**: Evidence documents are hashed client-side using the standard Web Crypto API (`crypto.subtle.digest('SHA-256', ...)`). Only the resulting digest string is stored in the record proof field.

---

## 2. Portability & User Sovereignty
- **Full JSON Export**: Users can export their complete verifiable audit trail at any time via `/vault.html`.
- **Client-Side Reset**: Users can purge their local state instantly via the "Reset State" button.
