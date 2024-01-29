use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-env-changed=OUT_DIR");

    // Set OUT_DIR to a specific value (e.g., "out") if not already set
    env::set_var("OUT_DIR", env::var("OUT_DIR").unwrap_or("out".to_string()));

    tonic_build::compile_protos("./src/notifications/voting.proto")?;
    Ok(())
}
