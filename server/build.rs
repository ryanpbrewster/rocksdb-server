extern crate tower_grpc_build;

fn main() {
    println!("cargo:rerun-if-changed=../proto");
    tower_grpc_build::Config::new()
        .enable_server(true)
        .build(&["../proto/kvstore.proto"], &["../proto"])
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));
}
