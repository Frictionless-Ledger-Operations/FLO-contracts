//! Solana Balance Fetcher Library
//!
//! This library provides functionality to:
//! - Periodically fetch wallet balances from Solana RPC
//! - Cache them locally (SQLite)
//! - Retrieve latest cached balance for offline use
//! - Trigger immediate fetch after transactions

pub mod models;
pub mod storage;
pub mod network;
pub mod balance_service;

use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::task;

/// Initialize the balance fetcher service
/// 
/// # Arguments
/// * `rpc_url` - URL of the Solana RPC node (e.g., "https://api.mainnet-beta.solana.com")
/// * `db_path` - Path to SQLite database file
/// 
/// # Returns
/// A handle to the running service
pub async fn init_balance_service(
    rpc_url: String,
    db_path: String,
) -> anyhow::Result<Arc<balance_service::BalanceService>> {
    
    
    // Create directory if needed (for the library itself)
    if let Some(parent) = std::path::Path::new(&db_path).parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            anyhow::anyhow!("Failed to create DB directory {}: {}", parent.display(), e)
        })?;
    }

    // Connect to SQLite with proper mode
    let db_pool = SqlitePool::connect(&format!("sqlite:{}?mode=rwc", db_path)).await
        .map_err(|e| anyhow::anyhow!("Failed to connect to SQLite: {}", e))?;

    // Initialize tables
    storage::init_db(&db_pool).await?;

    // Create service
    let service = Arc::new(balance_service::BalanceService::new(rpc_url, db_pool));

    Ok(service)
}