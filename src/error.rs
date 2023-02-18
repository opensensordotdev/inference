//! Custom error implementations for the inference crate
//!
//! TODO: These should probably be more granular based on the precise error codes that come out of the
//! gRPC server responses.

/// Error implementation for generic models
#[derive(thiserror::Error)]
pub enum ModelError {
    /// If the inference request fails
    #[error("Failed to execute inference request with provided inputs")]
    InferenceError(String),
    /// If the model fails to build
    #[error("Failed to build model")]
    BuildError(String),
}

/// Error for TritonClient
#[derive(thiserror::Error)]
pub enum TritonClientError {
    /// If the requested model doesn't exist in the model repository
    #[error("The requested model does not exist on this Triton Server")]
    ModelDoesNotExistError(String),
    /// If the TritonClient fails to build
    #[error("Failed to build client")]
    BuildError(String),
    /// If an unknown, unrecoverable error occurs
    #[error("Unknown error occurred")]
    UnknownError(String),
}

// We are still using a bespoke implementation of `Debug`
// to get a nice report using the error source chain
impl std::fmt::Debug for ModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

// We are still using a bespoke implementation of `Debug`
// to get a nice report using the error source chain
impl std::fmt::Debug for TritonClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{e}\n")?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{cause}")?;
        current = cause.source();
    }
    Ok(())
}
