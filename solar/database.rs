//! Package database management

use crate::error::{SolarError, Result};
use crate::package::Package;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Installed package record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstalledPackage {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub install_date: u64,
    pub files: Vec<String>,
    pub depends: Vec<String>,
    pub optdepends: Vec<String>,
    pub conflicts: Vec<String>,
    pub provides: Vec<String>,
}

impl From<&Package> for InstalledPackage {
    fn from(pkg: &Package) -> Self {
        Self {
            name: pkg.name.clone(),
            version: pkg.version.clone(),
            description: pkg.description.clone(),
            install_date: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            files: Vec::new(),
            depends: pkg.depends.clone(),
            optdepends: pkg.optdepends.clone(),
            conflicts: pkg.conflicts.clone(),
            provides: pkg.provides.clone(),
        }
    }
}

/// Package database
pub struct Database {
    path: PathBuf,
    packages: HashMap<String, InstalledPackage>,
}

impl Database {
    /// Open the package database
    pub fn open(root: &Path) -> Result<Self> {
        let db_path = root.join("var/lib/solar");
        std::fs::create_dir_all(&db_path)?;
        
        let mut db = Self {
            path: db_path,
            packages: HashMap::new(),
        };
        
        db.load()?;
        
        Ok(db)
    }

    /// Load the database from disk
    fn load(&mut self) -> Result<()> {
        let local_path = self.path.join("local");
        
        if !local_path.exists() {
            std::fs::create_dir_all(&local_path)?;
            return Ok(());
        }

        for entry in std::fs::read_dir(&local_path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_string();
                
                let desc_path = path.join("desc");
                if desc_path.exists() {
                    let content = std::fs::read_to_string(&desc_path)?;
                    let pkg: InstalledPackage = serde_json::from_str(&content)
                        .map_err(|e| SolarError::DatabaseError(e.to_string()))?;
                    
                    self.packages.insert(name, pkg);
                }
            }
        }

        Ok(())
    }

    /// Save the database to disk
    fn save(&self) -> Result<()> {
        let local_path = self.path.join("local");
        
        for (name, pkg) in &self.packages {
            let pkg_path = local_path.join(name);
            std::fs::create_dir_all(&pkg_path)?;
            
            let desc_path = pkg_path.join("desc");
            let content = serde_json::to_string_pretty(pkg)
                .map_err(|e| SolarError::DatabaseError(e.to_string()))?;
            
            std::fs::write(&desc_path, content)?;
        }

        Ok(())
    }

    /// Check if a package is installed
    pub fn is_installed(&self, name: &str) -> Result<bool> {
        Ok(self.packages.contains_key(name))
    }

    /// Get an installed package
    pub fn get(&self, name: &str) -> Option<&InstalledPackage> {
        self.packages.get(name)
    }

    /// Add a package to the database
    pub fn add(&mut self, pkg: &Package) -> Result<()> {
        let installed = InstalledPackage::from(pkg);
        self.packages.insert(pkg.name.clone(), installed);
        self.save()?;
        Ok(())
    }

    /// Remove a package from the database
    pub fn remove(&mut self, name: &str) -> Result<()> {
        self.packages.remove(name);
        self.save()?;
        
        // Remove the package directory
        let pkg_path = self.path.join("local").join(name);
        if pkg_path.exists() {
            std::fs::remove_dir_all(&pkg_path)?;
        }
        
        Ok(())
    }

    /// Query packages
    pub fn query(&self, pattern: Option<&str>) -> Result<Vec<Package>> {
        let mut results = Vec::new();
        
        for pkg in self.packages.values() {
            if let Some(pat) = pattern {
                if pkg.name.contains(pat) {
                    results.push(Package {
                        name: pkg.name.clone(),
                        version: pkg.version.clone(),
                        description: pkg.description.clone(),
                        homepage: None,
                        license: None,
                        arch: vec!["any".to_string()],
                        depends: pkg.depends.clone(),
                        optdepends: pkg.optdepends.clone(),
                        conflicts: pkg.conflicts.clone(),
                        provides: pkg.provides.clone(),
                        replaces: Vec::new(),
                        backup: Vec::new(),
                        source: Vec::new(),
                        sha256sums: Vec::new(),
                        sha256sig: Vec::new(),
                        validpgpkeys: Vec::new(),
                        prepare: None,
                        build: None,
                        package: None,
                    });
                }
            } else {
                results.push(Package {
                    name: pkg.name.clone(),
                    version: pkg.version.clone(),
                    description: pkg.description.clone(),
                    homepage: None,
                    license: None,
                    arch: vec!["any".to_string()],
                    depends: pkg.depends.clone(),
                    optdepends: pkg.optdepends.clone(),
                    conflicts: pkg.conflicts.clone(),
                    provides: pkg.provides.clone(),
                    replaces: Vec::new(),
                    backup: Vec::new(),
                    source: Vec::new(),
                    sha256sums: Vec::new(),
                    sha256sig: Vec::new(),
                    validpgpkeys: Vec::new(),
                    prepare: None,
                    build: None,
                    package: None,
                });
            }
        }
        
        Ok(results)
    }

    /// List all installed packages
    pub fn list(&self) -> Result<Vec<&InstalledPackage>> {
        Ok(self.packages.values().collect())
    }
}
