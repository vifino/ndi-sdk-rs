mod error;
mod find;
mod routing;
mod source;
pub mod sys;

pub use error::{NDIError, NDIResult};
pub use find::*;
pub use routing::*;
pub use source::*;

use libloading::{Library, Symbol};
use std::path::PathBuf;
use std::sync::LazyLock;
use std::{env, ffi, fs};

pub(crate) static HANDLE: LazyLock<NDILib> =
    LazyLock::new(|| initialize().expect("Expected to lazily initialize NDI library handle"));

pub(crate) struct NDILib {
    pub(crate) lib: *const sys::NDIlib_v5,
}
unsafe impl Send for NDILib {}
unsafe impl Sync for NDILib {}

/// Initialize the NDI Library
fn initialize() -> NDIResult<NDILib> {
    let ndi_lib = load_lib()?;
    let load_fn: Symbol<unsafe extern "C" fn() -> *const sys::NDIlib_v5> =
        unsafe { ndi_lib.get(b"NDIlib_v5_load\0")? };

    let v5 = unsafe { load_fn() };
    if v5.is_null() {
        return Err(NDIError::LoadV5Failed);
    }

    let Some(init) = (unsafe { (*v5).initialize }) else {
        return Err(NDIError::MissingSymbolV5("initialize"));
    };

    if unsafe { init() } {
        Ok(NDILib { lib: v5 })
    } else {
        Err(NDIError::InitializeFailed)
    }
}

/// Load the NDI dynamic library.
fn load_lib() -> NDIResult<Library> {
    let lib_name = if cfg!(target_os = "macos") {
        "libndi.dylib"
    } else if cfg!(unix) {
        "libndi.so.6"
    } else if cfg!(all(windows, target_pointer_width = "64")) {
        "Processing.NDI.Lib.x64.dll"
    } else if cfg!(all(windows, target_pointer_width = "32")) {
        "Processing.NDI.Lib.x86.dll"
    } else {
        panic!("NDI SDK is only supported on Linux, macOS and Windows x86/x64")
    };

    let mut search_paths = Vec::new();
    if let Ok(rtd) = env::var("NDI_RUNTIME_DIR_V6") {
        let p = PathBuf::from(rtd);
        if cfg!(windows) {
            search_paths.push(p.join("lib"));
        } else {
            search_paths.push(p);
        }
    };

    #[cfg(unix)]
    search_paths.push(PathBuf::from("/usr/local/lib"));
    #[cfg(all(unix, not(target_os = "macos")))]
    search_paths.push(PathBuf::from("/usr/lib"));

    for path in search_paths {
        let file = path.join(lib_name);
        if let Ok(true) = fs::exists(&file) {
            return Ok(unsafe { Library::new(file)? });
        }
    }

    // Only the operating system is left to guess.
    Ok(unsafe { Library::new(lib_name)? })
}

pub fn version() -> NDIResult<String> {
    let Some(version_fn) = (unsafe { (*HANDLE.lib).version }) else {
        return Err(NDIError::MissingSymbolV5("version"));
    };

    let version_ptr = unsafe { version_fn() };
    if version_ptr.is_null() {
        return Err(NDIError::UnexpectedNullPointer("version"));
    }
    let c_str = unsafe { ffi::CStr::from_ptr(version_ptr) };
    Ok(c_str.to_str().map(|s| s.to_owned())?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_alloc() -> NDIResult<()> {
        unsafe {
            let vp_1 = (*HANDLE.lib).version.unwrap()();
            let vp_2 = (*HANDLE.lib).version.unwrap()();
            // If those pointers change, it would probably allocate, hence need freeing.
            assert_eq!(vp_1, vp_2);
        }
        println!("Version: {}", version()?);
        Ok(())
    }
}
