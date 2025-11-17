use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_files = [
        ("../proto/rmcs_resource_api/model.proto", "model_descriptor.bin"),
        ("../proto/rmcs_resource_api/device.proto", "device_descriptor.bin"),
        ("../proto/rmcs_resource_api/group.proto", "group_descriptor.bin"),
        ("../proto/rmcs_resource_api/set.proto", "set_descriptor.bin"),
        ("../proto/rmcs_resource_api/data.proto", "data_descriptor.bin"),
        ("../proto/rmcs_resource_api/buffer.proto", "buffer_descriptor.bin"),
        ("../proto/rmcs_resource_api/slice.proto", "slice_descriptor.bin"),
        ("../proto/rmcs_resource_api/log.proto", "log_descriptor.bin")
    ];
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    for tuple in proto_files {
        let (fproto, fdescriptor) = tuple;
        tonic_prost_build::configure()
            .protoc_arg("--experimental_allow_proto3_optional") // for older systems
            .build_client(true)
            .build_server(true)
            .file_descriptor_set_path(out_dir.join(fdescriptor))
            .out_dir("./src")
            .compile_protos(&[fproto], &["../proto"])?;
    }

    Ok(())
}
