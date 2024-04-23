use std::path::{Path, PathBuf};

use crate::{http_error::{http_errors, HttpError}, io_util::validate_path, HttpRequest};

pub fn split_method(input: &str) -> Option<(String, String, String)> {
    let mut parts = input.splitn(3, ' ');

    let method = parts.next()?;
    let path = parts.next()?;
    let version = parts.next()?;

    Some((method.to_string(), path.to_string(), version.to_string()))
}

pub fn get_valid_path(request: &HttpRequest) -> Result<String, HttpError> {
    if !request.is_init || !request.valid {
        return Err(http_errors::msg::internal_server_error("Failed to fetch valid path, request is not valid"));
    }

    let base_path = Path::new("./public").canonicalize().map_err(|e| HttpError::convert_from(e, Some("Failed to resolve the path")))?;
    let mut full_path = PathBuf::from(&base_path);

    full_path.push(request.get_file_name().trim_start_matches("/"));

    match full_path.canonicalize() {
        Ok(canonical_path) => {
            if canonical_path.starts_with(&base_path) {
                Ok(canonical_path.display().to_string())
            } else {
                Err(http_errors::msg::forbidden("User requested invalid file"))
            }
        },
        Err(_) => Err(http_errors::msg::not_found("Invalid path provided")),
    }
}