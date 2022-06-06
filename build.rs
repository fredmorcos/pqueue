use std::env;
use std::path::Path;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    cbindgen::Builder::new()
        .with_language(cbindgen::Language::C)
        .with_crate(&crate_dir)
        .generate()
        .unwrap()
        .write_to_file(Path::new(&crate_dir).join("c").join("pqueue.h"));

    let lib = Path::new(&crate_dir).join("src").join("lib.rs");
    println!("cargo:rerun-if-changed={}", lib.display());
}
