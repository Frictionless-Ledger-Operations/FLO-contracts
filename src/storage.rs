use crate::models::CachedBalance;
use anyhow::Result;
use sqlx::{SqlitePool};

/// Initialize SQLite database and create tables if not exists
pub async fn init_db(pool: &SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS balances (
            wallet_address TEXT PRIMARY KEY,
            lamports INTEGER NOT NULL,
            last_updated TEXT NOT NULL -- Store as ISO string
        )
        "#,
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Save or update a cached balance in the database
pub async fn save_balance(pool: &SqlitePool, balance: &CachedBalance) -> Result<()> {
    sqlx::query(
        r#"
        INSERT OR REPLACE INTO balances (wallet_address, lamports, last_updated)
        VALUES (?, ?, ?)
        "#,
    )
    .bind(&balance.wallet_address)
    .bind(balance.lamports as i64)
    .bind(&balance.last_updated_iso) // <-- Use string
    .execute(pool)
    .await?;
    Ok(())
}

/// Get cached balance for a wallet address
pub async fn get_cached_balance(pool: &SqlitePool, wallet_address: &str) -> Result<Option<CachedBalance>> {
    let row: Option<(String, i64, String)> = sqlx::query_as(
        "SELECT wallet_address, lamports, last_updated FROM balances WHERE wallet_address = ?"
    )
    .bind(wallet_address)
    .fetch_optional(pool)
    .await?;

    if let Some((addr, lamports, last_updated_iso)) = row {
        Ok(Some(CachedBalance {
            wallet_address: addr,
            lamports: lamports as u64,
            last_updated_iso,
        }))
    } else {
        Ok(None)
    }
}