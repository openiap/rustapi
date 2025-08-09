use std::ffi::CString;
use std::fs;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn bootstrap() -> *const c_char {
    let os = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    let debug = std::env::var("DEBUG").unwrap_or_else(|_| "".to_string());
    let openiap_skip_debug_check = std::env::var("OPENIAP_SKIP_DEBUG_CHECK").unwrap_or_else(|_| "".to_string());

    let mut libdir = std::env::current_dir().unwrap_or_else(|_| {
        std::path::PathBuf::from(".")
    });

    if openiap_skip_debug_check.is_empty() {
        // for 6 parent directories, check if target/debug exists
        for _ in 0..10 {
            let s = libdir.join("target").join("debug");
            if !debug.is_empty() { println!("checking for target/debug: {}", s.display()); }
            if libdir.join("target").join("debug").exists() {
                libdir = libdir.join("target").join("debug");
                break;
            }
            if libdir.join("target").join("release").exists() {
                libdir = libdir.join("target").join("release");
                break;
            }
            if !libdir.pop() {
                // If we can't pop, we are at the root directory
                break;
            }
        }
    }

    let debug_lib = match (os, arch) {
        ("windows", "x86") => "openiap_clib.dll",
        ("windows", "x86_64") => "openiap_clib.dll",
        ("windows", "aarch64") => "openiap_clib.dll",
        ("linux", "x86_64") => {
            // Check for musl (Alpine Linux)
            if std::path::Path::new("/etc/alpine-release").exists() {
                "libopeniap_clib_musl.so"
            } else {
                "libopeniap_clib.so"
            }
        },
        ("linux", "aarch64") => {
            if std::path::Path::new("/etc/alpine-release").exists() {
                "libopeniap_clib_musl.so"
            } else {
                "libopeniap_clib.so"
            }
        },
        ("macos", "x86_64") => "libopeniap_clib.dylib",
        ("macos", "aarch64") => "libopeniap_clib.dylib",
        ("freebsd", "x86_64") => "libopeniap_clib.so",
        _ => {
            let state = format!("Error: Unsupported platform {}-{}", os, arch);
            let state_str = CString::new(state).unwrap().into_raw();
            return state_str;
        }
    };
    if libdir.join(debug_lib).exists() {
        let debug_path = libdir.join(debug_lib);
        let absolute_path = match debug_path.canonicalize() {
            Ok(path) => path,
            Err(_) => debug_path.clone(),
        };
        let state = format!("{}", absolute_path.display());
        let state_str = CString::new(state).unwrap().into_raw();
        return state_str;
    }

     let lib_name = match (os, arch) {
        ("windows", "x86") => "openiap-windows-i686.dll",
        ("windows", "x86_64") => "openiap-windows-x64.dll",
        ("windows", "aarch64") => "openiap-windows-arm64.dll",
        ("linux", "x86_64") => {
            if std::path::Path::new("/etc/alpine-release").exists() {
                "libopeniap-linux-musl-x64.a"
            } else {
                "libopeniap-linux-x64.so"
            }
        },
        ("linux", "aarch64") => {
            if std::path::Path::new("/etc/alpine-release").exists() {
                "libopeniap-linux-musl-arm64.a"
            } else {
                "libopeniap-linux-arm64.so"
            }
        },
        ("macos", "x86_64") => "libopeniap-macos-x64.dylib",
        ("macos", "aarch64") => "libopeniap-macos-arm64.dylib",
        ("freebsd", "x86_64") => "libopeniap-freebsd-x64.so",
        _ => {
            let state = format!("Error: Unsupported platform {}-{}", os, arch);
            let state_str = CString::new(state).unwrap().into_raw();
            return state_str;
        }
    };
    let version = env!("CARGO_PKG_VERSION");
    if !debug.is_empty() { println!("Using library: {} version {}", lib_name, version); }

    let url = format!(
        "https://github.com/openiap/rustapi/releases/{}/download/{}",
        version,
        lib_name
    );

    // Use system temp directory for cross-platform compatibility
    let libdir = std::env::temp_dir().join("openiap");
    if !debug.is_empty() { 
        println!("Using temp directory for lib: {}", libdir.display()); 
    }
    

    match fs::create_dir_all(&libdir) {
        Ok(_) => {}
        Err(e) => {
            let state = format!("Error: failed to create directory {}: {}", libdir.display(), e);
            let state_str = CString::new(state).unwrap().into_raw();
            return state_str;
        }
    };
    let dest = libdir.join(lib_name);
    if dest.exists() {
        let absolute_dest = match dest.canonicalize() {
            Ok(path) => path,
            Err(_) => dest.clone(),
        };
        let state = format!("{}", absolute_dest.display());
        let state_str = CString::new(state).unwrap().into_raw();
        return state_str;
    }

    if !debug.is_empty() { println!("downloading {} to {}", url, dest.display()); }
    let mut response = match reqwest::blocking::get(&url) {
        Ok(resp) => resp,
        Err(e) => {
            let state = format!("Error: failed to download {}: {}", url, e);
            let state_str = CString::new(state).unwrap().into_raw();
            return state_str;
        }
    };
    if !response.status().is_success() {
        let url = format!(
            "https://github.com/openiap/rustapi/releases/latest/download/{}",
            lib_name
        );
        if !debug.is_empty() { println!("downloading {} to {}", url, dest.display()); }
        response = match reqwest::blocking::get(&url) {
            Ok(resp) => resp,
            Err(e) => {
                let state = format!("Error: failed to download {}: {}", url, e);
                let state_str = CString::new(state).unwrap().into_raw();
                return state_str;
            }
        };

    }
    if !response.status().is_success() {
        let state = format!("Error: failed downloading: {}", response.status());
        let state_str = CString::new(state).unwrap().into_raw();
        return state_str;
    }
    let bytes = match response.bytes() {
        Ok(b) => b,
        Err(e) => {
            let state = format!("Error: failed to read response bytes: {}", e);
            let state_str = CString::new(state).unwrap().into_raw();
            return state_str;
        }
    };
    match fs::write(&dest, &bytes) {
        Ok(_) => {}
        Err(e) => {
            let state = format!("Error: failed to write to {}: {}", dest.display(), e);
            let state_str = CString::new(state).unwrap().into_raw();
            return state_str;
        }
    }
    //let state = format!("downloaded {} bytes", bytes.len());
    let absolute_dest = match dest.canonicalize() {
        Ok(path) => path,
        Err(_) => dest.clone(),
    };
    let state = format!("{}", absolute_dest.display());
    let state_str = CString::new(state).unwrap().into_raw();
    state_str
}
