//! Data encryption utilities using ring
//!
//! This module provides functionality for encrypting and decrypting data
//! using the ring cryptography library.

use anyhow::{Result, anyhow};
use ring::aead::{Aad, BoundKey, Nonce, NonceSequence, OpeningKey, SealingKey, UnboundKey, CHACHA20_POLY1305};
use ring::error::Unspecified;
use ring::rand::{SecureRandom, SystemRandom};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

/// Encryption error types
#[derive(Debug, thiserror::Error)]
pub enum EncryptionError {
    /// Key derivation error
    #[error("Key derivation error: {0}")]
    KeyDerivation(String),
    
    /// Encryption error
    #[error("Encryption error: {0}")]
    Encryption(String),
    
    /// Decryption error
    #[error("Decryption error: {0}")]
    Decryption(String),
    
    /// Invalid key
    #[error("Invalid key")]
    InvalidKey,
    
    /// Invalid data
    #[error("Invalid data")]
    InvalidData,
}

/// Nonce sequence for encryption
struct NonceGen {
    /// Current nonce value
    current: Mutex<[u8; 12]>,
}

impl NonceGen {
    /// Create a new nonce generator with a random initial value
    fn new() -> Result<Self, Unspecified> {
        let rng = SystemRandom::new();
        let mut nonce = [0u8; 12];
        rng.fill(&mut nonce)?;
        
        Ok(Self {
            current: Mutex::new(nonce),
        })
    }
}

impl NonceSequence for NonceGen {
    fn advance(&self) -> Result<Nonce, Unspecified> {
        let mut nonce = self.current.lock().unwrap();
        
        // Increment the last 4 bytes as a 32-bit counter
        let mut counter = u32::from_be_bytes([nonce[8], nonce[9], nonce[10], nonce[11]]);
        counter = counter.wrapping_add(1);
        let counter_bytes = counter.to_be_bytes();
        nonce[8] = counter_bytes[0];
        nonce[9] = counter_bytes[1];
        nonce[10] = counter_bytes[2];
        nonce[11] = counter_bytes[3];
        
        Nonce::try_assume_unique_for_key(&*nonce)
    }
}

/// Encrypted data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedData {
    /// Nonce used for encryption
    pub nonce: Vec<u8>,
    
    /// Encrypted data
    pub ciphertext: Vec<u8>,
}

/// Encryption service
pub struct EncryptionService {
    /// Random number generator
    rng: SystemRandom,
    
    /// Encryption key
    key: UnboundKey,
    
    /// Nonce generator
    nonce_gen: NonceGen,
}

impl EncryptionService {
    /// Create a new encryption service with the given key
    pub fn new(key_material: &str) -> Result<Self> {
        // Derive a key from the key material
        let key = Self::derive_key(key_material)?;
        
        // Create a nonce generator
        let nonce_gen = NonceGen::new().map_err(|e| anyhow!("Failed to create nonce generator: {}", e))?;
        
        Ok(Self {
            rng: SystemRandom::new(),
            key,
            nonce_gen,
        })
    }
    
    /// Encrypt data
    pub fn encrypt(&self, data: &[u8]) -> Result<EncryptedData> {
        // Create a sealing key
        let mut sealing_key = SealingKey::new(self.key.clone(), &self.nonce_gen);
        
        // Encrypt the data
        let mut in_out = data.to_vec();
        let aad = Aad::empty();
        
        sealing_key.seal_in_place_append_tag(aad, &mut in_out)
            .map_err(|e| EncryptionError::Encryption(e.to_string()))?;
        
        // Get the nonce
        let nonce = self.nonce_gen.current.lock().unwrap().clone();
        
        Ok(EncryptedData {
            nonce: nonce.to_vec(),
            ciphertext: in_out,
        })
    }
    
    /// Decrypt data
    pub fn decrypt(&self, encrypted_data: &EncryptedData) -> Result<Vec<u8>> {
        // Create a nonce from the stored value
        let nonce = Nonce::try_assume_unique_for_key(&encrypted_data.nonce)
            .map_err(|e| EncryptionError::Decryption(e.to_string()))?;
        
        // Create an opening key
        let mut opening_key = OpeningKey::new(self.key.clone(), OneNonceSequence::new(nonce));
        
        // Decrypt the data
        let mut in_out = encrypted_data.ciphertext.clone();
        let aad = Aad::empty();
        
        let decrypted_data = opening_key.open_in_place(aad, &mut in_out)
            .map_err(|e| EncryptionError::Decryption(e.to_string()))?;
        
        Ok(decrypted_data.to_vec())
    }
    
    /// Encrypt a string
    pub fn encrypt_string(&self, data: &str) -> Result<String> {
        let encrypted = self.encrypt(data.as_bytes())?;
        let json = serde_json::to_string(&encrypted)
            .map_err(|e| anyhow!("Failed to serialize encrypted data: {}", e))?;
        
        Ok(base64::encode(json))
    }
    
    /// Decrypt a string
    pub fn decrypt_string(&self, data: &str) -> Result<String> {
        let json = base64::decode(data)
            .map_err(|e| anyhow!("Failed to decode base64: {}", e))?;
        
        let encrypted: EncryptedData = serde_json::from_slice(&json)
            .map_err(|e| anyhow!("Failed to deserialize encrypted data: {}", e))?;
        
        let decrypted = self.decrypt(&encrypted)?;
        
        String::from_utf8(decrypted)
            .map_err(|e| anyhow!("Failed to convert decrypted data to string: {}", e))
    }
    
    /// Derive a key from key material
    fn derive_key(key_material: &str) -> Result<UnboundKey> {
        use ring::pbkdf2;
        
        let salt = b"digital-twin-desktop-salt";
        let iterations = 10000;
        
        let mut key = [0u8; 32]; // ChaCha20-Poly1305 uses 256-bit keys
        
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            iterations.try_into().unwrap(),
            salt,
            key_material.as_bytes(),
            &mut key,
        );
        
        UnboundKey::new(&CHACHA20_POLY1305, &key)
            .map_err(|e| EncryptionError::KeyDerivation(e.to_string()).into())
    }
}

/// One-time nonce sequence for decryption
struct OneNonceSequence {
    nonce: Nonce,
    used: bool,
}

impl OneNonceSequence {
    fn new(nonce: Nonce) -> Self {
        Self {
            nonce,
            used: false,
        }
    }
}

impl NonceSequence for OneNonceSequence {
    fn advance(&self) -> Result<Nonce, Unspecified> {
        if self.used {
            return Err(Unspecified);
        }
        
        let mut this = self as *const Self as *mut Self;
        unsafe {
            (*this).used = true;
        }
        
        Ok(self.nonce.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_encryption_decryption() {
        let service = EncryptionService::new("test-key").unwrap();
        
        let data = b"Hello, world!";
        let encrypted = service.encrypt(data).unwrap();
        let decrypted = service.decrypt(&encrypted).unwrap();
        
        assert_eq!(decrypted, data);
    }
    
    #[test]
    fn test_string_encryption_decryption() {
        let service = EncryptionService::new("test-key").unwrap();
        
        let data = "Hello, world!";
        let encrypted = service.encrypt_string(data).unwrap();
        let decrypted = service.decrypt_string(&encrypted).unwrap();
        
        assert_eq!(decrypted, data);
    }
    
    #[test]
    fn test_different_keys() {
        let service1 = EncryptionService::new("key1").unwrap();
        let service2 = EncryptionService::new("key2").unwrap();
        
        let data = b"Hello, world!";
        let encrypted = service1.encrypt(data).unwrap();
        
        // Decryption with a different key should fail
        let result = service2.decrypt(&encrypted);
        assert!(result.is_err());
    }
}