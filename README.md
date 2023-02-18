# Inference

A Rust crate for managing the inference process for machine learning (ML) models. Currently, we support interacting with a Triton Inference Server, loading models from a MinIO Model Store.

## Requirements

- [rust](https://www.rust-lang.org/): Minimum Supported Rust Version 1.58
- [lld linker](https://lld.llvm.org/): for faster Rust builds.
  - See [.cargo/config.toml](.cargo/config.toml) for platform-specific installation instructions and the [Rust Performance Book](https://nnethercote.github.io/perf-book/compile-times.html) for the reasons for using lld.
- [docker](https://www.docker.com/): Container engine
- NVIDIA container toolkit for Docker and a GPU supported by the container toolkit.
  - It may be possible to avoid the NVIDIA GPU requirement by changing the resource reservations for the `triton` service in `docker compose` to not require a GPU...YMMV based on whether the model you're trying to serve inference requests from was already compiled/optimized for GPU-only inference.
- [docker compose](https://docs.docker.com/compose/): Multi-container orchestration. NOTE: `docker-compose` is now deprecated and the compose functionality is integrated into the `docker compose` command. To install alongside an existing docker installation, run `sudo apt-get install docker-compose-plugin`. [ref](https://docs.docker.com/compose/#compose-v2-and-the-new-docker-compose-command).
- [protoc](https://grpc.io/docs/protoc-installation/): Google Protocol Buffer compiler. Needed to build protobufs to bind to the Triton gRPC server.

For Debian-based Linux distros, you can install `inference`'s dependencies (except Docker & NVIDIA container toolkit, that require special repository configuration documented above) with the following command:

`apt-get install clang build-essential lld clang protobuf-compiler libprotobuf-dev zstd libzstd-dev make cmake pkg-config libssl-dev`

`inference` is tested on Ubuntu 22.04 LTS, but welcomes pull requests to fix Windows or MacOS issues.

## Quick Start

1. Clone repo: ```git clone https://github.com/opensensordotdev/inference.git```
  - Ensure all requirements have been installed, especially the lld linker and `protoc`!  Otherwise `inference` won't build!
2. `make`: Download the latest versions of the Triton Inference Server Protocol Buffer files & Triton sample ML models
3. `docker compose up`: Start the MinIO and Triton containers + monitoring infrastructure
  - If you have a GPU available, uncomment the below section of the `inference.triton` service in `docker-compose.yaml`. In order for your GPU to work with Triton, the CUDA versions on your host OS and the CUDA version expected by Triton have to be compatible.
    ```yaml
      deploy:
        resources:
          reservations:
            devices:
              - driver: nvidia
                capabilities: [gpu]
    ```
  - If you don't have a GPU, comment out that section
4. Upload the contents of the `sample_models` directory to the `models` bucket vis the MinIO web UI at `localhost:9001` 
5. `cargo test`: Verify all cargo tests pass

## Model Inspection

`http://localhost:8000/v2/models/simple`

Will print model name and parameters required to set up the inputs and outputs.

## Errata

### gRPC Setup

proto folder will contain protocol buffers. Only `grpc_service.proto` is referenced in the build.rs
because `model_config.proto` is included by `grpc_service`. Generated code from tonic is in `inference.rs`

- Json is served on port 8000
- gRPC calls are submitted on on 8001
- Prometheus metrics are on 8002

### Multiplexing Tonic Channels

Submitting requests to a gRPC service requires a mutable reference to a Client. This prohibits you from
passing a single Client around to multiple Tasks and creates a bottleneck for async code.

Trying to hide this from users by wrapping what amounts to a synchronous resource in a struct and using
async message passing to access it might help some but still doesn't fix the core problem.

While it would be possible to make a connection pool of multiple `Client<Channel>`s and hide this pool in
a struct accessed with async message passing, this is complicated.

It also doesn't work to store a `tonic.transport.Channel` in the TritonClient struct...it requires the struct to implement
some obscure internal tonic traits. [tonic.transport.Channel](https://docs.rs/tonic/latest/tonic/transport/struct.Channel.html).

The idiomatic way appears to be storing a single master Client in a struct and then providing a function that returns a clone
of the Client since [Cloning clients is cheap](https://github.com/hyperium/tonic/issues/33).

A limitation of this could be that gRPC servers usually have a finite number of connections they can multiplex (100 seems
to be the number a lot of places throw out). See [gRPC performance best practices](https://grpc.io/docs/guides/performance/).

`tonic` seems to have a default buffer size of 1024. Source: `DEFAULT_BUFFER_SIZE` [channel/mod.rs](https://github.com/hyperium/tonic/blob/master/tonic/src/transport/channel/mod.rs)

This might be useful eventually if you have multiple Triton pods and want to discover which ones are live + update
the endpoint list [grpc load balancing](https://truelayer.com/blog/grpc-load-balancing-in-rust/),
[github](https://github.com/TrueLayer/ginepro).

Not clear if there's a connection pool under the hood there or how they're able to connect to multiple servers?

### Alternate Implementations/Inspiration

[Triton-client-rs](https://github.com/octoml/triton-client-rs)

### Integration with DALI

[DALI (Data Loading Library)](https://github.com/NVIDIA/dali)
[Triton DALI backend](https://github.com/triton-inference-server/dali_backend)
