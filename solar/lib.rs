//! Solar Linux Package Manager
//! 
//! A modern package manager for Linux systems with Starlark-based package definitions.

pub mod config;
pub mod database;
pub mod package;
pub mod repo;
pub mod build;
pub mod error;

pub use config::Config;
pub use database::Database;
pub use package::Package;
pub use repo::Repository;
pub use error::SolarError;

use solarsuite::SolarSuite;
use std::path::{Path, PathBuf};
use tracing::info;

/// Solar package manager core
pub struct Solar {
    config: Config,
    database: Database,
    suite: SolarSuite,
    root: PathBuf,
}

impl Solar {
    /// Create a new Solar package manager instance
    pub fn new(root: Option<PathBuf>) -> Result<Self, SolarError> {
        let root = root.unwrap_or_else(|| PathBuf::from("/"));
        let config = Config::load(&root)?;
        let database = Database::open(&root)?;
        let suite = SolarSuite::new()?;
        
        Ok(Self {
            config,
            database,
            suite,
            root,
        })
    }

    /// Get the root directory
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Get the configuration
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Get the database
    pub fn database(&self) -> &Database {
        &self.database
    }

    /// Get the SolarSuite interpreter
    pub fn suite(&self) -> &SolarSuite {
        &self.suite
    }

    /// Install a package by name
    pub async fn install(&mut self, name: &str) -> Result<(), SolarError> {
        info!("Installing package: {}", name);
        
        // Check if already installed
        if self.database.is_installed(name)? {
            return Err(SolarError::PackageAlreadyInstalled(name.to_string()));
        }

        // Find the package in repositories
        let pkg = self.find_package(name)?;
        
        // Collect dependencies first (to avoid borrow issues)
        let deps: Vec<String> = pkg.depends.clone();
        
        // Install dependencies first
        for dep in deps {
            if !self.database.is_installed(&dep)? {
                // Use boxed pin for recursive async call
                Box::pin(self.install(&dep)).await?;
            }
        }

        // Build and install the package
        self.build_and_install(&pkg).await?;
        
        Ok(())
    }

    /// Remove a package by name
    pub fn remove(&mut self, name: &str) -> Result<(), SolarError> {
        info!("Removing package: {}", name);
        
        if !self.database.is_installed(name)? {
            return Err(SolarError::PackageNotInstalled(name.to_string()));
        }

        self.database.remove(name)?;
        
        Ok(())
    }

    /// Query installed packages
    pub fn query(&self, pattern: Option<&str>) -> Result<Vec<Package>, SolarError> {
        self.database.query(pattern)
    }

    /// Search repositories for packages
    pub fn search(&self, pattern: &str) -> Result<Vec<Package>, SolarError> {
        let mut results = Vec::new();
        
        for repo_path in &self.config.repositories {
            if let Ok(repo) = Repository::load(repo_path) {
                for pkg in repo.packages() {
                    if pkg.name.contains(pattern) || 
                       pkg.description.as_ref().map(|d| d.contains(pattern)).unwrap_or(false) {
                        results.push(pkg.clone());
                    }
                }
            }
        }
        
        Ok(results)
    }

    /// Update package databases
    pub async fn update(&mut self) -> Result<(), SolarError> {
        info!("Updating package databases");
        
        let repo_paths: Vec<String> = self.config.repositories.clone();
        for repo_path in repo_paths {
            if let Ok(mut repo) = Repository::load(&repo_path) {
                repo.sync().await?;
            }
        }
        
        Ok(())
    }

    /// Upgrade installed packages
    pub async fn upgrade(&mut self) -> Result<(), SolarError> {
        info!("Upgrading installed packages");
        
        let installed = self.database.query(None)?;
        
        for pkg in installed {
            if let Ok(latest) = self.find_package(&pkg.name) {
                if latest.version != pkg.version {
                    self.install(&pkg.name).await?;
                }
            }
        }
        
        Ok(())
    }

    /// Find a package in repositories
    fn find_package(&self, name: &str) -> Result<Package, SolarError> {
        for repo_path in &self.config.repositories {
            if let Ok(repo) = Repository::load(repo_path) {
                if let Some(pkg) = repo.find(name) {
                    return Ok(pkg);
                }
            }
        }
        
        Err(SolarError::PackageNotFound(name.to_string()))
    }

    /// Build and install a package
    async fn build_and_install(&mut self, pkg: &Package) -> Result<(), SolarError> {
        info!("Building package: {}-{}", pkg.name, pkg.version);
        
        // Download sources
        let build_dir = self.config.build_dir().join(&pkg.name);
        std::fs::create_dir_all(&build_dir)?;
        
        // Run build scripts using the suite
        // TODO: Implement full build pipeline
        
        // Register in database
        self.database.add(pkg)?;
        
        Ok(())
    }
}
