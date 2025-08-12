use std::error::Error;
use std::process::Command;

fn build_subscription_service() -> Result<(), Box<dyn Error>> {
    let output = Command::new("ls").output()?;
    println!("output: {}", String::from_utf8_lossy(&output.stdout));

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src/api/gen")
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .extern_path(".google.protobuf.Timestamp", "::prost_wkt_types::Timestamp")
        .file_descriptor_set_path("src/api/gen/description.bin")
        .protoc_arg("--experimental_allow_proto3_optional") // Add this line
        .compile_protos(&["proto/api.proto"], &["proto", "../"])?;

    if !output.status.success() {
        return Err(Box::from(format!(
            "protoc failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    if !output.status.success() {
        return Err(Box::from(format!(
            "protoc failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )));
    }

    Ok(())
}
