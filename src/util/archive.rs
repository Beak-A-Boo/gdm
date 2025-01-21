use std::path::Path;
use std::{fs, io, path::PathBuf};
use thiserror::Error;
use zip::result::ZipError;

#[derive(Error, Debug)]
pub enum ExtractError {
    #[error("IO Error")]
    IoError(#[from] std::io::Error),
    #[error("Zip Error")]
    ZipError(#[from] zip::result::ZipError),
}

pub fn extract(
    archive: &Path,
    target_directory: &Path,
    skip_empty_top_level_directory: Option<bool>,
) -> Result<(), ExtractError> {
    let file = fs::File::open(archive)?;
    let mut archive = zip::ZipArchive::new(file)?;

    let strip_top_level =
        skip_empty_top_level_directory.unwrap_or(true) && has_top_level_directory(&mut archive)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;

        let mut relative_path = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        if strip_top_level {
            relative_path = relative_path.components().skip(1).collect();
            if relative_path.components().count() == 0 {
                continue;
            }
        }

        let outpath = target_directory.join(relative_path);

        if (*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                fs::create_dir_all(p)?;
            }
            let mut outfile = fs::File::create(&outpath)?;
            io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(())
}

fn has_top_level_directory(archive: &mut zip::ZipArchive<fs::File>) -> Result<bool, ZipError> {
    if archive.len() < 2 {
        return Ok(false);
    }

    let mut toplevel_dir: Option<PathBuf> = None;
    for i in 0..archive.len() {
        let file = match archive.by_index(i)?.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        if let Some(toplevel_dir) = &toplevel_dir {
            if !file.starts_with(toplevel_dir) {
                return Ok(false);
            }
        } else {
            toplevel_dir = Some(file.components().take(1).collect());
        }
    }
    Ok(true)
}
