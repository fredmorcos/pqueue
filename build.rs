use std::env;
use std::path::Path;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let enum_config = cbindgen::EnumConfig::default();
    let enum_config = cbindgen::EnumConfig {
        prefix_with_name: true,
        ..enum_config
    };
    let config = cbindgen::Config::default();
    let config = cbindgen::Config {
        usize_is_size_t: true,
        enumeration: enum_config,
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
