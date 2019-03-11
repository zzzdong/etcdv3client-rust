#[cfg(feature = "gen")]
extern crate protobuf_codegen_pure;

#[cfg(feature = "gen")]
fn generate_protobuf_binding_file() {
    protobuf_codegen_pure::run(protobuf_codegen_pure::Args {
        out_dir: "src/pb",
        includes: &["proto"],
        input: &["proto/rpc.proto", "proto/kv.proto", "proto/auth.proto"],
        ..Default::default()
    })
    .unwrap();
}

#[cfg(not(feature = "gen"))]
fn generate_protobuf_binding_file() {}

fn main() {
    generate_protobuf_binding_file()
}
