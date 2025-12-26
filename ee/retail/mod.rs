//! Retail Platform Bridge
//!
//! Generic e-commerce integration (Amazon SP-API, Shopify, Walmart, etc.)
//! Technology-focused, vendor-neutral design

pub mod adapter;
pub mod listings;
pub mod orders;
pub mod fulfillment;

pub use adapter::{RetailPlatform, PlatformConfig, RetailError};
pub use listings::{Listing, ListingUpdate, PriceUpdate};
pub use orders::{Order, OrderItem, OrderStatus};
pub use fulfillment::{Fulfillment, ShipmentStatus, TrackingInfo};
