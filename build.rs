fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    built::write_built_file().expect("Failed to acquire build-time information");

    println!(
        "cargo:rustc-env=GDM_VERSION={}",
        option_env!("GDM_VERSION").unwrap_or("DEV")
    );
}
