fn main() {
    built::write_built_file().expect("Failed to acquire build-time information");

    println!(
        "cargo:rustc-env=GDM_VERSION={}",
        option_env!("GDM_VERSION").unwrap_or("DEV")
    );
}
