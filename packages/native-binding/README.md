# VeriMantle Native Binding

Native Node.js bindings for VeriMantle Rust core using NAPI-RS.

## Usage

```typescript
import { verifyAction, getAttestation, checkCarbonBudget } from '@verimantle/native';

// Verify an agent action
const result = await verifyAction({
  agentId: 'agent-123',
  action: 'transfer_funds',
  context: JSON.stringify({ amount: 1000, currency: 'USD' }),
});

if (result.allowed) {
  console.log('Action allowed');
} else {
  console.log('Blocked by:', result.blockingPolicies);
}

// Get TEE attestation
const attestation = await getAttestation('random-nonce-123');
console.log('Platform:', attestation.platform);
console.log('Quote:', attestation.quote);

// Check carbon budget
const allowed = checkCarbonBudget('agent-123', 50.0);
```

## Building

```bash
npm install
npm run build
```

## Architecture

This package uses NAPI-RS to expose Rust functions to Node.js:

- `verifyAction()` → `verimantle-gate::engine::GateEngine::verify()`
- `getAttestation()` → `verimantle-gate::tee::TeeRuntime::get_attestation()`
- `checkCarbonBudget()` → `verimantle-treasury::carbon::CarbonLedger`
