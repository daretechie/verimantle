//! Sovereign Memory Enterprise Features
//!
//! Per LICENSING.md: Cross-cloud migration, encryption, sharding
//! Per licensing_split.md: Enterprise tier for multi-cloud deals

pub mod migration;
pub mod encryption;

// Re-exports
pub use migration::{CloudMigrator, MigrationConfig, CloudTarget};
pub use encryption::{MemoryEncryptor, EncryptionConfig, KeyProvider};
