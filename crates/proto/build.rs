use std::{env, path::PathBuf};
fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("openiap.bin"))
        // .type_attribute(".", "#[repr(C)]")
        // .type_attribute("SigninRequest", "#[derive(serde::Serialize, serde::Deserialize)]")
        // .type_attribute("QueryRequest", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("WatchEvent", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_well_known_types(false)
        .compile(&["proto/base.proto"], &["proto"])
        .unwrap();
}