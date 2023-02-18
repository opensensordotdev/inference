//! Triton server gRPC requests related to the state of the hardware the server is running on

use tonic::{Request, Status};
use tracing::{event, instrument, Level};

use crate::inference::{CudaSharedMemoryRegisterRequest, CudaSharedMemoryRegisterResponse};
use crate::inference::{CudaSharedMemoryStatusRequest, CudaSharedMemoryStatusResponse};
use crate::inference::{CudaSharedMemoryUnregisterRequest, CudaSharedMemoryUnregisterResponse};
use crate::inference::{SystemSharedMemoryRegisterRequest, SystemSharedMemoryRegisterResponse};
use crate::inference::{SystemSharedMemoryStatusRequest, SystemSharedMemoryStatusResponse};
use crate::inference::{SystemSharedMemoryUnregisterRequest, SystemSharedMemoryUnregisterResponse};
use crate::TritonClient;

impl TritonClient {
    /// Get the status of all registered system-shared-memory regions.
    ///
    /// RPC SystemSharedMemoryStatus
    #[instrument]
    pub async fn system_shared_memory_status(
        &mut self,
        name: Option<&str>,
    ) -> Result<SystemSharedMemoryStatusResponse, Status> {
        // convert parameters into the types accepted by the RPC request
        let name = name.unwrap_or("").to_owned();

        // submit request
        let msg = SystemSharedMemoryStatusRequest { name };
        let request = Request::new(msg);
        match self.client().system_shared_memory_status(request).await {
            Ok(r) => {
                event!(Level::INFO, system_shared_memory_status = ?r);
                Ok(r.into_inner())
            }
            Err(s) => {
                event!(Level::ERROR, status = ?s);
                Err(s)
            }
        }
    }

    /// Register a system-shared-memory region
    ///
    /// RPC SystemSharedMemoryRegister
    #[instrument]
    pub async fn register_system_shared_memory(
        &mut self,
        name: &str,
        key: &str,
        offset: u64,
        byte_size: u64,
    ) -> Result<SystemSharedMemoryRegisterResponse, Status> {
        // convert parameters into the types accepted by the RPC request
        let name = name.to_owned();
        let key = key.to_owned();

        // submit request
        let msg = SystemSharedMemoryRegisterRequest {
            name,
            key,
            offset,
            byte_size,
        };
        let request = Request::new(msg);
        match self.client().system_shared_memory_register(request).await {
            Ok(r) => {
                event!(
                    Level::INFO,
                    "successfully registered system shared memory, {r:?}"
                );
                Ok(r.into_inner())
            }
            Err(s) => {
                event!(Level::ERROR, "failed to register system shared memory, {s}");
                Err(s)
            }
        }
    }

    /// Unregister a system-shared-memory region
    ///
    /// RPC SystemSharedMemoryUnregister
    #[instrument]
    pub async fn unregister_system_shared_memory(
        &mut self,
        name: &str,
    ) -> Result<SystemSharedMemoryUnregisterResponse, Status> {
        // convert parameters into the types accepted by the RPC request
        let name = name.to_owned();

        // submit request
        let msg = SystemSharedMemoryUnregisterRequest { name };
        let request = Request::new(msg);
        match self.client().system_shared_memory_unregister(request).await {
            Ok(r) => {
                event!(
                    Level::INFO,
                    "successfully unregistered system shared memory, {r:?}"
                );
                Ok(r.into_inner())
            }
            Err(s) => {
                event!(
                    Level::ERROR,
                    "failed to unregistered system shared memory, {s}"
                );
                Err(s)
            }
        }
    }

    /// Get the status of all registered CUDA-shared-memory regions.
    ///
    /// RPC CudaSharedMemoryStatus
    #[instrument]
    pub async fn cuda_shared_memory_status(
        &mut self,
        name: Option<&str>,
    ) -> Result<CudaSharedMemoryStatusResponse, Status> {
        let name = match name {
            Some(s) => {
                event!(
                    Level::INFO,
                    "getting CUDA shared memory status for region {s}"
                );
                s
            }
            None => {
                event!(
                    Level::INFO,
                    "no named region specified, getting CUDA shared memory status for all regions"
                );
                "" // if no named region specified, get all regions
            }
        }
        .to_owned();

        // submit request
        let msg = CudaSharedMemoryStatusRequest { name };
        let request = Request::new(msg);
        match self.client().cuda_shared_memory_status(request).await {
            Ok(r) => {
                event!(Level::INFO, "got CUDA shared memory status, {r:?}");
                Ok(r.into_inner())
            }
            Err(s) => {
                event!(Level::ERROR, "failed to get CUDA shared memory status, {s}");
                Err(s)
            }
        }
    }

    /// Register a CUDA-shared-memory region
    ///
    /// RPC CudaSharedMemoryRegister
    #[instrument]
    pub async fn register_cuda_shared_memory(
        &mut self,
        name: &str,
        raw_handle: Vec<u8>,
        device_id: i64,
        byte_size: u64,
    ) -> Result<CudaSharedMemoryRegisterResponse, Status> {
        // convert parameters into the types accepted by the RPC request
        let name = name.to_owned();

        // submit request
        let msg = CudaSharedMemoryRegisterRequest {
            name,
            raw_handle,
            device_id,
            byte_size,
        };
        let request = Request::new(msg);
        match self.client().cuda_shared_memory_register(request).await {
            Ok(r) => {
                event!(
                    Level::INFO,
                    "successfully registered CUDA shared memory, {r:?}"
                );
                Ok(r.into_inner())
            }
            Err(s) => {
                event!(Level::ERROR, "failed to register CUDA shared memory, {s}");
                Err(s)
            }
        }
    }

    /// Unregister a CUDA-shared-memory region
    ///
    /// RPC CudaSharedMemoryUnregister
    #[instrument]
    pub async fn unregister_cuda_shared_memory(
        &mut self,
        name: &str,
    ) -> Result<CudaSharedMemoryUnregisterResponse, Status> {
        // convert parameters into the types accepted by the RPC request
        let name = name.to_owned();

        // submit request
        let msg = CudaSharedMemoryUnregisterRequest { name };
        let request = Request::new(msg);
        match self.client().cuda_shared_memory_unregister(request).await {
            Ok(r) => {
                event!(
                    Level::INFO,
                    "successfully unregistered CUDA shared memory, {r:?}"
                );
                Ok(r.into_inner())
            }
            Err(s) => {
                event!(Level::ERROR, "failed to unregister CUDA shared memory, {s}");
                Err(s)
            }
        }
    }
}
