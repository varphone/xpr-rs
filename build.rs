use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let wrapper_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("wrapper.h");
    let wrapper_path = wrapper_path.to_str().unwrap();
    let mut wrapper = File::create(wrapper_path).unwrap();
    writeln!(wrapper, "#include <xpr/xpr_avframe.h>").unwrap();
    writeln!(wrapper, "#include <xpr/xpr_common.h>").unwrap();
    writeln!(wrapper, "#include <xpr/xpr_rtsp.h>").unwrap();
    writeln!(wrapper, "#include <xpr/xpr_streamblock.h>").unwrap();
    writeln!(wrapper, "#include <xpr/xpr_sys.h>").unwrap();
    writeln!(wrapper, "#include <xpr/xpr_utils.h>").unwrap();

    let bindings = bindgen::Builder::default()
        .header(wrapper_path)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .anon_fields_prefix("un")
        .derive_debug(true)
        .impl_debug(false)
        .derive_default(true)
        .derive_partialeq(true)
        .derive_eq(true)
        .impl_partialeq(true)
        .allowlist_function("xpr_.*")
        .allowlist_function("XPR_.*")
        .allowlist_type("XPR_.*")
        .allowlist_var("AV_.*")
        .allowlist_var("XPR_.*")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-lib=xpr");
}
