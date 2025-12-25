//! VeriMantle-Arbiter: Conflict Resolution & Coordination Engine
//!
//! Per ARCHITECTURE.md: "The Core (Rust/Hyper-Loop)"
//!
//! Features implemented:
//! - **Thread-per-Core**: Minimal context switching for sub-ms latency
//! - **Raft Consensus**: Strong consistency for Atomic Business Locks
//! - **Priority Preemption**: Higher priority agents can preempt locks
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    VeriMantle-Arbiter                       │
//! ├─────────────────────────────────────────────────────────────┤
//! │         Thread-per-Core Runtime (Hyper-Loop)                │
//! │  ┌─────────┐    ┌─────────┐    ┌─────────┐                 │
//! │  │ Core 0  │    │ Core 1  │    │ Core N  │                 │
//! │  │         │    │         │    │         │                 │
//! │  └────┬────┘    └────┬────┘    └────┬────┘                 │
//! │       │              │              │                       │
//! │       └──────────────┼──────────────┘                       │
//! │                      ▼                                      │
//! │           ┌─────────────────────┐                          │
//! │           │ Raft Lock Manager   │                          │
//! │           │ (Strong Consistency)│                          │
//! │           └─────────────────────┘                          │
//! └─────────────────────────────────────────────────────────────┘
//! ```

pub mod locks;
pub mod queue;
pub mod coordinator;
pub mod types;

// Hyper-Stack modules (per ARCHITECTURE.md)
pub mod raft;              // Raft Consensus for Atomic Business Locks
pub mod thread_per_core;   // Thread-per-Core for minimal latency

// Re-exports
pub use locks::LockManager;
pub use queue::PriorityQueue;
pub use coordinator::Coordinator;
pub use types::{BusinessLock, CoordinationRequest, CoordinationResult, LockType};
pub use raft::{RaftLockManager, RaftConfig, RaftState};
pub use thread_per_core::{ThreadPerCoreRuntime, ThreadPerCoreConfig};
