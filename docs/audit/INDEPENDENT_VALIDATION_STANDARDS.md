# Independent Validation & Verification Body (VVB) Standards

## 1. Third-Party Governance
To ensure compliance with the ICVCM Core Carbon Principles (CCPs), all carbon reduction and removal claims require verification by an accredited, independent Validation and Verification Body (VVB).

## 2. Evidence Package Structure
Every evidence package submitted by `acnc-project-mrv` contains:
1. `monitoring_report.json`: Canonical JSON summary of metered inputs.
2. `evidence_tree_root`: Merkle root / SHA-256 digest of all underlying receipts.
3. `leakage_and_uncertainty_audit`: Full calculation trace of gross versus conservative reductions.
4. `anti_double_counting_affidavit`: Registry search proof verifying no overlapping claims exist.
