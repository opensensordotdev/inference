fn main() {
    std::env::set_var("PROTOC", "/usr/bin/protoc");
    tonic_build::configure()
        .build_server(false)
        .out_dir("src") // you can change the generated code's location
        .compile(
            &["proto/grpc_service.proto"],
            &["proto"], // specify the root location to search proto dependencies
        )
        .unwrap();
}
