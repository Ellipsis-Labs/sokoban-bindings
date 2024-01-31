use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {
    println!("cargo:rerun-if-changed=NULL");

    let bindgen_crate = env::var("CARGO_MANIFEST_DIR").unwrap();
    let header_target_location = env::var("SKB_HEADER_PATH")
        .as_deref()
        .map(PathBuf::from)
        .unwrap_or(
            Path::new(&bindgen_crate)
                .parent()
                .expect("in a cargo workspace")
                .join("examples")
                .join("c"),
        );
    cbindgen::Builder::new()
        .with_crate(&bindgen_crate)
        .with_parse_expand(&["sokoban-bindgen-example"])
        .with_language(cbindgen::Language::C)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(Path::new(&header_target_location).join("sokoban.h"));
}
