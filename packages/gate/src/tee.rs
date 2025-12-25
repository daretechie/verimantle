//! Confidential Computing - TEE Support
//!
//! Per ARCHITECTURE.md: "The Black Box"
//! - Intel TDX / AMD SEV-SNP for hardware enclaves
//! - Critical keys and PII processed inside enclaves
//! - Even cloud providers cannot see the data
//!
//! This module provides abstraction over TEE capabilities.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// TEE attestation report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationReport {
    /// Report type (TDX, SEV, SGX, Simulated)
    pub tee_type: TeeType,
    /// Measurement of the enclave
    pub measurement: Vec<u8>,
    /// Nonce used in attestation
    pub nonce: Vec<u8>,
    /// Timestamp of attestation
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Signature from the TEE
    pub signature: Vec<u8>,
}

/// Supported TEE types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TeeType {
    /// Intel Trust Domain Extensions
    IntelTDX,
    /// AMD Secure Encrypted Virtualization
    AmdSEV,
    /// Intel Software Guard Extensions
    IntelSGX,
    /// Simulated TEE for development
    Simulated,
}

/// Sealed data - encrypted by TEE.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SealedData {
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
    pub tee_type: TeeType,
}

/// TEE Enclave abstraction.
pub struct Enclave {
    tee_type: TeeType,
    measurement: Vec<u8>,
    secrets: HashMap<String, Vec<u8>>,
}

impl Enclave {
    /// Create a new enclave (simulated for now).
    pub fn new() -> Self {
        Self::simulated()
    }

    /// Create a simulated enclave for development.
    pub fn simulated() -> Self {
        Self {
            tee_type: TeeType::Simulated,
            measurement: vec![0; 32], // Fake measurement
            secrets: HashMap::new(),
        }
    }

    /// Detect and create a real TEE if available.
    pub fn detect() -> Self {
        // Check for Intel TDX
        if Self::is_tdx_available() {
            return Self::intel_tdx();
        }
        
        // Check for AMD SEV
        if Self::is_sev_available() {
            return Self::amd_sev();
        }
        
        // Fallback to simulated
        Self::simulated()
    }

    fn is_tdx_available() -> bool {
        // Check /sys/devices/system/cpu/flags for tdx
        #[cfg(target_os = "linux")]
        {
            if let Ok(flags) = std::fs::read_to_string("/proc/cpuinfo") {
                return flags.contains("tdx");
            }
        }
        false
    }

    fn is_sev_available() -> bool {
        // Check for SEV device
        #[cfg(target_os = "linux")]
        {
            return std::path::Path::new("/dev/sev").exists();
        }
        #[cfg(not(target_os = "linux"))]
        false
    }

    fn intel_tdx() -> Self {
        Self {
            tee_type: TeeType::IntelTDX,
            measurement: vec![0; 48], // TD_REPORT is 48 bytes
            secrets: HashMap::new(),
        }
    }

    fn amd_sev() -> Self {
        Self {
            tee_type: TeeType::AmdSEV,
            measurement: vec![0; 32],
            secrets: HashMap::new(),
        }
    }

    /// Get the TEE type.
    pub fn tee_type(&self) -> TeeType {
        self.tee_type
    }

    /// Generate an attestation report.
    pub fn attest(&self, nonce: &[u8]) -> AttestationReport {
        AttestationReport {
            tee_type: self.tee_type,
            measurement: self.measurement.clone(),
            nonce: nonce.to_vec(),
            timestamp: chrono::Utc::now(),
            signature: self.sign(nonce),
        }
    }

    /// Seal data (encrypt with TEE key).
    pub fn seal(&self, plaintext: &[u8]) -> SealedData {
        let nonce: [u8; 12] = rand_bytes();
        let ciphertext = self.encrypt(plaintext, &nonce);
        SealedData {
            ciphertext,
            nonce: nonce.to_vec(),
            tee_type: self.tee_type,
        }
    }

    /// Unseal data (decrypt with TEE key).
    pub fn unseal(&self, sealed: &SealedData) -> Result<Vec<u8>, anyhow::Error> {
        if sealed.tee_type != self.tee_type {
            return Err(anyhow::anyhow!("TEE type mismatch"));
        }
        Ok(self.decrypt(&sealed.ciphertext, &sealed.nonce))
    }

    /// Store a secret in the enclave.
    pub fn store_secret(&mut self, name: impl Into<String>, secret: Vec<u8>) {
        self.secrets.insert(name.into(), secret);
    }

    /// Retrieve a secret from the enclave.
    pub fn get_secret(&self, name: &str) -> Option<&Vec<u8>> {
        self.secrets.get(name)
    }

    // Internal crypto operations (simulated)
    fn sign(&self, data: &[u8]) -> Vec<u8> {
        // In real TEE, this would use hardware-backed keys
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        self.measurement.hash(&mut hasher);
        hasher.finish().to_le_bytes().to_vec()
    }

    fn encrypt(&self, plaintext: &[u8], _nonce: &[u8]) -> Vec<u8> {
        // In real TEE, this would use hardware-backed encryption
        // For simulation, just XOR with a key derived from measurement
        plaintext.iter()
            .zip(self.measurement.iter().cycle())
            .map(|(p, k)| p ^ k)
            .collect()
    }

    fn decrypt(&self, ciphertext: &[u8], _nonce: &[u8]) -> Vec<u8> {
        // Symmetric, so same as encrypt
        self.encrypt(ciphertext, _nonce)
    }
}

impl Default for Enclave {
    fn default() -> Self {
        Self::new()
    }
}

fn rand_bytes<const N: usize>() -> [u8; N] {
    let mut bytes = [0u8; N];
    // In production, use a proper CSPRNG
    for (i, byte) in bytes.iter_mut().enumerate() {
        *byte = (i as u8).wrapping_mul(31).wrapping_add(17);
    }
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enclave_creation() {
        let enclave = Enclave::new();
        assert_eq!(enclave.tee_type(), TeeType::Simulated);
    }

    #[test]
    fn test_seal_unseal() {
        let enclave = Enclave::new();
        let plaintext = b"secret data";
        
        let sealed = enclave.seal(plaintext);
        let unsealed = enclave.unseal(&sealed).unwrap();
        
        assert_eq!(plaintext.as_slice(), unsealed.as_slice());
    }

    #[test]
    fn test_attestation() {
        let enclave = Enclave::new();
        let nonce = b"random_nonce";
        
        let report = enclave.attest(nonce);
        
        assert_eq!(report.tee_type, TeeType::Simulated);
        assert_eq!(report.nonce, nonce.to_vec());
        assert!(!report.signature.is_empty());
    }

    #[test]
    fn test_secret_storage() {
        let mut enclave = Enclave::new();
        enclave.store_secret("api_key", b"super_secret".to_vec());
        
        let secret = enclave.get_secret("api_key").unwrap();
        assert_eq!(secret.as_slice(), b"super_secret");
    }
}
