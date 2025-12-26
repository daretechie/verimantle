# VeriMantle

**The High-Performance Kernel for Autonomous AI Agents**

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange.svg)](https://www.rust-lang.org/)
[![Tests](https://img.shields.io/badge/Tests-334%20passing-green.svg)](#testing)

VeriMantle is an open-source infrastructure layer for orchestrating, verifying, and governing autonomous AI agents. Built in Rust for maximum performance, it provides the missing "kernel" for the agent economy.

## Why VeriMantle?

| Problem | Solution |
|---------|----------|
| Agents can't talk to legacy systems | **Legacy Bridge** - SAP, SWIFT, Mainframe connectors |
| Agent identity is trapped in one cloud | **Memory Passport** - Portable agent state |
| No human oversight for risky actions | **Escalation System** - Trust thresholds + approvals |
| 73% of LLM apps are vulnerable | **Prompt Guard** - Multi-layer injection defense |
| EU AI Act compliance (Aug 2025) | **Compliance Export** - Article 9-15 documentation |

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         VeriMantle                               │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────┐            │
│  │  Gate   │  │ Synapse │  │ Arbiter │  │  Nexus  │            │
│  │ Policy  │  │ Memory  │  │ Control │  │ Routing │            │
│  └─────────┘  └─────────┘  └─────────┘  └─────────┘            │
└─────────────────────────────────────────────────────────────────┘
```

### Packages (Apache 2.0)

| Package | Description | Tests |
|---------|-------------|-------|
| **gate** | Policy enforcement, neural verification, prompt guard | 127 |
| **synapse** | Memory state, embeddings, CRDT, passport | 67 |
| **arbiter** | Coordination, kill switch, escalation, compliance | 86 |
| **nexus** | Protocol translation (A2A, MCP), routing | 54 |

## Quick Start

```bash
# Clone repository
git clone https://github.com/daretechie/verimantle.git
cd verimantle

# Run tests
cd packages/gate && cargo test
cd packages/synapse && cargo test
cd packages/arbiter && cargo test
cd packages/nexus && cargo test
```

## Key Features

### 🔌 Legacy Bridge (Phase 2)

Connect agents to enterprise systems:

```rust
use verimantle_gate::connectors::{SqlConnector, ConnectorConfig};

// Free SQL connector (Community)
let connector = SqlConnector::new(ConnectorConfig::default());
let result = connector.translate(&a2a_task)?;
```

Parsers included: SWIFT MT, SAP IDOC, COBOL Copybook

### 🛂 Memory Passport (Phase 2)

Portable agent identity with sovereignty controls:

```rust
use verimantle_synapse::passport::{MemoryPassport, PassportExporter};

let passport = MemoryPassport::new(agent_identity, "US");
let exporter = PassportExporter::new();
let data = exporter.export(&passport, &options)?;
```

GDPR Article 20 compliant export included.

### 🚨 Escalation System (Phase 2)

Human-in-the-loop for high-risk actions:

```rust
use verimantle_arbiter::escalation::{EscalationTrigger, ApprovalWorkflow};

let trigger = EscalationTrigger::new(config);
if trigger.evaluate(trust_score)?.should_escalate() {
    workflow.request_approval(request)?;
}
```

### 🛡️ Prompt Guard (Security)

Multi-layer prompt injection defense:

```rust
use verimantle_gate::prompt_guard::PromptGuard;

let guard = PromptGuard::new();
let analysis = guard.analyze(user_input);
if analysis.action == PromptAction::Block {
    return Err("Potential injection detected");
}
```

### 📋 EU AI Act Compliance (Phase 3)

Technical documentation for Article 9-15:

```rust
use verimantle_arbiter::eu_ai_act::{EuAiActExporter, TechnicalDocumentation};

let exporter = EuAiActExporter::new();
let report = exporter.generate_report(&documentation);
let text = exporter.export_text(&documentation);
```

### 💰 Cost Attribution (Phase 3)

Track agent spending to prevent runaway costs:

```rust
use verimantle_arbiter::cost::{CostTracker, CostCategory};

let tracker = CostTracker::new();
tracker.record(tracker.event("agent-1", CostCategory::LlmInference)
    .amount(0.003)
    .quantity(1000.0, "tokens")
    .build());
```

## Enterprise Edition (ee/)

Commercial features for production deployments:

| Feature | Description |
|---------|-------------|
| **SAP Connector** | RFC, BAPI, OData, Event Mesh |
| **SWIFT Connector** | ISO 20022, GPI, Sanctions |
| **Mainframe Connector** | CICS, IMS, IBM MQ |
| **Cross-Cloud Migration** | AWS, GCP, Azure adapters |
| **Memory Encryption** | KMS integration, envelope encryption |
| **Slack/Teams/PagerDuty** | Native escalation integrations |
| **Carbon Grid API** | Real-time intensity + Intersect |

See [ee/LICENSE-ENTERPRISE.md](ee/LICENSE-ENTERPRISE.md) for licensing.

## Testing

```bash
# Run all tests (334 total)
cd packages/gate && cargo test      # 127 tests
cd packages/synapse && cargo test   # 67 tests
cd packages/arbiter && cargo test   # 86 tests
cd packages/nexus && cargo test     # 54 tests
```

## MANDATE Compliance

Per [MANDATE.md](MANDATE.md), all code follows:

- ✅ **Rust Core** - All critical paths in Rust
- ✅ **100% Test Coverage** - 334 tests passing
- ✅ **Zero Tolerance** - No mocks in production
- ✅ **WASM Sandboxing** - Connectors isolated
- ✅ **Kill Switch** - Emergency agent termination
- ✅ **Carbon Aware** - ESG-compliant scheduling

## License

- **packages/** - Apache 2.0 (Free, Open Source)
- **ee/** - Commercial License (See [ee/LICENSE-ENTERPRISE.md](ee/LICENSE-ENTERPRISE.md))

## Contributing

Contributions to `packages/` are welcome under Apache 2.0.
Enterprise features in `ee/` require a CLA.

---

Built for the agent economy. 🤖
