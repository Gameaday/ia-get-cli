fn main() {
    // Handle Windows-specific manifest for long path support
    #[cfg(target_os = "windows")]
    embed_windows_manifest();

    // Check if we're building for Android and provide guidance
    if let Ok(target) = std::env::var("TARGET") {
        if target.contains("android") {
            println!("cargo:warning=Building for Android target: {}", target);
            println!(
                "cargo:warning=For complete Android APK/AAB builds, use: ./scripts/build-mobile.sh"
            );
            println!("cargo:warning=For native libraries only, use: ./scripts/build-android-libs-only.sh");
        }
    }

    // Note: Full artifact packaging is handled by CI/CD workflow after build completion
    // The build script runs before the binary is created, so we can't package it here
    // For development builds, only native libraries are created - full APK requires Flutter build
    println!("cargo:warning=Build script completed - complete artifacts created by CI/CD workflow");
}

#[cfg(target_os = "windows")]
fn embed_windows_manifest() {
    match embed_manifest::embed_manifest_file("ia-get.exe.manifest") {
        Ok(_) => println!("cargo:warning=Windows manifest embedded successfully"),
        Err(e) => println!("cargo:warning=Failed to embed Windows manifest: {}", e),
    }
}
