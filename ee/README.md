# VeriMantle Enterprise Edition

> **"You can run VeriMantle on your laptop for free. But if you want to run 10,000 agents safely, you need VeriMantle Cloud."**

This directory contains VeriMantle Enterprise features, licensed under the [Enterprise License](./LICENSE-ENTERPRISE.md).

## Enterprise Modules

| Module | Description | Feature |
|--------|-------------|---------|
| `cloud/` | Multi-Cell Mesh | Coordinate 100+ nodes globally |
| `cockpit/` | Mission Control Dashboard | Team management, SSO |
| `sso/` | Enterprise Authentication | SAML, OIDC, LDAP |
| `audit-export/` | Compliance Export | ISO 42001, SOC2 reports |
| `sovereign-mesh/` | Global Geo-Fencing | Multi-region data sovereignty |

## Comparison: Open Source vs Enterprise

| Feature | Open Source | Enterprise |
|---------|-------------|------------|
| **Agents** | Unlimited | Unlimited |
| **Nodes** | Single Node | **Multi-Node Cluster** |
| **State** | SQLite/Filesystem | **Global Graph (Synapse)** |
| **Security** | Basic Guardrails | **SOC2 / HIPAA / Takaful** |
| **Support** | Community | **24/7 SLA** |
| **Price** | Free | **Usage-Based** |

## Getting Started

1. **Evaluation**: Test enterprise features for 30 days free
2. **Purchase**: Visit https://verimantle.com/pricing
3. **License Key**: Add to your environment:
   ```bash
   export VERIMANTLE_LICENSE_KEY="your-license-key"
   ```

## Contact

- **Sales**: enterprise@verimantle.com
- **Support**: support@verimantle.com
