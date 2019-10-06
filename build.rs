fn main() {
    // Build auth.proto
    tonic_build::configure()
        .build_server(false)
        .compile(&["proto/auth.proto"], &["proto/"])
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));

    // Build kv.proto
    tonic_build::configure()
        .build_server(false)
        .compile(&["proto/kv.proto"], &["proto/"])
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));

    // Build rpc
    tonic_build::configure()
        .build_server(false)
        .compile(&["proto/rpc.proto"], &["proto/"])
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));
}
