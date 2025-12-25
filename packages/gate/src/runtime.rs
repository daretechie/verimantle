//! VeriMantle Hyper-Runtime: io_uring Backend
//!
//! Per ARCHITECTURE.md: "The Hyper-Loop"
//! - Uses io_uring for zero-copy network operations
//! - Thread-per-core architecture for minimal context switching
//!
//! This module provides the io_uring-optimized async runtime.

use std::future::Future;

/// Runtime configuration for the Hyper-Loop.
#[derive(Debug, Clone)]
pub struct HyperRuntimeConfig {
    /// Number of worker threads (default: num CPUs)
    pub worker_threads: usize,
    /// io_uring submission queue size
    pub sq_size: u32,
    /// Enable thread-per-core mode
    pub thread_per_core: bool,
}

impl Default for HyperRuntimeConfig {
    fn default() -> Self {
        Self {
            worker_threads: num_cpus(),
            sq_size: 256,
            thread_per_core: true,
        }
    }
}

fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
}

/// The Hyper-Runtime - io_uring-optimized async executor.
#[cfg(all(target_os = "linux", feature = "io_uring"))]
pub struct HyperRuntime {
    config: HyperRuntimeConfig,
}

#[cfg(all(target_os = "linux", feature = "io_uring"))]
impl HyperRuntime {
    /// Create a new Hyper-Runtime with default config.
    pub fn new() -> Self {
        Self::with_config(HyperRuntimeConfig::default())
    }

    /// Create with custom configuration.
    pub fn with_config(config: HyperRuntimeConfig) -> Self {
        Self { config }
    }

    /// Run a future on the io_uring runtime.
    pub fn block_on<F: Future>(&self, future: F) -> F::Output {
        tokio_uring::start(future)
    }

    /// Spawn a task on the io_uring runtime.
    pub fn spawn<F>(&self, future: F) -> tokio_uring::task::JoinHandle<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        tokio_uring::spawn(future)
    }

    /// Get configuration.
    pub fn config(&self) -> &HyperRuntimeConfig {
        &self.config
    }
}

#[cfg(all(target_os = "linux", feature = "io_uring"))]
impl Default for HyperRuntime {
    fn default() -> Self {
        Self::new()
    }
}

/// io_uring TCP Listener wrapper for zero-copy accepts.
#[cfg(all(target_os = "linux", feature = "io_uring"))]
pub mod net {
    use std::io;
    use std::net::SocketAddr;
    use tokio_uring::net::TcpListener as UringTcpListener;
    use tokio_uring::net::TcpStream as UringTcpStream;

    /// Zero-copy TCP listener using io_uring.
    pub struct TcpListener {
        inner: UringTcpListener,
    }

    impl TcpListener {
        /// Bind to an address.
        pub fn bind(addr: SocketAddr) -> io::Result<Self> {
            let inner = UringTcpListener::bind(addr)?;
            Ok(Self { inner })
        }

        /// Accept a new connection with zero-copy.
        pub async fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
            let (stream, addr) = self.inner.accept().await?;
            Ok((TcpStream { inner: stream }, addr))
        }
    }

    /// Zero-copy TCP stream using io_uring.
    pub struct TcpStream {
        inner: UringTcpStream,
    }

    impl TcpStream {
        /// Read with zero-copy buffer.
        pub async fn read(&self, buf: Vec<u8>) -> io::Result<(usize, Vec<u8>)> {
            let (result, buf) = self.inner.read(buf).await;
            Ok((result?, buf))
        }

        /// Write with zero-copy buffer.
        pub async fn write(&self, buf: Vec<u8>) -> io::Result<(usize, Vec<u8>)> {
            let (result, buf) = self.inner.write(buf).await;
            Ok((result?, buf))
        }
    }
}

/// Fallback runtime for non-Linux or when io_uring is disabled.
#[cfg(not(all(target_os = "linux", feature = "io_uring")))]
pub struct HyperRuntime {
    runtime: tokio::runtime::Runtime,
    config: HyperRuntimeConfig,
}

#[cfg(not(all(target_os = "linux", feature = "io_uring")))]
impl HyperRuntime {
    pub fn new() -> Self {
        Self::with_config(HyperRuntimeConfig::default())
    }

    pub fn with_config(config: HyperRuntimeConfig) -> Self {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(config.worker_threads)
            .enable_all()
            .build()
            .expect("Failed to build Tokio runtime");
        Self { runtime, config }
    }

    pub fn block_on<F: Future>(&self, future: F) -> F::Output {
        self.runtime.block_on(future)
    }

    pub fn spawn<F>(&self, future: F) -> tokio::task::JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.runtime.spawn(future)
    }

    pub fn config(&self) -> &HyperRuntimeConfig {
        &self.config
    }
}

#[cfg(not(all(target_os = "linux", feature = "io_uring")))]
impl Default for HyperRuntime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime_creation() {
        let runtime = HyperRuntime::new();
        assert!(runtime.config().worker_threads > 0);
    }

    #[test]
    fn test_runtime_block_on() {
        let runtime = HyperRuntime::new();
        let result = runtime.block_on(async { 42 });
        assert_eq!(result, 42);
    }
}
