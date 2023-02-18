//! Triton gRPC requests related to the server's health and readiness to serve a model

use tonic::{Request, Status};
use tracing::{event, instrument, Level};

use crate::TritonClient;

use crate::inference::{ServerLiveRequest, ServerReadyRequest};
use crate::inference::{ServerMetadataRequest, ServerMetadataResponse};

impl TritonClient {
    /// Is an inference server live at the provided client?
    ///  
    /// RPC ServerLive
    #[instrument]
    pub async fn is_server_live(&self) -> Result<bool, Status> {
        let request = Request::new(ServerLiveRequest {});
        match self.client().server_live(request).await {
            Ok(r) => {
                // if you get a valid response, server can still be live or dead
                let server_liveliness = r.get_ref().live;
                event!(Level::INFO, server_live = ?r);
                Ok(server_liveliness)
            }
            Err(s) => {
                // if you don't get a valid response, the server isn't live by default
                event!(Level::ERROR, "failed to query server liveliness, {s}");
                Err(s)
            }
        }
    }

    /// Is an inference server ready at the provided client?
    ///
    /// RPC ServerReady
    #[instrument]
    pub async fn is_server_ready(&self) -> Result<bool, Status> {
        let request = Request::new(ServerReadyRequest {});
        match self.client().server_ready(request).await {
            Ok(r) => {
                let readiness = r.get_ref().ready;
                event!(Level::INFO, server_ready = ?r);
                Ok(readiness)
            }
            Err(s) => {
                event!(Level::ERROR, "failed to query server readiness, {s}");
                Err(s)
            }
        }
    }

    /// Get the server metadata for the provided client
    ///
    /// RPC ServerMetadata
    #[instrument]
    pub async fn server_metadata(&self) -> Result<ServerMetadataResponse, Status> {
        let request = Request::new(ServerMetadataRequest {});
        match self.client().server_metadata(request).await {
            Ok(r) => {
                event!(Level::INFO, server_metadata = ?r);
                Ok(r.into_inner())
            }
            Err(s) => {
                event!(Level::ERROR, "failed to get server metadata, {s}");
                Err(s)
            }
        }
    }
}
