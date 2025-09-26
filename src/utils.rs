use std::collections::{HashMap, HashSet};

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

pub fn compare(
    filemap: &FileMap,
    outpath: &Utf8Path,
) -> Result<(Vec<Utf8PathBuf>, Vec<Utf8PathBuf>, Vec<Utf8PathBuf>)> {
    use walkdir::WalkDir;
    let mut missing = Vec::new();
    let mut differing = Vec::new();
    let mut extra = Vec::new();
    let mut all = HashSet::new();

    for entry in WalkDir::new(outpath) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let file_path =
                Utf8PathBuf::from_path_buf(entry.path().to_path_buf()).map_err(|_| {
                    anyhow::anyhow!("non-utf8 file path: \"{}\"", entry.path().display())
                })?;

            all.insert(file_path.clone());

            if let Some(content) = filemap.get(&file_path) {
                fn contents_match(file_path: &Utf8Path, content: &str) -> Result<bool> {
                    let entry_content = std::fs::read_to_string(file_path)?;
                    // TODO: hash?
                    Ok(entry_content == content)
                }
                if entry.metadata()?.len() == content.len() as u64
                    || contents_match(&file_path, content)?
                {
                    // same
                } else {
                    differing.push(file_path);
                }
            } else {
                extra.push(file_path);
            }
        }
    }

    filemap
        .keys()
        .cloned()
        .collect::<HashSet<_>>()
        .difference(&all)
        .for_each(|file| {
            missing.push(file.to_owned());
        });

    Ok((differing, extra, missing))
}
