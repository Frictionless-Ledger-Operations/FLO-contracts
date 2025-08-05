use crate::models::CachedBalance;
use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use chrono::Utc;

/// Fetch current balance from Solana RPC
pub fn fetch_balance_from_chain(rpc_client: &RpcClient, wallet_address: &str) -> Result<CachedBalance> {
    let pubkey = Pubkey::from_str(wallet_address)?;
    let lamports = rpc_client.get_balance(&pubkey)?;
    
    // ✅ FIXED: Use the new() constructor instead of direct field access
    Ok(CachedBalance::new(
        wallet_address.to_string(),
        lamports,
        Utc::now(),
    ))
}