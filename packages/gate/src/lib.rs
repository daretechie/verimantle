//! VeriMantle-Gate: Neuro-Symbolic Verification Engine
//!
//! Per ARCHITECTURE.md: "The Core (Rust/Hyper-Loop)"
//!
//! Features implemented:
//! - io_uring: Zero-copy async I/O (Linux)
//! - WASM: Policy nano-isolation
//! - TEE: Confidential computing (TDX/SEV)
//! - eBPF: Zero-overhead observability
//! - Neuro-Symbolic: Fast Path (<1ms) + Safety Path (<20ms)

pub mod policy;
pub mod dsl;
pub mod neural;
pub mod engine;
pub mod types;

// Hyper-Stack modules (per ARCHITECTURE.md)
pub mod runtime;          // io_uring Hyper-Loop
pub mod tee;              // Hardware Enclaves (TDX/SEV)
pub mod observability;    // eBPF-compatible tracing

#[cfg(feature = "wasm")]
pub mod wasm;             // WASM Component Model

// Re-exports
pub use engine::GateEngine;
pub use policy::{Policy, PolicyRule, PolicyAction};
pub use types::{VerificationRequest, VerificationResult};
pub use runtime::HyperRuntime;
pub use tee::Enclave;
pub use observability::{ObservabilityPlane, GateMetrics};
