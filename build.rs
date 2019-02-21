extern crate protoc_rust_grpc;

fn main() {
    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: "src/pb",
        includes: &["proto"],
        input: &["proto/rpc.proto", "proto/kv.proto", "proto/auth.proto"],
        rust_protobuf: true,
        ..Default::default()
    })
    .expect("protoc-rust-grpc");
}
