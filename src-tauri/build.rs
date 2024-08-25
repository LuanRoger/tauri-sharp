use std::{error::Error, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    tauri_build::build();

    compile_protos(&["../protos/greet.proto"])?;

    Ok(())
}

fn compile_protos(protos: &[impl AsRef<Path>]) -> Result<(), Box<dyn Error>> {
    if protos.is_empty() {
        return Err("No proto files found".into());
    }

    let proto_path: &Path = protos[0].as_ref();
    let proto_dir = proto_path
        .parent()
        .expect("proto file should reside in a directory");

    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile(&[proto_path], &[proto_dir])?;

    Ok(())
}
