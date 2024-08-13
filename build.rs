use std::{env, path::PathBuf};

fn main() {
    let proto = "./protos/gogh.v1.proto";
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let plugins = "#[derive(serde::Serialize, serde::Deserialize)]";

    tonic_build::configure()
        .type_attribute("GenerateImageRequest", plugins)
        .type_attribute("GenerateImageResponse", plugins)
        .type_attribute("GetImageRequest", plugins)
        .type_attribute("GetImageResponse", plugins)
        .server_mod_attribute("attrs", "#[cfg(feature = \"server\")]")
        .build_server(true)
        .build_client(false)
        .out_dir(out_dir)
        .compile(&[proto], &["protos"])
        .unwrap_or_else(|e| panic!("Failed to compile protos: {:?}", e));

    println!("cargo:rerun-if-changed=protos/genplat.proto");
}
