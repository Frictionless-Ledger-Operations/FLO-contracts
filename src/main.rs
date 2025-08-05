use solana_balance_fetcher::init_balance_service;
use std::sync::Arc;
use anyhow::Context;
use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    env_logger::init();

    // Get proper application data directory
    let app_dir = directories::ProjectDirs::from(".local", "share", "solana-balance")
        .context("Failed to determine application data directory")?;
    
    // Create directory if it doesn't exist
    std::fs::create_dir_all(app_dir.data_dir())
        .context("Failed to create data directory")?;
    
    // Build database path
    let db_path = app_dir.data_dir().join("balances.db");
    let db_path_str = db_path.to_string_lossy().to_string();
    
    println!("Using database at: {}", db_path_str);
    
    let rpc_url = "https://api.devnet.solana.com".to_string();
    let wallet_address = "7LPQPi3dmU7Fmuevapvs8uhuwqTQBT9ZYLRHrYMwmqPJ".to_string();

    // Initialize service with proper DB path
    let service = init_balance_service(rpc_url, db_path_str).await
        .context("Failed to initialize balance service")?;

    // Immediately fetch balance after a transaction
    match service.fetch_and_cache_balance(&wallet_address).await {
        Ok(balance) => println!("Fetched balance: {} SOL", balance.lamports as f64 / 1_000_000_000.0),
        Err(e) => eprintln!("Error fetching balance: {}", e),
    }

    // Start periodic fetch
    let service_clone = service.clone();
    let wallet_clone = wallet_address.clone();
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(30));
        loop {
            interval.tick().await;
            if let Err(e) = service_clone.fetch_and_cache_balance(&wallet_clone).await {
                eprintln!("Error in periodic fetch: {}", e);
            }
        }
    });

    // Keep the service alive
    println!("Balance service running. Press Ctrl+C to exit.");
    tokio::signal::ctrl_c().await?;
    Ok(())
}