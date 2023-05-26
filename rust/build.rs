use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_files = [
        ("../proto/model.proto", "model_descriptor.bin"),
        ("../proto/device.proto", "device_descriptor.bin"),
        ("../proto/group.proto", "group_descriptor.bin"),
        ("../proto/data.proto", "data_descriptor.bin"),
        ("../proto/buffer.proto", "buffer_descriptor.bin")
    ];
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    for tuple in proto_files {
        let (fproto, fdescriptor) = tuple;
        tonic_build::configure()
            .protoc_arg("--experimental_allow_proto3_optional") // for older systems
            .build_client(true)
            .build_server(true)
            .file_descriptor_set_path(out_dir.join(fdescriptor))
            .out_dir("./src")
            .compile(&[fproto], &["../proto"])?;
    }

    Ok(())
}
