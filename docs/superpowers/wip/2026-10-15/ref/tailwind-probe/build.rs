fn main() {
    // Default: downloads the pinned Tailwind CLI for the HOST platform.
    // On Windows ARM64 set TAILWIND_CLI to a pre-downloaded x64 exe (see reference).
    let mut config = topcoat::tailwind::BuildConfig::new();
    if std::env::var_os("TAILWIND_CLI").is_some() {
        config = config.executable_env("TAILWIND_CLI");
    }
    println!("cargo:rerun-if-env-changed=TAILWIND_CLI");
    config.render().unwrap();
}
