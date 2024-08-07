use std::{env, path::PathBuf};
// extern crate napi_build;
fn main() {
    //napi_build::setup();

    // add "/usr/local/node" to path environment variable
    // let path = env::var("PATH").unwrap();
    // let new_path = format!("/usr/local/node:{}", path);
    // env::set_var("PATH", new_path);
    

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