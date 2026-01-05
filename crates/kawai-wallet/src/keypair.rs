//! Kawai Keypair wrapper

use crate::error::{Error, Result};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signer};
use std::fmt;

/// Wrapper around Solana Keypair with additional functionality
pub struct KawaiKeypair {
    inner: Keypair,
    name: Option<String>,
}

impl KawaiKeypair {
    /// Create a new random keypair
    pub fn new() -> Self {
        Self {
            inner: Keypair::new(),
            name: None,
        }
    }

    /// Create from existing Solana keypair
    pub fn from_keypair(keypair: Keypair) -> Self {
        Self {
            inner: keypair,
            name: None,
        }
    }

    /// Create from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let keypair = Keypair::from_bytes(bytes)
            .map_err(|e| Error::InvalidKeypair(e.to_string()))?;
        Ok(Self {
            inner: keypair,
            name: None,
        })
    }

    /// Create from base58 string
    pub fn from_base58(s: &str) -> Result<Self> {
        let bytes = bs58::decode(s)
            .into_vec()
            .map_err(|e| Error::InvalidPrivateKey(e.to_string()))?;
        Self::from_bytes(&bytes)
    }

    /// Set the wallet name
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Get the wallet name
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Get the public key
    pub fn pubkey(&self) -> Pubkey {
        self.inner.pubkey()
    }

    /// Get the public key as string
    pub fn pubkey_string(&self) -> String {
        self.inner.pubkey().to_string()
    }

    /// Get the secret key bytes
    pub fn secret(&self) -> &[u8] {
        self.inner.secret().as_bytes()
    }

    /// Get the full keypair bytes
    pub fn to_bytes(&self) -> [u8; 64] {
        self.inner.to_bytes()
    }

    /// Export as base58 string
    pub fn to_base58(&self) -> String {
        bs58::encode(self.inner.to_bytes()).into_string()
    }

    /// Get reference to inner keypair
    pub fn inner(&self) -> &Keypair {
        &self.inner
    }

    /// Get shortened pubkey for display
    pub fn short_pubkey(&self) -> String {
        let pk = self.pubkey_string();
        format!("{}...{}", &pk[..4], &pk[pk.len()-4..])
    }
}

impl Default for KawaiKeypair {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for KawaiKeypair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("KawaiKeypair")
            .field("pubkey", &self.pubkey_string())
            .field("name", &self.name)
            .finish()
    }
}

impl fmt::Display for KawaiKeypair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(name) = &self.name {
            write!(f, "{} ({})", name, self.short_pubkey())
        } else {
            write!(f, "{}", self.short_pubkey())
        }
    }
}

impl Signer for KawaiKeypair {
    fn pubkey(&self) -> Pubkey {
        self.inner.pubkey()
    }

    fn try_pubkey(&self) -> std::result::Result<Pubkey, solana_sdk::signer::SignerError> {
        self.inner.try_pubkey()
    }

    fn sign_message(&self, message: &[u8]) -> solana_sdk::signature::Signature {
        self.inner.sign_message(message)
    }

    fn try_sign_message(
        &self,
        message: &[u8],
    ) -> std::result::Result<solana_sdk::signature::Signature, solana_sdk::signer::SignerError> {
        self.inner.try_sign_message(message)
    }

    fn is_interactive(&self) -> bool {
        self.inner.is_interactive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_keypair() {
        let kp = KawaiKeypair::new();
        assert!(!kp.pubkey_string().is_empty());
    }

    #[test]
    fn test_base58_roundtrip() {
        let kp = KawaiKeypair::new();
        let base58 = kp.to_base58();
        let kp2 = KawaiKeypair::from_base58(&base58).unwrap();
        assert_eq!(kp.pubkey(), kp2.pubkey());
    }

    #[test]
    fn test_with_name() {
        let kp = KawaiKeypair::new().with_name("My Wallet");
        assert_eq!(kp.name(), Some("My Wallet"));
    }
}

