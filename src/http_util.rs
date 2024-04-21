use std::path::{Path, PathBuf};

use crate::HttpRequest;

pub fn split_method(input: &str) -> Option<(String, String, String)> {
    let mut parts = input.splitn(3, ' ');

    let method = parts.next()?;
    let path = parts.next()?;
    let version = parts.next()?;

    Some((method.to_string(), path.to_string(), version.to_string()))
}

pub fn get_valid_path(request: &HttpRequest) -> Result<String, String> {
    if !request.is_init || !request.valid {
        return Err("ERROR: Failed to fetch valid path, request is not valid!".to_string());
    }

    let base_path = Path::new("./public").canonicalize().map_err(|_| "Failed to resolve base path".to_string())?;
    let mut full_path = PathBuf::from(&base_path);

    full_path.push(request.get_file_name().trim_start_matches("/"));

    match full_path.canonicalize() {
        Ok(canonical_path) => {
            if canonical_path.starts_with(&base_path) {
                Ok(canonical_path.display().to_string())
            } else {
                Err("ERROR: User requested invalid file!".to_string())
            }
        },
        Err(_) => Err("Invalid path provided".to_string()),
    }
}