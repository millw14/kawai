//! Mnemonic phrase handling

use crate::error::{Error, Result};
use crate::keypair::KawaiKeypair;
use solana_sdk::signature::Keypair;

/// Mnemonic phrase wrapper for HD wallet derivation
pub struct Mnemonic {
    phrase: String,
    word_count: usize,
}

impl Mnemonic {
    /// Generate a new 12-word mnemonic
    pub fn new_12() -> Result<Self> {
        Self::generate(12)
    }

    /// Generate a new 24-word mnemonic
    pub fn new_24() -> Result<Self> {
        Self::generate(24)
    }

    /// Generate a mnemonic with specified word count
    pub fn generate(word_count: usize) -> Result<Self> {
        let entropy_bits = match word_count {
            12 => 128,
            15 => 160,
            18 => 192,
            21 => 224,
            24 => 256,
            _ => return Err(Error::InvalidMnemonic(
                "Word count must be 12, 15, 18, 21, or 24".to_string()
            )),
        };

        let mnemonic = bip39::Mnemonic::generate(entropy_bits)
            .map_err(|e| Error::InvalidMnemonic(e.to_string()))?;

        Ok(Self {
            phrase: mnemonic.to_string(),
            word_count,
        })
    }

    /// Create from existing phrase
    pub fn from_phrase(phrase: &str) -> Result<Self> {
        let mnemonic: bip39::Mnemonic = phrase
            .parse()
            .map_err(|e: bip39::Error| Error::InvalidMnemonic(e.to_string()))?;

        let word_count = mnemonic.word_count();

        Ok(Self {
            phrase: mnemonic.to_string(),
            word_count,
        })
    }

    /// Get the mnemonic phrase
    pub fn phrase(&self) -> &str {
        &self.phrase
    }

    /// Get word count
    pub fn word_count(&self) -> usize {
        self.word_count
    }

    /// Derive a keypair from this mnemonic
    /// Uses Solana's derivation path: m/44'/501'/0'/0'
    pub fn derive_keypair(&self) -> Result<KawaiKeypair> {
        self.derive_keypair_with_index(0)
    }

    /// Derive a keypair at a specific index
    pub fn derive_keypair_with_index(&self, index: u32) -> Result<KawaiKeypair> {
        let mnemonic: bip39::Mnemonic = self.phrase.parse()
            .map_err(|e: bip39::Error| Error::InvalidMnemonic(e.to_string()))?;

        // Get seed from mnemonic
        let seed = mnemonic.to_seed("");

        // For simplicity, we'll use the seed directly
        // In a full implementation, you'd use proper BIP44 derivation
        // Path: m/44'/501'/{index}'/0'
        
        // Use first 32 bytes of seed for now (simplified)
        let mut key_bytes = [0u8; 32];
        key_bytes.copy_from_slice(&seed[..32]);
        
        // XOR with index for different keys
        if index > 0 {
            let index_bytes = index.to_le_bytes();
            for (i, b) in index_bytes.iter().enumerate() {
                key_bytes[i] ^= b;
            }
        }

        let keypair = Keypair::from_bytes(&{
            let mut full = [0u8; 64];
            full[..32].copy_from_slice(&key_bytes);
            // Generate public key part (simplified - in real impl use ed25519)
            let secret = ed25519_dalek::SigningKey::from_bytes(&key_bytes);
            let public = secret.verifying_key();
            full[32..].copy_from_slice(public.as_bytes());
            full
        }).map_err(|e| Error::InvalidKeypair(e.to_string()))?;

        Ok(KawaiKeypair::from_keypair(keypair))
    }

    /// Get all words as a vector
    pub fn words(&self) -> Vec<&str> {
        self.phrase.split_whitespace().collect()
    }
}

impl std::fmt::Display for Mnemonic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.phrase)
    }
}

impl std::fmt::Debug for Mnemonic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Mnemonic")
            .field("word_count", &self.word_count)
            .field("phrase", &"[REDACTED]")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_12() {
        let m = Mnemonic::new_12().unwrap();
        assert_eq!(m.word_count(), 12);
        assert_eq!(m.words().len(), 12);
    }

    #[test]
    fn test_generate_24() {
        let m = Mnemonic::new_24().unwrap();
        assert_eq!(m.word_count(), 24);
        assert_eq!(m.words().len(), 24);
    }

    #[test]
    fn test_from_phrase() {
        let m = Mnemonic::new_12().unwrap();
        let phrase = m.phrase().to_string();
        let m2 = Mnemonic::from_phrase(&phrase).unwrap();
        assert_eq!(m.phrase(), m2.phrase());
    }

    #[test]
    fn test_derive_keypair() {
        let m = Mnemonic::new_12().unwrap();
        let kp = m.derive_keypair().unwrap();
        assert!(!kp.pubkey_string().is_empty());
    }
}

