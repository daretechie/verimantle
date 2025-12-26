# VeriMantle SDK Generator

Polyglot SDK generation from a single OpenAPI specification.

## Quick Start

```bash
# Install dependencies
npm install

# Generate all SDKs
npm run generate:all
```

## Generated SDKs

| Language | Output Directory | Package Name |
|----------|------------------|--------------|
| C# (.NET 8) | `./csharp` | `VeriMantle.Client` |
| Python | `./python` | `verimantle_client` |
| Go | `./go` | `verimantle` |
| Java | `./java` | `dev.verimantle:verimantle-client` |
| TypeScript | `./typescript` | `@verimantle/client` |

## Usage

### C# / .NET

```csharp
using VeriMantle.Client;

var config = new Configuration { BasePath = "https://api.verimantle.dev/v1" };
var identityApi = new IdentityApi(config);

var agent = await identityApi.RegisterAgentAsync(new RegisterRequest { Name = "MyAgent" });
```

### Python

```python
from verimantle_client import ApiClient, IdentityApi

client = ApiClient()
identity_api = IdentityApi(client)

agent = identity_api.register_agent({"name": "MyAgent"})
```

### Go

```go
import "github.com/verimantle/sdk-gen/go"

client := verimantle.NewAPIClient(verimantle.NewConfiguration())
agent, _, _ := client.IdentityApi.RegisterAgent(context.Background()).Execute()
```

## Semantic Kernel Integration

The C# SDK can be used with Microsoft Semantic Kernel:

```csharp
// VeriMantle.SK/VeriMantlePlugin.cs
[KernelFunction]
public async Task<bool> VerifyAction(string agentId, string action)
{
    var result = await _gateApi.VerifyActionAsync(new VerifyRequest { AgentId = agentId, Action = action });
    return result.Allowed;
}
```

## Regenerating SDKs

When the API changes:

1. Update `apps/gateway/openapi.yaml`
2. Run `npm run generate:all`
3. Commit generated changes

## OpenAPI Source

The single source of truth is: `apps/gateway/openapi.yaml`

This spec is derived from `packages/sdk/src/ports.ts` (hexagonal ports).
