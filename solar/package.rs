//! Package definition and handling

use serde::{Deserialize, Serialize};

/// Package definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Package {
    /// Package name
    pub name: String,
    /// Package version
    pub version: String,
    /// Package description
    pub description: Option<String>,
    /// Package homepage
    pub homepage: Option<String>,
    /// Package license
    pub license: Option<String>,
    /// Package architecture
    pub arch: Vec<String>,
    /// Package dependencies
    pub depends: Vec<String>,
    /// Optional dependencies
    pub optdepends: Vec<String>,
    /// Package conflicts
    pub conflicts: Vec<String>,
    /// Package provides
    pub provides: Vec<String>,
    /// Package replaces
    pub replaces: Vec<String>,
    /// Package backup files
    pub backup: Vec<String>,
    /// Source URLs/files
    pub source: Vec<String>,
    /// Source checksums
    pub sha256sums: Vec<String>,
    /// Source signatures
    pub sha256sig: Vec<String>,
    /// Valid GPG keys
    pub validpgpkeys: Vec<String>,
    /// Build script (prepare)
    pub prepare: Option<String>,
    /// Build script (build)
    pub build: Option<String>,
    /// Build script (package)
    pub package: Option<String>,
}

impl Package {
    /// Get the full package identifier (name-version)
    pub fn id(&self) -> String {
        format!("{}-{}", self.name, self.version)
    }

    /// Get the package filename
    pub fn filename(&self) -> String {
        format!("{}-{}.solar", self.name, self.version)
    }

    /// Check if this package is compatible with the given architecture
    pub fn is_arch_compatible(&self, arch: &str) -> bool {
        self.arch.iter().any(|a| a == "any" || a == arch)
    }

    /// Check if this package conflicts with any installed packages
    pub fn has_conflicts(&self, installed: &[&str]) -> bool {
        self.conflicts.iter().any(|c| installed.contains(&c.as_str()))
    }
}

impl From<solarsuite::PackageDef> for Package {
    fn from(def: solarsuite::PackageDef) -> Self {
        Self {
            name: def.name,
            version: def.version,
            description: def.description,
            homepage: def.homepage,
            license: def.license,
            arch: def.arch,
            depends: def.depends,
            optdepends: def.optdepends,
            conflicts: def.conflicts,
            provides: def.provides,
            replaces: def.replaces,
            backup: def.backup,
            source: def.source,
            sha256sums: def.sha256sums,
            sha256sig: def.sha256sig,
            validpgpkeys: def.validpgpkeys,
            prepare: def.prepare,
            build: def.build,
            package: def.package,
        }
    }
}
