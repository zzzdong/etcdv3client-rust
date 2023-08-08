fn main() {
    gen_pb_code()
}

#[cfg(not(feature = "gen"))]
fn gen_pb_code() {}

#[cfg(feature = "gen")]
fn gen_pb_code() {
    // Build auth.proto
    tonic_build::configure()
        .build_server(false)
        .out_dir("src/pb/")
        .compile(&["proto/auth.proto"], &["proto/"])
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));

    // Build kv.proto
    tonic_build::configure()
        .build_server(false)
        .out_dir("src/pb/")
        .compile(&["proto/kv.proto"], &["proto/"])
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));

    // Build rpc.proto
    tonic_build::configure()
        .build_server(false)
        .out_dir("src/pb/")
        .compile(&["proto/rpc.proto"], &["proto/"])
        .unwrap_or_else(|e| panic!("protobuf compilation failed: {}", e));
}
