//! Implements model registry operations for a TritonClient

use std::collections::HashMap;

use tonic::{Request, Status};
use tracing::{event, instrument, Level};

use crate::inference::ModelRepositoryParameter;
use crate::inference::{RepositoryIndexRequest, RepositoryIndexResponse};
use crate::inference::{RepositoryModelLoadRequest, RepositoryModelLoadResponse};
use crate::inference::{RepositoryModelUnloadRequest, RepositoryModelUnloadResponse};
use crate::TritonClient;

impl TritonClient {
    /// Get the model repository contents
    ///
    /// RPC RepositoryIndex
    #[instrument]
    pub async fn model_repository_index(&self) -> Result<RepositoryIndexResponse, Status> {
        // If you provide no repository_name, you get all models registered on the server
        // If you set ready = true, you only get models whose status is `ready`, ie. they're ready
        // for the client to submit inference requests
        let message = RepositoryIndexRequest {
            repository_name: "".to_owned(),
            ready: true,
        };
        let request = Request::new(message);
        match self.client().repository_index(request).await {
            Ok(r) => {
                event!(Level::INFO, model_repository_index = ?r);
                Ok(r.into_inner())
            }
            Err(s) => {
                event!(Level::ERROR, status = ?s);
                Err(s)
            }
        }
    }

    /// Load or reload a model from a repository
    ///
    /// RPC RepositoryModelLoad
    #[instrument]
    pub async fn load_model(
        &self,
        repository_name: Option<&str>,
        model_name: &str,
        parameters: Option<HashMap<String, ModelRepositoryParameter>>,
    ) -> Result<RepositoryModelLoadResponse, Status> {
        // convert parameters into the types accepted by the RPC request
        let repository_name = repository_name.unwrap_or("").to_owned();
        let model_name = model_name.to_owned();
        let parameters = match parameters {
            Some(p) => p,
            None => HashMap::new(),
        };

        // submit request
        let request = RepositoryModelLoadRequest {
            repository_name,
            model_name,
            parameters,
        };
        let request = Request::new(request);
        match self.client().repository_model_load(request).await {
            Ok(r) => {
                event!(Level::INFO, "successfully loaded model {r:?}");
                Ok(r.into_inner())
            }
            Err(s) => {
                event!(Level::ERROR, "failed to load model {s}");
                Err(s)
            }
        }
    }

    /// Unload a model
    ///
    /// RPC RepositoryModelUnload
    #[instrument]
    pub async fn unload_model(
        &self,
        repository_name: Option<&str>,
        model_name: &str,
        parameters: Option<HashMap<String, ModelRepositoryParameter>>,
    ) -> Result<RepositoryModelUnloadResponse, Status> {
        // convert parameters into the types accepted by the RPC request
        let repository_name = repository_name.unwrap_or("").to_owned();
        let model_name = model_name.to_owned();
        let parameters = match parameters {
            Some(p) => p,
            None => HashMap::new(),
        };

        // submit request
        let request = RepositoryModelUnloadRequest {
            repository_name,
            model_name,
            parameters,
        };
        let request = Request::new(request);
        match self.client().repository_model_unload(request).await {
            Ok(r) => {
                event!(Level::INFO, "successfully unloaded model, {r:?}");
                Ok(r.into_inner())
            }
            Err(s) => {
                event!(Level::ERROR, "failed to unload model, {s}");
                Err(s)
            }
        }
    }
}
