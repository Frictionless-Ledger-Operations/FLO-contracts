use crate::models::CachedBalance;
use crate::network;
use crate::storage;
use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use sqlx::SqlitePool;
use std::time::Duration;
use tokio::time::{sleep, interval};
use log::{info, warn};

/// Balance service handles periodic fetching and on-demand updates
pub struct BalanceService {
    rpc_client: RpcClient,
    db_pool: SqlitePool,
}

impl BalanceService {
    /// Create new balance service instance
    pub fn new(rpc_url: String, db_pool: SqlitePool) -> Self {
        let rpc_client = RpcClient::new(rpc_url);
        Self { rpc_client, db_pool }
    }

    /// Fetch balance for a specific wallet and cache it
    pub async fn fetch_and_cache_balance(&self, wallet_address: &str) -> Result<CachedBalance> {
        info!("Fetching balance for {}", wallet_address);
        match network::fetch_balance_from_chain(&self.rpc_client, wallet_address) {
            Ok(balance) => {
                storage::save_balance(&self.db_pool, &balance).await?;
                info!("Successfully cached balance for {}", wallet_address);
                Ok(balance)
            },
            Err(e) => {
                warn!("Failed to fetch balance for {}: {}", wallet_address, e);
                // Fallback to cached value if available
                if let Ok(Some(cached)) = storage::get_cached_balance(&self.db_pool, wallet_address).await {
                    info!("Using cached balance for {}", wallet_address);
                    Ok(cached)
                } else {
                    Err(e.into())
                }
            }
        }
    }

    /// Start background task to fetch all wallets every 5 minutes
    pub async fn start_periodic_fetch(&self, wallet_addresses: Vec<String>) {
        let mut interval = interval(Duration::from_secs(5 * 60)); // 5 minutes
        loop {
            interval.tick().await;
            info!("Starting periodic balance fetch for {} wallets", wallet_addresses.len());
            for address in &wallet_addresses {
                if let Err(e) = self.fetch_and_cache_balance(address).await {
                    warn!("Periodic fetch failed for {}: {}", address, e);
                }
            }
        }
    }

    /// Get latest cached balance (for offline display)
    pub async fn get_latest_cached_balance(&self, wallet_address: &str) -> Result<Option<CachedBalance>> {
        storage::get_cached_balance(&self.db_pool, wallet_address).await.map_err(Into::into)
    }
}