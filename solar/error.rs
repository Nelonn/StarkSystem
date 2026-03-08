//! Error types for Solar package manager

use thiserror::Error;
use solarsuite::SolarSuiteError;

/// Solar package manager errors
#[derive(Error, Debug)]
pub enum SolarError {
    #[error("Package not found: {0}")]
    PackageNotFound(String),
    
    #[error("Package already installed: {0}")]
    PackageAlreadyInstalled(String),
    
    #[error("Package not installed: {0}")]
    PackageNotInstalled(String),
    
    #[error("Dependency resolution failed: {0}")]
    DependencyError(String),
    
    #[error("Build failed: {0}")]
    BuildError(String),
    
    #[error("Download failed: {0}")]
    DownloadError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Repository error: {0}")]
    RepositoryError(String),
    
    #[error("Starlark error: {0}")]
    StarlarkError(#[from] SolarSuiteError),
    
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, SolarError>;
