extern crate tower_grpc_build;

fn main() {
    tower_grpc_build::Config::new()
        .enable_server(true)
        .build(&["../proto/helloworld/helloworld.proto"], &["../proto/helloworld"])
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));
}
