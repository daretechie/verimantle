# Security Policy

*"Trust is not a sentiment. It is a verifiable proof."*

AgentKern is mission-critical infrastructure for the Agentic Economy. We treat security not as a feature, but as the fundamental physics of our universe.

---

## 1. The Security Philosophy: Defense in Depth

We operate on a **Zero Trust** model assuming three compromises:
1.  **The Network is Compromised**: All traffic is mTLS encrypted, even strictly local.
2.  **The Admin is Compromised**: Admin keys cannot decrypt agent state (Confidential Computing).
3.  **The Supply Chain is Compromised**: We pin dependencies and sign artifacts (Sigstore).

---

## 2. Confidential Computing (The TEE Layer)

AgentKern Enterprise is designed to run inside **Trusted Execution Environments (TEEs)**.

*   **Memory Encryption**: Agent memory (Synapse) and Logic (Gate) are encrypted in RAM using AMD SEV-SNP or Intel TDX.
*   **Remote Attestation**: Nodes effectively "boot verify" themselves. Other agents will refuse to talk to a node that cannot prove it is running unmodified, signed AgentKern code.
*   **Key Isolation**: Private keys for Agent Identities never leave the enclave. They are generated inside the TEE and are inaccessible to the host OS or cloud provider.

---

## 3. Compliance & Standards

We build to meet the world's most stringent standards for AI and Finance.

| Standard | Status | Implementation |
|----------|--------|----------------|
| **ISO 42001** | **Ready** | AI Management System (AIMS) audit logs built into `Arbiter`. |
| **SOC 2 Type II** | **Ready** | Infrastructure controls, access policies, and change management. |
| **EU AI Act** | **Compliant** | "High Risk" system logging, human oversight hooks, and data governance. |
| **FIPS 140-3** | **Compatible** | When running with compliant HSM modules for key storage. |

---

## 4. Supply Chain Security

We implement **SLSA Level 3** (Supply-chain Levels for Software Artifacts).

*   **SBOMs**: A CycloneDX Software Bill of Materials is generated for every release.
*   **Signing**: All release artifacts (Binaries, Docker Images, WASM Modules) are signed using **Sigstore/Cosign**.
*   **Dependency pinning**: All `Cargo.lock` and `package-lock.json` files are strictly pinned and audited for vulnerabilities weekly.

---

## 5. Vulnerability Reporting

We take all security reports seriously.

**DO NOT** report security vulnerabilities via public GitHub issues.

### Reporting Process
Please email **security@agentkern.io** with:
*   Description of the vulnerability.
*   Steps to reproduce.
*   Potential impact.

We strive to acknowledge reports within 24 hours and provide a fix timeline within 72 hours.

### Bug Bounty
We operate a private Bug Bounty program for critical vulnerabilities in the `Gate` (Verification) and `Treasury` (Payments) modules. Please contact us for an invite.

---

## 6. Incident Response

In the event of a detected compromise (e.g., Prompt Injection bypass, Consensus failure):

1.  **Arbiter Kill Switch**: The global `Arbiter` signal can freeze all agent transactions immediately.
2.  **Revocation**: Compromised Agent Identities are added to the global revocation list (CRL).
3.  **Rollback**: State is rolled back to the last known verifiable snapshot using `Synapse` time-travel features.
