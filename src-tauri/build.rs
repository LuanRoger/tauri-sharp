use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tauri_build::build();
    tonic_build::compile_protos("protos/greet.proto")?;

    Ok(())
}
