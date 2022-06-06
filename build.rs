use std::env;
use std::path::Path;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let config = cbindgen::Config::default();
    let config = cbindgen::Config {
        usize_is_size_t: true,
        ..config
    };

    cbindgen::Builder::new()
        .with_config(config)
        .with_language(cbindgen::Language::C)
        .with_crate(&crate_dir)
        .generate()
        .unwrap()
        .write_to_file(Path::new(&crate_dir).join("c").join("pqueue.h"));

    let lib = Path::new(&crate_dir).join("src").join("lib.rs");
    println!("cargo:rerun-if-changed={}", lib.display());
}
