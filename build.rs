use regex::Regex;
use std::env;
use std::path::{Path, PathBuf};
use std::{fs, io};

fn copy_dir_recursive(from: impl AsRef<Path>, to: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&to)?;
    for maybe_entry in fs::read_dir(from)? {
        let entry = maybe_entry?;
        let item = entry.path();
        let dest = to.as_ref().join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir_recursive(item, dest)?;
        } else {
            fs::copy(item, dest)?;
        }
    }
    Ok(())
}

fn rewrite_headers(out_path: &PathBuf) {
    // Processing.NDI.DynamicLoad.h has a lot of anonymous unions that fuck up usability.
    // Hackily replace the whole thing.

    let vendored = Path::new("vendored/include");
    let final_includes = out_path.join("include");
    copy_dir_recursive(vendored, &final_includes)
        .expect("Copying vendored to out dir needs to succeed");

    let re = Regex::new(
        r#"union\s+?\{[\r\n]\s+(.+?;).*?[\r\n]\s+PROCESSINGNDILIB_DEPRECATED.+?;.*?[\r\n]\s+\};"#,
    )
    .expect("This crazy regex worked at one point");
    let broken = fs::read_to_string(vendored.join("Processing.NDI.DynamicLoad.h"))
        .expect("Need to load the DynamicLoad.h header");
    let less_broken = re
        .replace_all(&broken, |caps: &regex::Captures| caps[1].to_string())
        .to_string();

    fs::write(
        final_includes.join("Processing.NDI.DynamicLoad.h"),
        less_broken,
    )
    .expect("Need to write replacement file successfully");
}

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    rewrite_headers(&out_path);

    let includes = out_path.join("include");
    let bindings = bindgen::Builder::default()
        .header(
            includes
                .join("Processing.NDI.Lib.h")
                .into_os_string()
                .into_string()
                .expect("Expected path to header to be valid UTF-8"),
        )
        .clang_arg(format!("-I{}", includes.display()))
        .allowlist_type("NDIlib_v5")
        .prepend_enum_name(false)
        .newtype_enum("NDIlib_FourCC_audio_type_e")
        .newtype_enum("NDIlib_FourCC_video_type_e")
        .newtype_enum("NDIlib_frame_format_type_e")
        .newtype_enum("NDIlib_frame_type_e")
        .newtype_enum("NDIlib_recv_bandwidth_e")
        .newtype_enum("NDIlib_recv_color_format_e")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
