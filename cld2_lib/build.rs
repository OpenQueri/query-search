use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=cpp/cld2_main.cpp");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let cpp_src      = manifest_dir.join("cpp/cld2_main.cpp");
    let include_dir  = manifest_dir.join("cpp/lib/include");
    let internal_dir = include_dir.join("internal");
    let lib_dir      = manifest_dir.join("cpp/lib");

    cc::Build::new()
        .cpp(true)
        .file(&cpp_src)
        .include(&include_dir)
        .include(&internal_dir)
        .flag("-std=c++11")
        .flag("-Wall")
        .compile("cld2_wrapper");

    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=dylib=cld2");
    println!("cargo:rustc-link-lib=static=cld2_wrapper");
    println!("cargo:rustc-link-lib=dylib=stdc++");

    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_dir.display());


    println!("cargo:warning=✅ cld2-rs-lib built! rpath set to {}", lib_dir.display());
}