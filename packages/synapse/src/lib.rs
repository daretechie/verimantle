//! VeriMantle-Synapse: Graph-based State Ledger with CRDTs
//!
//! Per ARCHITECTURE.md Section 3: "The Speed of Light"
//!
//! Features implemented:
//! - **CRDTs**: Conflict-free Replicated Data Types for eventual consistency
//! - **Graph Vector DB**: State stored as graph with vector embeddings
//! - **Intent Tracking**: Monitor goal progression and detect drift
//! - **TEE Integration**: Encrypted state for sensitive data
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    VeriMantle-Synapse                       │
//! ├─────────────────────────────────────────────────────────────┤
//! │  ┌─────────────────────────────────────────────────────┐   │
//! │  │           Graph Vector Database                      │   │
//! │  │  ┌────────┐    ┌────────┐    ┌────────┐            │   │
//! │  │  │ Agent  │───►│ Intent │───►│ State  │            │   │
//! │  │  │ Node   │    │ Node   │    │ Node   │            │   │
//! │  │  └────────┘    └────────┘    └────────┘            │   │
//! │  └─────────────────────────────────────────────────────┘   │
//! │                          │                                  │
//! │                    CRDT Replication                         │
//! │              (US ← → EU ← → Asia ← → Africa)                │
//! └─────────────────────────────────────────────────────────────┘
//! ```

pub mod state;
pub mod intent;
pub mod drift;
pub mod types;
pub mod graph;   // Graph Vector Database

// Re-exports
pub use state::StateStore;
pub use intent::{IntentPath, IntentStep};
pub use drift::DriftDetector;
pub use types::{AgentState, StateQuery, StateUpdate};
pub use graph::{GraphVectorDB, GraphNode, GraphEdge, NodeType, EdgeType};
