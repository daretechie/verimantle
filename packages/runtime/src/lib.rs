//! VeriMantle Universal Runtime
//!
//! Single binary that runs anywhere:
//! - Container (Docker, Podman)
//! - Kubernetes (any cloud)
//! - Serverless (any platform)
//! - Bare metal
//! - Edge devices
//! - Browser (WASM)
//!
//! No vendor-specific code. Auto-detects and adapts.

pub mod detect;
pub mod config;
pub mod serve;

pub use detect::{Environment, detect_environment};
pub use config::{RuntimeConfig, auto_configure};
pub use serve::{serve, Protocol};

/// VeriMantle kernel version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Run VeriMantle with auto-detection.
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Detect environment
    let env = detect_environment();
    tracing::info!("Detected environment: {:?}", env);
    
    // 2. Auto-configure based on environment
    let config = auto_configure(&env);
    tracing::info!("Configuration: {:?}", config);
    
    // 3. Start serving
    serve(&config).await?;
    
    Ok(())
}
