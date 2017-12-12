/// build.rs
/// Run custom functionality when a build executes
fn main() {
    use std::env::var;
    use std::path::Path;

    // When targeting x86_64-pc-windows-gnu, we need to include the DLL libraries
    // found in the lib/x86_64 directory
    if var("TARGET")
        .map(|target| target == "x86_64-pc-windows-gnu")
        .unwrap_or(false)
    {
        let dir = var("CARGO_MANIFEST_DIR").unwrap();
        println!(
            "cargo:rustc-link-search=native={}",
            Path::new(&dir).join("lib/x86_64").display()
        )
    }

    // When targeting i686-pc-windows-gnu, we need to include the DLL libraries
    // found in the lib/i686 directory
    if var("TARGET")
        .map(|target| target == "i686-pc-windows-gnu")
        .unwrap_or(false)
    {
        let dir = var("CARGO_MANIFEST_DIR").unwrap();
        println!(
            "cargo:rustc-link-search=native={}",
            Path::new(&dir).join("lib/i686").display()
        )
    }
}
