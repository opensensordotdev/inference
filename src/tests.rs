//! All tests in this file assume a live Triton server running with the default ports.  All default
//! ports are described in this crate's README

use crate::TritonClient;
use tokio;
use tracing_test::traced_test;

/// Can I build a client?
#[tokio::test]
#[traced_test]
async fn test_build_client() {
    let addr = "http://127.0.0.1:8001";
    let _server = TritonClient::new(addr).await;
}

/// Does the server return 'true' on the server_live RPC call?
///
/// Edge case: might not return true immediately after you start a new Triton server
#[tokio::test]
#[traced_test]
async fn test_server_live() {
    let addr = "http://127.0.0.1:8001";
    let server = TritonClient::new(addr).await;
    assert!(server
        .is_server_live()
        .await
        .expect("server liveliness probe failed"));
}

/// Does the server return 'true' on the server_ready RPC call?
///
/// Edge case: might not return true immediately after you start a new Triton server
#[tokio::test]
#[traced_test]
async fn test_server_ready() {
    let addr = "http://127.0.0.1:8001";
    let server = TritonClient::new(addr).await;
    assert!(server
        .is_server_ready()
        .await
        .expect("server readiness probe failed"));
}

/// Can I get simple model stats?
///
/// Edge case: if simple models are unloaded, this will fail
#[tokio::test]
#[traced_test]
async fn test_model_statistics() {
    let addr = "http://127.0.0.1:8001";
    let server = TritonClient::new(addr).await;
    assert!(server
        .is_server_ready()
        .await
        .expect("server readiness probe failed"));
    server
        .model_statistics("simple", "")
        .await
        .expect("model statistics probe failed");
}
