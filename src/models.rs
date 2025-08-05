use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use chrono::{DateTime, Utc};

/// Represents a cached wallet balance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedBalance {
    /// Wallet public key (as string)
    pub wallet_address: String,
    /// Lamports balance
    pub lamports: u64,
    /// Timestamp of last successful fetch (ISO 8601 string)
    pub last_updated_iso: String,
}

impl CachedBalance {
    /// Convert wallet address string to Solana Pubkey
    pub fn pubkey(&self) -> Result<Pubkey, solana_sdk::pubkey::ParsePubkeyError> {
        Pubkey::from_str(&self.wallet_address)
    }

    /// Get parsed DateTime<Utc>
    pub fn last_updated(&self) -> Result<DateTime<Utc>, chrono::ParseError> {
        DateTime::parse_from_rfc3339(&self.last_updated_iso)
            .map(|dt| dt.with_timezone(&Utc))
    }

    /// Create from parsed values
    pub fn new(wallet_address: String, lamports: u64, last_updated: DateTime<Utc>) -> Self {
        Self {
            wallet_address,
            lamports,
            last_updated_iso: last_updated.to_rfc3339(),
        }
    }
}