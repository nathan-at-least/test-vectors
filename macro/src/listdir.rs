use std::path::Path;

pub(crate) fn list_dir(dir: &Path) -> std::io::Result<Vec<String>> {
    let mut names = vec![];
    for entres in dir.read_dir()? {
        let entry = entres?;
        if entry.path().metadata()?.is_dir() {
            let osname = entry.file_name();
            let name = osname.as_os_str().to_str().ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::Other, "invalid directory name")
            })?;
            names.push(name.to_string());
        }
    }
    Ok(names)
}

#[cfg(test)]
mod tests;
