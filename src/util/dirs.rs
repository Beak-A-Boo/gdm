use directories::ProjectDirs;

pub fn project_dirs() -> ProjectDirs {
    ProjectDirs::from("", "", "gdm").unwrap()
}
