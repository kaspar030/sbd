use std::collections::HashMap;

use anyhow::Result;
use camino::{Utf8Path, Utf8PathBuf};

pub type FileMap = HashMap<Utf8PathBuf, String>;

pub fn write_all(outpath: &Utf8Path, files: &FileMap, overwrite: bool) -> Result<()> {
    if !overwrite && outpath.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "Directory already exists",
        )
        .into());
    }

    for (file, content) in files {
        let file_path = outpath.join(file);
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(file_path, content)?;
    }

    Ok(())
}
