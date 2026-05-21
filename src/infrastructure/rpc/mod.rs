use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::transport::{Channel, Endpoint};
use crate::worker::server::download_service::download_service_client::DownloadServiceClient;
use crate::worker::server::download_service::{DownloadTaskRequest, StatusResponse};

#[derive(Debug)]
pub enum RpcError {
    /// Failed to establish a connection to the worker
    ConnectionFailed(String, String),
    /// The RPC call timed out
    Timeout(String),
    /// An error occurred during the transmission of the request/response
    TransmissionError(String),
}

impl std::fmt::Display for RpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RpcError::ConnectionFailed(addr, err) => write!(f, "Connection failed to {}: {}", addr, err),
            RpcError::Timeout(addr) => write!(f, "Request timeout for {}", addr),
            RpcError::TransmissionError(err) => write!(f, "Transmission error: {}", err),
        }
    }
}

impl std::error::Error for RpcError {}

/// Manages gRPC connections and task transmission to workers.
/// This layer abstracts the underlying tonic implementation and provides connection pooling.
pub struct RpcManager {
    channels: Arc<Mutex<HashMap<String, Channel>>>,
}

impl RpcManager {
    /// Create a new RpcManager with an empty connection pool.
    pub fn new() -> Self {
        Self {
            channels: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Internal helper to get or create a connection channel for a given address.
    async fn get_or_create_channel(&self, address: &str) -> Result<Channel, RpcError> {
        let mut channels = self.channels.lock().await;
        if let Some(channel) = channels.get(address) {
            return Ok(channel.clone());
        }

        let endpoint = Endpoint::from_shared(address.to_string())
            .map_err(|e| RpcError::ConnectionFailed(address.to_string(), e.to_string()))?;
        
        // Set a timeout for the connection attempt itself to avoid hanging during handshake
        let channel = endpoint
            .connect_timeout(std::time::Duration::from_secs(5))
            .connect()
            .await
            .map_err(|e| RpcError::ConnectionFailed(address.to_string(), e.to_string()))?;

        channels.insert(address.to_string(), channel.clone());
        Ok(channel)
    }

    /// Sends a download task to a specific worker address.
    /// 
    /// This function abstracts the gRPC client details and handles connection pooling,
    /// timeouts, and error translation into a unified RpcError type.
    pub async fn send_task_to_worker(
        &self,
        address: &str,
        request: DownloadTaskRequest,
    ) -> Result<StatusResponse, RpcError> {
        let channel = self.get_or_create_channel(address).await?;
        let mut client = DownloadServiceClient::new(channel);

        // Set a timeout for the RPC call itself to prevent hanging indefinitely on network issues
        let timeout_duration = std::time::Duration::from_secs(30);
        
        match tokio::time::timeout(timeout_duration, client.submit_task(request)).await {
            Ok(result) => {
                match result {
                    Ok(response) => Ok(response.into_inner()),
                    Err(status) => Err(RpcError::TransmissionError(format!(
                        "gRPC error: {} - {}",
                        status.code(),
                        status.message()
                    ))),
                }
            }
            Err(_) => Err(RpcError::Timeout(address.to_string())),
        }
    }
}
