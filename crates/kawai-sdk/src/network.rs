//! Network configuration for Kawai SDK

use std::fmt;

/// Solana network configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Network {
    /// Mainnet Beta - production network
    Mainnet,
    /// Devnet - development network (free SOL)
    Devnet,
    /// Testnet - test network
    Testnet,
    /// Localhost - local validator
    Localhost,
    /// Custom RPC endpoint
    Custom,
}

impl Network {
    /// Get the default RPC URL for this network
    pub fn rpc_url(&self) -> &'static str {
        match self {
            Network::Mainnet => "https://api.mainnet-beta.solana.com",
            Network::Devnet => "https://api.devnet.solana.com",
            Network::Testnet => "https://api.testnet.solana.com",
            Network::Localhost => "http://127.0.0.1:8899",
            Network::Custom => "",
        }
    }

    /// Get the WebSocket URL for this network
    pub fn ws_url(&self) -> &'static str {
        match self {
            Network::Mainnet => "wss://api.mainnet-beta.solana.com",
            Network::Devnet => "wss://api.devnet.solana.com",
            Network::Testnet => "wss://api.testnet.solana.com",
            Network::Localhost => "ws://127.0.0.1:8900",
            Network::Custom => "",
        }
    }

    /// Check if this is a development network (allows airdrops)
    pub fn is_dev(&self) -> bool {
        matches!(self, Network::Devnet | Network::Testnet | Network::Localhost)
    }

    /// Check if this is production
    pub fn is_mainnet(&self) -> bool {
        matches!(self, Network::Mainnet)
    }

    /// Get the network name
    pub fn name(&self) -> &'static str {
        match self {
            Network::Mainnet => "mainnet-beta",
            Network::Devnet => "devnet",
            Network::Testnet => "testnet",
            Network::Localhost => "localhost",
            Network::Custom => "custom",
        }
    }
}

impl fmt::Display for Network {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl Default for Network {
    fn default() -> Self {
        Network::Devnet
    }
}

impl std::str::FromStr for Network {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "mainnet" | "mainnet-beta" | "m" => Ok(Network::Mainnet),
            "devnet" | "dev" | "d" => Ok(Network::Devnet),
            "testnet" | "test" | "t" => Ok(Network::Testnet),
            "localhost" | "local" | "l" => Ok(Network::Localhost),
            _ => Err(crate::error::Error::Network(format!(
                "Unknown network: {}. Use mainnet, devnet, testnet, or localhost",
                s
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_urls() {
        assert!(Network::Devnet.rpc_url().contains("devnet"));
        assert!(Network::Mainnet.rpc_url().contains("mainnet"));
    }

    #[test]
    fn test_is_dev() {
        assert!(Network::Devnet.is_dev());
        assert!(Network::Testnet.is_dev());
        assert!(Network::Localhost.is_dev());
        assert!(!Network::Mainnet.is_dev());
    }

    #[test]
    fn test_from_str() {
        assert_eq!("devnet".parse::<Network>().unwrap(), Network::Devnet);
        assert_eq!("dev".parse::<Network>().unwrap(), Network::Devnet);
        assert_eq!("mainnet".parse::<Network>().unwrap(), Network::Mainnet);
    }
}

