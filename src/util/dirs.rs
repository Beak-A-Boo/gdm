use std::path::PathBuf;

use directories::ProjectDirs;
use path_clean::PathClean;

pub fn project_dirs() -> ProjectDirs {
    ProjectDirs::from("", "", "gdm").unwrap()
}

pub fn get_actual_path(path: Option<PathBuf>) -> PathBuf {
    path.unwrap_or_else(|| PathBuf::from(".")).clean()
}
