//! Filesystem operations and validation utilities used internally by the crate.
use std::{fs, io, path::Path};

/// Validates that `path` refers to a readable file.
pub fn validate_file(path: &Path) -> Result<(), io::Error> {
    let metadata = fs::metadata(path).map_err(|err| {
        io::Error::new(
            err.kind(),
            format!("cannot access file {}: {err}", path.display()),
        )
    })?;

    if !metadata.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("path is not a file: {}", path.display()),
        ));
    }

    fs::File::open(path).map(|_| ()).map_err(|err| {
        io::Error::new(
            err.kind(),
            format!("file is not readable: {}: {err}", path.display()),
        )
    })
}

/// Validates that `path` refers to a readable directory.
pub fn validate_directory(path: &Path) -> Result<(), io::Error> {
    let metadata = fs::metadata(path).map_err(|err| {
        io::Error::new(
            err.kind(),
            format!("cannot access directory {}: {err}", path.display()),
        )
    })?;

    if !metadata.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotADirectory,
            format!("path is not a directory: {}", path.display()),
        ));
    }

    fs::read_dir(path).map(|_| ()).map_err(|err| {
        io::Error::new(
            err.kind(),
            format!("directory is not readable: {}: {err}", path.display()),
        )
    })
}

/// Validates that `path` refers to a writeable file location..
pub fn validate_output_path(path: &Path) -> Result<(), io::Error> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));

    if !parent.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("parent directory does not exist: {}", parent.display()),
        ));
    }

    if !parent.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotADirectory,
            format!("parent path is not a directory: {}", parent.display()),
        ));
    }

    if path.exists() {
        if path.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::IsADirectory,
                format!("output path is a directory: {}", path.display()),
            ));
        }

        fs::OpenOptions::new().write(true).open(path).map_err(|e| {
            io::Error::new(
                e.kind(),
                format!("cannot write to {}: {}", path.display(), e),
            )
        })?;
    } else {
        let metadata = fs::metadata(parent).map_err(|e| {
            io::Error::new(
                e.kind(),
                format!("cannot access parent directory {}: {}", parent.display(), e),
            )
        })?;

        if metadata.permissions().readonly() {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                format!("parent directory is read-only: {}", parent.display()),
            ));
        }
    }

    Ok(())
}
