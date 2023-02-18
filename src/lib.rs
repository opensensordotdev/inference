//! Library wrapping the gRPC interface to NVIDIA's Triton Inference Server
//!
//! Exposes a Rust API for creating Triton clients and running ML inference

#![warn(missing_docs)]

pub mod error;
#[allow(clippy::all, missing_docs)]
pub mod inference;
pub mod model;
pub mod registry;
pub mod server;
pub mod system;

#[cfg(test)]
mod tests; // tests in `tests.rs` file

use std::fmt::Debug;

use crate::inference::grpc_inference_service_client::GrpcInferenceServiceClient;
use crate::inference::model_infer_request::{InferInputTensor, InferRequestedOutputTensor};
use crate::inference::{InferParameter, ModelInferRequest, ModelInferResponse};

use std::collections::HashMap;
use tonic::transport::Channel;
use tonic::Status;
use tracing::{event, instrument, Level};

/// A gRPC client for the Triton inference server
pub struct TritonClient {
    client: GrpcInferenceServiceClient<Channel>,
    addr: String,
}

/// Common methods for a derived data producing model to implement
/// TODO: Add more useful methods and factor more of the SimpleModel implementation into generics
pub trait TritonModel {
    /// Build an inference request that can be submitted to a model from
    fn build_inference_request(&self, raw_inputs: Vec<Vec<u8>>) -> ModelInferRequest;
    /// Display a model's output
    fn display_output(&self, response: &ModelInferResponse);
}

/// Base configuration for a model in a Triton Inference Server
/// Used to store the model's parameters and to generate inference requests that can be submitted
/// through a TritonClient
#[derive(Clone, Debug)]
pub struct TritonModelBase {
    /// Name of model
    pub name: String,
    /// Version identifier of model
    pub version: String,
    /// Inference parameters required by the model
    pub param: HashMap<String, InferParameter>,
    /// Shape of input tensors for the model
    pub inputs: Vec<InferInputTensor>,
    /// Shape of output tensors
    pub outputs: Vec<InferRequestedOutputTensor>,
}

impl TritonModelBase {
    /// Construct a new TritonModelBase with the minimum required parameters
    /// TODO: implicitly require Models to store this information by adding getter methods to
    /// the Model Trait
    pub fn new(
        name: String,
        version: String,
        inputs: Vec<InferInputTensor>,
        outputs: Vec<InferRequestedOutputTensor>,
        param: HashMap<String, InferParameter>,
    ) -> Self {
        Self {
            name,
            version,
            param,
            inputs,
            outputs,
        }
    }
}

impl Debug for TritonClient {
    /// Writes the address the TritonClient is connected to
    ///
    /// Explicitly specify Debug implementation so when tracing picks up struct methods for a TritonClient
    /// only the relevant info is added to the trace
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TritonClient")
            .field("addr", &self.addr)
            .finish()
    }
}

impl TritonClient {
    /// Build a new TritonClient
    ///
    /// In order to support running in containerized environments where the Triton server itself
    /// might not be up before a model container using inference tries to query it, this function
    /// will keep retrying client creation every 3 seconds
    ///
    /// TODO: add exponential backoff and eventual failure error type instead of just waiting 3
    /// seconds and trying again
    #[instrument]
    pub async fn new(addr: &str) -> TritonClient {
        event!(Level::TRACE, addr);
        let mut client: Option<GrpcInferenceServiceClient<Channel>> = None;
        while client.is_none() {
            client = match Self::build_client(addr.to_owned()).await {
                Ok(c) => Some(c),
                Err(e) => {
                    event!(Level::WARN, error = ?e);
                    tokio::time::sleep(core::time::Duration::new(3, 0)).await;
                    None
                }
            };
        }
        event!(Level::INFO, "Connected to Triton client at {}", addr);

        Self {
            client: client.unwrap(),
            addr: addr.to_owned(),
        }
    }

    /// Create a gRPC client for Triton
    ///
    /// Service GRPCInferenceService
    ///
    /// Errors:
    /// tonic::transport::Error if addr doesn't resolve to a Triton gRPC server
    async fn build_client(
        addr: String,
    ) -> Result<GrpcInferenceServiceClient<Channel>, tonic::transport::Error> {
        GrpcInferenceServiceClient::connect(addr).await
    }

    /// Get a client
    pub fn client(&self) -> GrpcInferenceServiceClient<Channel> {
        self.client.clone()
    }

    /// Submit a ModelInferRequest to the server
    ///
    /// Note: it is not currently enforced whether the ModelInferRequest came from a model that's
    /// associated with this server, so it is possible for this request to fail for a number of
    /// reasons, including:
    /// - Model doesn't exist
    /// - The input isn't correctly formed
    ///
    /// RPC ModelInfer
    #[instrument(skip(inference_request))]
    pub async fn submit_inference_request(
        &self,
        inference_request: ModelInferRequest,
    ) -> Result<ModelInferResponse, Status> {
        event!(Level::TRACE, model_name = %inference_request.model_name,
            model_version = %inference_request.model_version,
            request_id = %inference_request.id,
        );
        let response = self
            .client()
            .model_infer(inference_request)
            .await?
            .into_inner();

        Ok(response)
    }

    /// Get all the models on the Triton gRPC server
    ///
    /// TODO: implement me! Probably use the model_repository_index method on a TritonClient
    pub async fn model_names(&self) -> Vec<String> {
        unimplemented!();
    }
}
