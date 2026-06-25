fn main() {
    let dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = std::path::Path::new(&dir).join("artifacts.json");

    if !path.exists() {
        // Stub so the codebase compiles without the private artifacts repo.
        // Release binaries are built in CI with the real list.
        std::fs::write(&path, r#"{"version":"0.0.0-stub","categories":[]}"#).unwrap();
    }

    println!("cargo:rerun-if-changed=artifacts.json");
}
