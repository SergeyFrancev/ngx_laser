use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum NgxLaserError {
    #[error("File not found")]
    FileNotFound,
    #[error("Zero thumb size")]
    ZeroThumbSize,
    #[error("Invalid URI")]
    InvalidUri,
    #[error("Not allowed size")]
    NotAllowedSize,
    #[error("Config file should be valid .toml")]
    InvalidConfig,
    // #[error("Image error: {0:?}")]
    // CreateThumbError(#[from] image::ImageError),
    #[error("Io error: {0:?}")]
    Io(#[from] std::io::Error),
    #[error("Create thumb directory: {0:?}")]
    CreateDirectoryError(PathBuf),
    #[error("Thumb directory is invalid: {0:?}")]
    InvalidThumbDirectory(PathBuf),


    #[error("Path not found: {0:?}")]
    PathNotFound(PathBuf),
}
