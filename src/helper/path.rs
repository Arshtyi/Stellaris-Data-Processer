use shellexpand;
use std::path::{Path, PathBuf};

fn expand_to_absolute(input_path: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let expanded_path = shellexpand::env(input_path)?;
    let path = Path::new(expanded_path.as_ref());

    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(std::env::current_dir()?.join(path))
    }
}

pub fn resolve_path(input_path: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let absolute_path = expand_to_absolute(input_path)?;

    std::fs::canonicalize(&absolute_path).map_err(|_| {
        Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Path not found: {}", absolute_path.display()),
        )) as Box<dyn std::error::Error>
    })
}

pub fn resolve_path_and_create(input_path: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let absolute_path = expand_to_absolute(input_path)?;

    if !absolute_path.exists() {
        std::fs::create_dir_all(&absolute_path)?;
        println!(
            "Created directory: {} as the output path.",
            absolute_path.display()
        );
    }

    Ok(std::fs::canonicalize(&absolute_path)?)
}
