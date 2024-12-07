extern crate cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    cbindgen::Builder::new()
      .with_crate(crate_dir)
      .with_language(cbindgen::Language::C)
      .with_header("typedef struct Option_Client {
  int some_field; // Example field
} Option_Client;
")
      .generate()
      .expect("Unable to generate bindings")
      .write_to_file("../../clib_openiap.h");
}