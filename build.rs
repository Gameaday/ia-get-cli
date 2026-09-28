fn main() {
    // Embed the Windows application manifest (enables long-path support and
    // declares compatibility). This is a no-op on non-Windows targets.
    #[cfg(target_os = "windows")]
    embed_windows_manifest();
}

#[cfg(target_os = "windows")]
fn embed_windows_manifest() {
    match embed_manifest::embed_manifest_file("ia-get.exe.manifest") {
        Ok(_) => println!("cargo:warning=Windows manifest embedded successfully"),
        Err(e) => println!("cargo:warning=Failed to embed Windows manifest: {}", e),
    }
}
