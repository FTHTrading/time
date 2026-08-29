# Carbon Offset & Removal Validation Standards

## 1. Distinct Offset Accounting Lifecycle
1. **Measured Footprint**: Disclosed calculation using regional emissions factors. *Not an offset.*
2. **Verified Reduction**: Evidence-backed comparative reduction against historical baseline. *Not an offset.*
3. **Retired Offset / Removal**: Third-party registry certificate permanently cancelled on an authoritative registry. *Eligible for validated reward credit.*

---

## 2. Supported Registries
- **Gold Standard for the Global Goals (GS)**
- **Verra Verified Carbon Standard (VCS)**
- **Puro.earth Carbon Removal Standard (CORC)**
- **Climate Action Reserve (CAR)**
- **American Carbon Registry (ACR)**

---

## 3. Mandatory Certificate Metadata Fields
To achieve `REGISTRY_VERIFIED` status, an entry must supply:
- `registry`: Authorized registry name.
- `serial_number`: Immutable registry serial number.
- `project_id`: Unique registry project identifier.
- `vintage`: Year of carbon vintage.
- `tonnes_co2e_retired`: Volume in metric tonnes.
- `retirement_date`: Date of formal cancellation.
- `evidence_uri`: Publicly resolvable registry verification link.
