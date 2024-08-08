#[cfg(windows)]
pub const NPM: &'static str = "npm.cmd";

#[cfg(not(windows))]
pub const NPM: &'static str = "npm";

use npm_rs::*;

fn main() {
    println!("cargo:rerun-if-changed=openiap-napi");
    println!("cargo:rerun-if-changed=openiap-napi-postbuild");

    let exit_status = NpmEnv::default()
       .with_node_env(&NodeEnv::Development)
       .set_path("../openiap-napi")
       .init_env()
       // .install(None)
       .run("build")
       .exec();
    match exit_status {
        Ok(status) => {
            if !status.success() {
                panic!("npm run build failed: {:?}", status);
            }
        }
        Err(err) => {
            panic!("npm run build failed: {:?}", err);
        }
    }
    // if !exit_status.is_err() {
    //     let err = exit_status.unwrap_err();
    //     panic!("npm run build failed: {:?}", err);
    // }


    // // run npm install
    // let output = std::process::Command::new(NPM)
    //     .arg("install")
    //     .current_dir("openiap-napi-postbuild")
    //     .output()
    //     .expect("Failed to run npm install");
    // if !output.status.success() {
    //     panic!("npm install failed: {}", String::from_utf8_lossy(&output.stderr));
    // }
    // // run npm run build
    // let output = std::process::Command::new(NPM)
    //     .arg("run")
    //     .arg("build")
    //     .current_dir("openiap-napi-postbuild")
    //     .output()
    //     .expect("Failed to run npm run build");
    // if !output.status.success() {
    //     panic!("npm run build failed: {}", String::from_utf8_lossy(&output.stderr));
    // }
    // copy the built files to the target directory
    // std::fs::copy("openiap-napi-postbuild/dist/index.js", "openiap-napi-postbuild/index.js")
    //     .expect("Failed to copy index.js");
    // std::fs::copy("openiap-napi-postbuild/dist/index.d.ts", "openiap-napi-postbuild/index.d.ts")
    //     .expect("Failed to copy index.d.ts");
    // std::fs::copy("openiap-napi-postbuild/package.json", "openiap-napi-postbuild/package.json")
    //     .expect("Failed to copy package.json");
    // std::fs::copy("openiap-napi-postbuild/package-lock.json", "openiap-napi-postbuild/package-lock.json")
    //     .expect("Failed to copy package-lock.json");
    // std::fs::copy("openiap-napi-postbuild/README.md", "openiap-napi-postbuild/README.md")
    //     .expect("Failed to copy README.md");

}