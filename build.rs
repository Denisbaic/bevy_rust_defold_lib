use std::env;
use std::path::PathBuf;

fn main() {
    // Определяем путь к папке target/{profile}/
    let profile = env::var("PROFILE").unwrap(); // "debug" или "release"
    let out_dir = PathBuf::from(format!("target/{}/", profile));

    // Генерируем заголовочный файл
    let header = cbindgen::generate(".").expect("Unable to generate bindings");
    let header_path = out_dir.join("rust_defold_try.h");

    // Записываем его в нужную директорию
    header.write_to_file(&header_path);

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=build.rs");

    println!("Generated header: {:?}", header_path);

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .use_core()
        .ctypes_prefix("cty")
        .derive_default(true)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");
}
