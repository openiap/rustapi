use std::{env, path::PathBuf};
/// Generates the protobuf code.
fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap_or(".".into()));
    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("openiap.bin"))
        // .type_attribute("WatchEvent", "#[derive(serde::Serialize, serde::Deserialize)]")
        // .type_attribute("Workitem", "#[derive(serde::Serialize, serde::Deserialize)]")
        // .type_attribute("WorkItemQueue", "#[derive(serde::Serialize, serde::Deserialize)]")
        // .compile_well_known_types(false)
        .compile(&["proto/base.proto"], &["proto"])
        .unwrap_or_default();
}
