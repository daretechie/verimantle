//! Enterprise Energy Module
//!
//! Per LICENSING.md: Real-time grid API, Intersect integration
//! Per licensing_split.md: Enterprise tier (Google acquisition target)

pub mod grid;
pub mod intersect;

// Re-exports
pub use grid::{GridApi, CarbonIntensityFeed, RegionData};
pub use intersect::{IntersectClient, IntersectConfig};
