pub mod project;
pub mod util;

pub mod built_info {
    // The file has been placed there by the build script.
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub const VERSION: &str = env!("GDM_VERSION");
