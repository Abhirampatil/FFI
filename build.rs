use std::env;
use std::path::PathBuf;

fn main() {
    // --- Compile the C++ library ---
    cc::Build::new()//Compile the C/C++ source as part of the Cargo build. lib.cpp will be compiled
        .cpp(true) 
        .file("cpp/lib.cpp")
        .flag_if_supported("-std=c++14")
        .warnings(true)
        .compile("proxie_cpp"); // compiles the C++ into a static lib

    // Re-run the build script if any C++ source/header changes. otherwise old stale cached build might be used instead
    // ins to cargo
    println!("cargo:rerun-if-changed=cpp/lib.cpp");
    println!("cargo:rerun-if-changed=cpp/wrapper.h");
    println!("cargo:rerun-if-changed=build.rs");

    // --- Generate Rust bindings from the C-compatible header ---
    // Bindgen Reads the C header (wrapper.h) and auto-generates the raw Rust FFI function declarations, so you don't hand-write them.

    let bindings = bindgen::Builder::default()
        .header("cpp/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Keep the generated surface small and predictable.
        .allowlist_function("str_length")
        .allowlist_function("str_reverse")
        .allowlist_function("count_vowels")
        .allowlist_function("to_uppercase")
        .allowlist_function("proxie_free_string")
        .generate() // reads the header, generates Rust bindings
        .expect("Unable to generate bindgen bindings from cpp/wrapper.h");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs")) // writes bindings.rs
        .expect("Couldn't write bindings.rs");
}
