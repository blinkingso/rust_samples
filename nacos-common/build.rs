fn main() {
    protobuf_codegen_pure::Codegen::new()
        .out_dir("src/proto")
        .inputs(&["src/proto/nacos_grpc_service.proto"])
        .include("src/proto")
        .run()
        .expect("Codegen failed.");
}
