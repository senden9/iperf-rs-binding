use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let dst = autotools::build("vendor");

    //println!("cargo:rustc-link-search=native={}", dst.display());
    println!(
        "cargo:rustc-link-search=native={}",
        dbg!(dst.join("lib").display())
    );
    println!("cargo:rustc-link-lib=static=iperf");
    //println!("cargo:rustc-link-lib=iperf");

    // Dependencies of IPerf:
    for lib in ["ssl", "crypto", "m"] {
        println!("cargo:rustc-link-lib={}", lib);
    }

    let iperf_header_path = dst
        .join("include/iperf_api.h")
        .to_str()
        .expect("Failed to build header path")
        .to_string();

    let wrapper_header_path = out_path.join("wrapper.h");

    let wrapper_content = format!("#include <stdint.h> \n#include \"{}\"", iperf_header_path);
    std::fs::write(&wrapper_header_path, wrapper_content).unwrap();

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(wrapper_header_path.to_str().unwrap())
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
