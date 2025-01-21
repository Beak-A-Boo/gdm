use anyhow::anyhow;
use directories::ProjectDirs;
use path_clean::PathClean;
use std::env;
use std::path::PathBuf;
use tokio::fs;

#[derive(Debug, Clone)]
pub struct Dirs {
    pub project_dir: PathBuf,
    pub absolute_project_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub download_dir: PathBuf,
    pub engines_install_dir: PathBuf,
}

pub async fn init(project_path: Option<PathBuf>) -> anyhow::Result<Dirs> {
    init0(project_path, true).await
}

async fn init0(project_path: Option<PathBuf>, init_project: bool) -> anyhow::Result<Dirs> {
    let engines_install_dir: PathBuf;
    let download_dir: PathBuf;
    let cache_dir: PathBuf;

    match env::var("GDM_USER_HOME").ok().map(PathBuf::from) {
        None => {
            // empty strings for qualifier and org name are fine
            let project_dirs = ProjectDirs::from("", "", "gdm")
                .ok_or(anyhow!("Could not read global directories"))?;
            cache_dir = project_dirs.cache_dir().to_path_buf();
            let data_local_dir = project_dirs.data_local_dir().to_path_buf();

            engines_install_dir = data_local_dir.join("engines");
            download_dir = cache_dir.join("downloads");
        }
        Some(gdm_home) => {
            cache_dir = gdm_home.join("cache");
            engines_install_dir = gdm_home.join("engines");
            download_dir = gdm_home.join("downloads");
        }
    }

    let mut result = Dirs {
        project_dir: PathBuf::from("."),
        absolute_project_dir: PathBuf::from("."),

        cache_dir,
        download_dir,

        engines_install_dir,
    };

    if init_project {
        result.project_dir = project_path.unwrap_or(result.project_dir).clean();
        fs::create_dir_all(&result.project_dir).await?;
        result.absolute_project_dir = dunce::canonicalize(&result.project_dir)?
    }

    Ok(result)
}

pub async fn init_no_project() -> anyhow::Result<Dirs> {
    init0(None, false).await
}
