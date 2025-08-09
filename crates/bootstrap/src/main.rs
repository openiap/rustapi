use std::ffi::CString;
use std::os::raw::c_char;
use std::path::PathBuf;

mod lib2;

fn main() -> Result<(), String> {
    let result_ptr = lib2::bootstrap();
    let result = unsafe { CString::from_raw(result_ptr as *mut c_char) };
    let result_str = result.to_string_lossy().into_owned();
    if result_str.starts_with("Error:") {
        return Err(result_str);
    }
    let dest = PathBuf::from(result_str);
    if !dest.exists() {
        return Err(format!("Error: downloaded file does not exist at {}", dest.display()));
    }
    println!("Library located at: {}", dest.display());

    Ok(())
}
