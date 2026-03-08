//! Repository management

use crate::error::{SolarError, Result};
use crate::package::Package;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Repository configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryConfig {
    pub name: String,
    pub url: String,
    pub priority: u32,
}

/// Package repository
pub struct Repository {
    name: String,
    url: String,
    packages: Vec<Package>,
    path: PathBuf,
}

impl Repository {
    /// Load a repository from URL or path
    pub fn load(path_or_url: &str) -> Result<Self> {
        if path_or_url.starts_with("http://") || path_or_url.starts_with("https://") {
            // Remote repository - load from cache or return empty
            let name = path_or_url
                .split('/')
                .filter(|s| !s.is_empty())
                .last()
                .unwrap_or("remote")
                .to_string();
            
            Ok(Self {
                name,
                url: path_or_url.to_string(),
                packages: Vec::new(),
                path: PathBuf::from(path_or_url),
            })
        } else {
            // Local repository
            let path = PathBuf::from(path_or_url);
            
            if !path.exists() {
                return Ok(Self {
                    name: path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("local")
                        .to_string(),
                    url: path_or_url.to_string(),
                    packages: Vec::new(),
                    path,
                });
            }

            let name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("local")
                .to_string();

            let mut packages = Vec::new();
            
            // Look for package definitions in the repository
            for entry in walkdir::WalkDir::new(&path)
                .min_depth(1)
                .max_depth(3)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let entry_path = entry.path();
                
                // Check for solar.lua (Starlark package definition)
                if entry_path.file_name() == Some(std::ffi::OsStr::new("solar.lua")) {
                    let suite = solarsuite::SolarSuite::new()?;
                    match suite.parse_file(entry_path) {
                        Ok(pkg_def) => {
                            packages.push(Package::from(pkg_def));
                        }
                        Err(e) => {
                            tracing::warn!("Failed to parse package at {:?}: {}", entry_path, e);
                        }
                    }
                }
            }

            Ok(Self {
                name,
                url: path_or_url.to_string(),
                packages,
                path,
            })
        }
    }

    /// Get the repository name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get all packages in the repository
    pub fn packages(&self) -> &[Package] {
        &self.packages
    }

    /// Find a package by name
    pub fn find(&self, name: &str) -> Option<Package> {
        self.packages.iter().find(|p| p.name == name).cloned()
    }

    /// Search for packages matching a pattern
    pub fn search(&self, pattern: &str) -> Vec<&Package> {
        self.packages
            .iter()
            .filter(|p| {
                p.name.contains(pattern) ||
                p.description.as_ref().map(|d| d.contains(pattern)).unwrap_or(false)
            })
            .collect()
    }

    /// Sync the repository (download latest package list)
    pub async fn sync(&mut self) -> Result<()> {
        if self.url.starts_with("http://") || self.url.starts_with("https://") {
            // Download repository database
            let db_url = format!("{}/solar.db", self.url);
            
            let response = reqwest::get(&db_url).await
                .map_err(|e| SolarError::DownloadError(e.to_string()))?;
            
            if response.status().is_success() {
                let _db_content = response.text().await
                    .map_err(|e| SolarError::DownloadError(e.to_string()))?;
                
                // Parse the database (could be JSON, TOML, or custom format)
                // For now, just log that we synced
                tracing::info!("Synced repository: {}", self.name);
            }
        }
        
        Ok(())
    }

    /// Get the repository path
    pub fn path(&self) -> &Path {
        &self.path
    }
}
