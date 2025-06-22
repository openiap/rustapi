use anyhow::{anyhow, Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    /// Directory to place the downloaded library in
    #[arg(long, default_value = "lib")]
    dir: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    let lib_name = match (os, arch) {
        ("windows", "x86") => "openiap-windows-i686.dll",
        ("windows", "x86_64") => "openiap-windows-x64.dll",
        ("windows", "aarch64") => "openiap-windows-arm64.dll",
        ("linux", "x86_64") => "libopeniap-linux-x64.so",
        ("linux", "aarch64") => "libopeniap-linux-arm64.so",
        ("macos", "x86_64") => "libopeniap-macos-x64.dylib",
        ("macos", "aarch64") => "libopeniap-macos-arm64.dylib",
        ("freebsd", "x86_64") => "libopeniap-freebsd-x64.so",
        _ => return Err(anyhow!("Unsupported platform {}-{}", os, arch)),
    };

    let url = format!(
        "https://github.com/openiap/rustapi/releases/latest/download/{}",
        lib_name
    );

    fs::create_dir_all(&args.dir)?;
    let dest = args.dir.join(lib_name);
    if dest.exists() {
        println!("library already present at {}", dest.display());
        return Ok(());
    }

    println!("downloading {} to {}", url, dest.display());
    let response = reqwest::blocking::get(&url)?;
    if !response.status().is_success() {
        return Err(anyhow!("failed downloading: {}", response.status()));
    }
    let bytes = response.bytes()?;
    fs::write(&dest, &bytes)?;
    println!("downloaded {} bytes", bytes.len());
    Ok(())
}
