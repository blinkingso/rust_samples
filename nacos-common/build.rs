use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // protobuf_codegen_pure::Codegen::new()
    //     .out_dir("src/proto")
    //     .inputs(&["src/proto/nacos_grpc_service.proto"])
    //     .include("src/proto")
    //     .run()
    //     .expect("Codegen failed.");
    // tonic_build::configure()
    //     .out_dir("src/proto/nacos")
    //     .file_descriptor_set_path(PathBuf::from("proto").join("nacos_grpc_service_descriptor.bin"))
    // .compile(&["proto/nacos_grpc_service.proto"], &["proto"])
    // .unwrap();
    Ok(())
}
