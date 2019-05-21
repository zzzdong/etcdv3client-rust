fn main() {
    // Build auth.proto
    tower_grpc_build::Config::new()
        .enable_server(true)
        .enable_client(true)
        .build(&["proto/auth.proto"], &["proto/"])
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));

    // Build kv.proto
    tower_grpc_build::Config::new()
        .enable_server(true)
        .enable_client(true)
        .build(&["proto/kv.proto"], &["proto/"])
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));

    // Build rpc
    tower_grpc_build::Config::new()
        .enable_server(true)
        .enable_client(true)
        .build(&["proto/rpc.proto"], &["proto"])
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));
}
