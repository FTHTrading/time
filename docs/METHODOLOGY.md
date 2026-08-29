# Emissions & Footprint Calculation Methodology

Version: `2026.1`  
Published Standards: EPA GHG Emission Factors Hub, IPCC AR6, eGRID 2024, ICAO, DEFRA

---

## 1. Disclosed Factors Matrix

| Category | Sub-Category | Emissions Factor | Disclosed Source |
|---|---|---|---|
| **Home Energy** | Grid Electricity (US Avg) | `0.385 kg CO2e / kWh` | EPA eGRID 2024 |
| **Home Energy** | Residential Natural Gas | `2.020 kg CO2e / therm` | EPA GHG Factors Hub |
| **Water** | Municipal Treated Water | `0.003 kg CO2e / gallon` | Water Research Foundation |
| **Transport** | Gasoline Passenger Car | `0.404 kg CO2e / mile` | EPA GHG Factors Hub |
| **Transport** | Diesel Passenger Car | `0.450 kg CO2e / mile` | EPA GHG Factors Hub |
| **Transport** | Electric Vehicle (Grid Avg) | `0.110 kg CO2e / mile` | DOE AFDC |
| **Transport** | Public Transit (Bus/Rail) | `0.140 kg CO2e / passenger-mile` | DOT FTA |
| **Transport** | Domestic Flight Economy | `0.210 kg CO2e / passenger-mile` | ICAO Carbon Calculator |
| **Food & Waste**| Landfill Food Waste | `2.500 kg CO2e / kg` | EPA WARM Model v15 |
| **Digital** | Cloud GPU Workload | `0.180 kg CO2e / hour` | Cloud Sustainability Disclosures |

---

## 2. Integer Mathematical Precision
All financial point and token issuance calculations use integer basis points or fixed-point representations to eliminate IEEE-754 floating-point drift.
