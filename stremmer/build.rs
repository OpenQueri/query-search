use std::env;
use std::path::PathBuf;

fn main(){
    let lib_path = PathBuf::from("cpp/lib").canonicalize().unwrap();
    println!("cargo:rustc-link-search=native={}",lib_path.display() );
    println!("cargo:rustc-link-lib=static=stemmer");

    println!("cargo:warning=Looking for library in: {}", lib_path.display());
    let bindings = bindgen::Builder::default()
        .header("cpp/include/libstemmer.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
    
    println!("cargo:rerun-if-changed=cpp/include/libstemmer.h");

}