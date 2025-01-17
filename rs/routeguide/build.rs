const PROTO_FILE: &str = "../../protos/route_guide.proto";
const PROTO_DIR: &str = "../../protos";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .build_transport(true)
        .type_attribute(".", "#[derive(serde::Deserialize, serde::Serialize)]")
        .compile_protos(&[PROTO_FILE], &[PROTO_DIR])?;
    Ok(())
}
