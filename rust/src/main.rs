use anyhow::Result;
use std::env;
use tokio_stream::StreamExt;
use tracing::{info, error, warn};
use yellowstone_grpc_client::GeyserGrpcClient;
use yellowstone_grpc_proto::prelude::*;
use tonic::transport::ClientTlsConfig;
use futures::sink::SinkExt;
use futures::TryFutureExt;
use backoff::{future::retry, ExponentialBackoff};

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Configuration - you'll need to set these based on your provider
    let endpoint = env::var("GEYSER_ENDPOINT")
        .unwrap_or_else(|_| "https://your-provider-endpoint".to_string());
    
    let x_token = env::var("GEYSER_ACCESS_TOKEN").ok();

    // Retry with exponential backoff - this is the official Triton pattern
    retry(ExponentialBackoff::default(), move || {
        let endpoint = endpoint.clone();
        let x_token = x_token.clone();
        
        async move {
            info!("Connecting to gRPC endpoint: {}", endpoint);
            
            // Create client following the official example pattern
            let client = GeyserGrpcClient::build_from_shared(endpoint)
                .map_err(|e| backoff::Error::transient(anyhow::Error::from(e)))?
                .x_token(x_token)
                .map_err(|e| backoff::Error::transient(anyhow::Error::from(e)))?
                .tls_config(ClientTlsConfig::new().with_native_roots())
                .map_err(|e| backoff::Error::transient(anyhow::Error::from(e)))?
                .connect()
                .await
                .map_err(|e| backoff::Error::transient(anyhow::Error::from(e)))?;
            
            info!("Successfully connected to Yellowstone gRPC");
            
            // Run the subscription logic
            run_subscription(client).await.map_err(|e| backoff::Error::transient(e))?;
            
            Ok::<(), backoff::Error<anyhow::Error>>(())
        }
        .inspect_err(|error| error!("Connection failed, will retry: {error}"))
    })
    .await
    .map_err(Into::into)
}

async fn run_subscription(mut client: GeyserGrpcClient<impl tonic::service::Interceptor>) -> Result<()> {
    
    // Use the new subscribe_with_request method like the official example
    let (mut subscribe_tx, mut stream) = client.subscribe_with_request(Some(SubscribeRequest {
        slots: std::collections::HashMap::from([
            ("client".to_string(), SubscribeRequestFilterSlots {
                filter_by_commitment: Some(true),
                interslot_updates: Some(false),
            })
        ]),
        commitment: Some(CommitmentLevel::Confirmed as i32),
        ..Default::default()
    })).await?;
    
    info!("Subscribed to slot updates, waiting for messages...");
    
    // Process incoming messages - this follows the official Triton example exactly
    while let Some(message) = stream.next().await {
        match message {
            Ok(msg) => {
                match msg.update_oneof {
                    Some(subscribe_update::UpdateOneof::Slot(slot_update)) => {
                        info!(
                            "Slot update: slot={}, parent={}, status={:?}",
                            slot_update.slot,
                            slot_update.parent.unwrap_or(0),
                            slot_update.status()
                        );
                    }
                    Some(subscribe_update::UpdateOneof::Account(account_update)) => {
                        info!(
                            "Account update: pubkey={}, slot={}, lamports={}",
                            bs58::encode(&account_update.account.as_ref().unwrap().pubkey).into_string(),
                            account_update.slot,
                            account_update.account.as_ref().unwrap().lamports
                        );
                    }
                    Some(subscribe_update::UpdateOneof::Transaction(tx_update)) => {
                        info!(
                            "Transaction update: slot={}, signature={}",
                            tx_update.slot,
                            bs58::encode(&tx_update.transaction.as_ref().unwrap().signature).into_string()
                        );
                    }
                    Some(subscribe_update::UpdateOneof::Block(block_update)) => {
                        info!(
                            "Block update: slot={}, blockhash={}",
                            block_update.slot,
                            bs58::encode(&block_update.blockhash).into_string()
                        );
                    }
                    Some(subscribe_update::UpdateOneof::Ping(_ping)) => {
                        info!("Received ping from server - replying to keep connection alive");
                        // Reply to ping directly here like the official example
                        subscribe_tx
                            .send(SubscribeRequest {
                                ping: Some(SubscribeRequestPing { id: 1 }),
                                ..Default::default()
                            })
                            .await?;
                    }
                    Some(subscribe_update::UpdateOneof::Pong(pong)) => {
                        info!("Received pong response with id: {}", pong.id);
                    }
                    None => {
                        error!("update not found in the message");
                        break;
                    }
                    _ => {
                        warn!("Received unknown update type");
                    }
                }
            }
            Err(e) => {
                error!("Stream error: {}", e);
                break;
            }
        }
    }
    
    warn!("Stream closed, will reconnect...");
    // Always return an error to trigger reconnection
    Err(anyhow::anyhow!("Stream ended, triggering reconnection"))
}