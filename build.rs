use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::str::from_utf8;

use bindgen;

const PROS_OMNIBUS: &str = "pros_omnibus.h";
const RS_FILE_OUT: &str = "pros_raw_gen.rs";

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let cc = Command::new("make")
        .arg("dbg_cc")
        .output()
        .expect("Failed to run make");

    let cc = std::str::from_utf8(&cc.stdout)
        .expect("Invalid utf8 from makefile")
        .trim();

    // Manually preprocess code
    let preprocess = Command::new(cc)
        // CC keeps comments for docs
        // -dD also adds #defines
        // https://gcc.gnu.org/onlinedocs/gcc/Preprocessor-Options.html
        .args("-CC -dD".split_ascii_whitespace())
        // Get PROS headers
        .args("-I ./include/".split_ascii_whitespace())
        // Input PROS core api
        .args("-E include/rust_bindgen_source.h".split_ascii_whitespace())
        // Output file spec
        .arg("-o")
        .arg(out_path.join(PROS_OMNIBUS))
        // Run the command
        .output()
        .expect("failed to preprocess code");

    let bindings = bindgen::Builder::default()
        .ctypes_prefix("::cortex_types")
        .use_core()
        .rustfmt_bindings(true)
        .generate_comments(true)
        .header(
            out_path
                .join(PROS_OMNIBUS)
                .as_path()
                .to_str()
                .expect("Weird UTF8 Error, check your filesystem"),
        )
        .generate()
        .expect("Failed to generate bindings")
        .write_to_file(out_path.join(RS_FILE_OUT))
        .expect("Couldn't write bindings");

    assert!(preprocess.status.success());
}
