//! Configuration management for Solar

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use crate::error::{SolarError, Result};

/// Solar configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Root directory for package installation
    #[serde(default = "default_root")]
    pub root: PathBuf,
    
    /// Package database path
    #[serde(default)]
    pub db_path: Option<PathBuf>,
    
    /// Build directory
    #[serde(default)]
    pub build_dir: Option<PathBuf>,
    
    /// Cache directory
    #[serde(default)]
    pub cache_dir: Option<PathBuf>,
    
    /// Log directory
    #[serde(default)]
    pub log_dir: Option<PathBuf>,
    
    /// Repositories to use
    #[serde(default)]
    pub repositories: Vec<String>,
    
    /// Architecture to build for
    #[serde(default = "default_arch")]
    pub arch: String,
    
    /// Parallel build jobs
    #[serde(default = "default_jobs")]
    pub jobs: usize,
    
    /// Verbose output
    #[serde(default)]
    pub verbose: bool,
    
    /// Color output
    #[serde(default = "default_true")]
    pub color: bool,
}

fn default_root() -> PathBuf {
    PathBuf::from("/")
}

fn default_arch() -> String {
    "x86_64".to_string()
}

fn default_jobs() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}

fn default_true() -> bool {
    true
}

impl Default for Config {
    fn default() -> Self {
        let root = PathBuf::from("/");
        Self {
            root: root.clone(),
            db_path: Some(root.join("var/lib/solar")),
            build_dir: Some(root.join("var/cache/solar/build")),
            cache_dir: Some(root.join("var/cache/solar/pkg")),
            log_dir: Some(root.join("var/log/solar")),
            repositories: Vec::new(),
            arch: default_arch(),
            jobs: default_jobs(),
            verbose: false,
            color: true,
        }
    }
}

impl Config {
    /// Get the database path, resolving relative to root if needed
    pub fn db_path(&self) -> PathBuf {
        self.db_path.clone().unwrap_or_else(|| self.root.join("var/lib/solar"))
    }

    /// Get the build directory, resolving relative to root if needed
    pub fn build_dir(&self) -> PathBuf {
        self.build_dir.clone().unwrap_or_else(|| self.root.join("var/cache/solar/build"))
    }

    /// Get the cache directory, resolving relative to root if needed
    pub fn cache_dir(&self) -> PathBuf {
        self.cache_dir.clone().unwrap_or_else(|| self.root.join("var/cache/solar/pkg"))
    }

    /// Get the log directory, resolving relative to root if needed
    pub fn log_dir(&self) -> PathBuf {
        self.log_dir.clone().unwrap_or_else(|| self.root.join("var/log/solar"))
    }

    /// Load configuration from file or create default
    pub fn load(root: &Path) -> Result<Self> {
        let config_path = root.join("etc/solar/config.toml");
        
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let mut config: Config = toml::from_str(&content)
                .map_err(|e| SolarError::ConfigError(e.to_string()))?;
            
            // Set root if not specified
            if config.root == PathBuf::from("/") || config.root.as_os_str().is_empty() {
                config.root = root.to_path_buf();
            }
            
            Ok(config)
        } else {
            // Create default configuration with paths relative to root
            let config = Self {
                root: root.to_path_buf(),
                db_path: Some(root.join("var/lib/solar")),
                build_dir: Some(root.join("var/cache/solar/build")),
                cache_dir: Some(root.join("var/cache/solar/pkg")),
                log_dir: Some(root.join("var/log/solar")),
                repositories: Vec::new(),
                arch: default_arch(),
                jobs: default_jobs(),
                verbose: false,
                color: true,
            };
            
            // Create directories
            std::fs::create_dir_all(&config.db_path())?;
            std::fs::create_dir_all(&config.build_dir())?;
            std::fs::create_dir_all(&config.cache_dir())?;
            std::fs::create_dir_all(&config.log_dir())?;
            
            Ok(config)
        }
    }

    /// Save configuration to file
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| SolarError::ConfigError(e.to_string()))?;
        
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        std::fs::write(path, content)?;
        
        Ok(())
    }
}
