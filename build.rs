use std::env;
use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=src/model/kafka_client.proto");
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("OUT_DIR is {}", out_dir);

    let mut config = prost_build::Config::new();
    config.message_attribute(".", "#[derive(derive_builder::Builder)]");
    config.compile_protos(&["src/model/kafka_client.proto"], &["src/"])?;
    // prost_build::compile_protos(&["src/model.proto"], &["src/"])?;

    // Specify your desired output directory here
    let dest_path = Path::new("src/").join("model");
    fs::create_dir_all(&dest_path)?;

    // Copy the generated files to your desired directory
    for entry in fs::read_dir(&out_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension() == Some(std::ffi::OsStr::new("rs")) {
            fs::copy(&path, &dest_path.join(path.file_name().unwrap()))?;
        }
    }

    Ok(())
}
