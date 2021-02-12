use std::path::Path;

fn generate_bindings(outdir: &Path, headerfile: &str, filter: &str) {
    let includedir = outdir.join("build").join("include");
    bindgen::Builder::default()
        .clang_arg(format!("-I{}", includedir.display()))
        .header(
            includedir
                .join("oqs")
                .join(format!("{}.h", headerfile))
                .to_str()
                .unwrap(),
        )
        // Options
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .size_t_is_usize(true)
        // Don't generate docs unless enabled
        // Otherwise it breaks tests
        .generate_comments(cfg!(feature = "docs"))
        // Whitelist OQS stuff
        .whitelist_recursively(false)
        .whitelist_type(filter)
        .whitelist_function(filter)
        .whitelist_var(filter)
        // Use core and libc
        .use_core()
        .ctypes_prefix("::libc")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings")
        .write_to_file(outdir.join(format!("{}_bindings.rs", headerfile)))
        .expect("Couldn't write bindings!");
}

fn main() {
    // Check if clang is available before compiling anything.
    bindgen::clang_version();

    let mut config = cmake::Config::new("liboqs");
    config.profile("Release");
    config.define("OQS_BUILD_ONLY_LIB", "Yes");
    config.define("BUILD_SHARED_LIBS", "Yes");
    if cfg!(feature = "openssl") {
        config.define("OQS_USE_OPENSSL", "Yes");
        println!("cargo:rustc-link-lib=crypto");
    } else {
        config.define("OQS_USE_OPENSSL", "No");
    }
    let outdir = config.build_target("oqs").build();

    // lib is put into $outdir/build/lib
    let libdir = outdir.join("build").join("lib");
    println!("cargo:rustc-link-search=native={}", libdir.display());
    println!("cargo:rustc-link-lib=oqs");
    let gen_bindings = |file, filter| generate_bindings(&outdir, file, filter);

    gen_bindings("common", "OQS_.*");
    gen_bindings("kem", "OQS_KEM.*");
    gen_bindings("sig", "OQS_SIG.*");

    // https://docs.rs/build-deps/0.1.4/build_deps/fn.rerun_if_changed_paths.html
    build_deps::rerun_if_changed_paths("liboqs/src/**/*").unwrap();
    build_deps::rerun_if_changed_paths("liboqs/src").unwrap();
    build_deps::rerun_if_changed_paths("liboqs/src/*").unwrap();
}
