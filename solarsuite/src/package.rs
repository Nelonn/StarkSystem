//! Package definition structures for Starlark-based package declarations

use serde::{Deserialize, Serialize};

/// Package metadata extracted from a Starlark definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageDef {
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

impl Default for PackageDef {
    fn default() -> Self {
        Self {
            name: String::new(),
            version: String::new(),
            description: None,
            homepage: None,
            license: None,
            arch: vec!["any".to_string()],
            depends: Vec::new(),
            optdepends: Vec::new(),
            conflicts: Vec::new(),
            provides: Vec::new(),
            replaces: Vec::new(),
            backup: Vec::new(),
            source: Vec::new(),
            sha256sums: Vec::new(),
            sha256sig: Vec::new(),
            validpgpkeys: Vec::new(),
            prepare: None,
            build: None,
            package: None,
        }
    }
}

impl PackageDef {
    /// Create a new package definition with name and version
    pub fn new(name: String, version: String) -> Self {
        Self {
            name,
            version,
            ..Default::default()
        }
    }

    /// Get the package filename
    pub fn filename(&self) -> String {
        format!(
            "{}-{}-{}.pkg.tar.gz",
            self.name,
            self.version,
            std::env::consts::ARCH
        )
    }
}
