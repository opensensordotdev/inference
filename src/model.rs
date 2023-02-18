//! Client for interfacing with the Triton inference server over gRPC

use tonic::{Request, Status};
use tracing::{event, instrument, Level};

use crate::inference::{ModelConfigRequest, ModelMetadataRequest, ModelStatisticsRequest};
use crate::TritonClient;

use crate::inference::{
    ModelConfigResponse, ModelMetadataResponse, ModelReadyRequest, ModelStatisticsResponse,
};

impl TritonClient {
    /// Is a specific model & version ready to accept inference requests?
    ///
    /// RPC ModelReady
    ///
    /// Returns an error if the model doesn't exist on the server or the request fails
    #[instrument]
    pub async fn is_model_ready(&self, name: &str, version: &str) -> Result<bool, Status> {
        let message = ModelReadyRequest {
            name: name.to_owned(),
            version: version.to_owned(),
        };
        let request = Request::new(message);

        match self.client().model_ready(request).await {
            Ok(r) => {
                let model_readiness = r.into_inner().ready;
                if model_readiness {
                    event!(Level::INFO, "model is ready");
                } else {
                    event!(Level::INFO, "model is not ready");
                }
                Ok(model_readiness)
            }
            Err(s) => {
                event!(Level::ERROR, "failed to get model readiness, {s}");
                Err(s)
            }
        }
    }

    /// Get a model's metadata
    ///
    /// RPC ModelMetadata
    #[instrument]
    pub async fn model_metadata(
        &self,
        name: &str,
        version: &str,
    ) -> Result<ModelMetadataResponse, Status> {
        let msg = ModelMetadataRequest {
            name: name.to_owned(),
            version: version.to_owned(),
        };
        let request = Request::new(msg);
        match self.client().model_metadata(request).await {
            Ok(r) => {
                let model_metadata = r.into_inner();
                event!(Level::INFO, ?model_metadata);
                Ok(model_metadata)
            }
            Err(s) => {
                event!(Level::ERROR, "failed to get model metadata, {s}");
                Err(s)
            }
        }
    }

    /// Get model configuration info
    ///
    /// RPC ModelConfig
    #[instrument]
    pub async fn model_config(
        &self,
        name: &str,
        version: &str,
    ) -> Result<ModelConfigResponse, Status> {
        let msg = ModelConfigRequest {
            name: name.to_owned(),
            version: version.to_owned(),
        };
        let request = Request::new(msg);
        match self.client().model_config(request).await {
            Ok(r) => {
                let model_config = r.into_inner();
                event!(Level::INFO, ?model_config);
                Ok(model_config)
            }
            Err(s) => {
                event!(Level::ERROR, "failed to get model configuration, {s}");
                Err(s)
            }
        }
    }

    /// Get the cumulative inference statistics for a model
    ///
    /// RPC ModelStatistics
    #[instrument]
    pub async fn model_statistics(
        &self,
        name: &str,
        version: &str,
    ) -> Result<ModelStatisticsResponse, Status> {
        let msg = ModelStatisticsRequest {
            name: name.to_owned(),
            version: version.to_owned(),
        };
        let request = Request::new(msg);
        match self.client().model_statistics(request).await {
            Ok(r) => {
                let model_statistics = r.into_inner();
                event!(Level::INFO, ?model_statistics);
                Ok(model_statistics)
            }
            Err(s) => {
                event!(Level::ERROR, "failed to get model statistics, {s}");
                Err(s)
            }
        }
    }
}
