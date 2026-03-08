//! Package definition structures for Starlark-based package declarations

use serde::{Deserialize, Serialize};
use std::fmt;

/// Package metadata extracted from a Starlark definition
#[derive(
    Debug, Clone, Serialize, Deserialize, Default,
)]
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

impl fmt::Display for PackageDef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Package({}@{})", self.name, self.version)
    }
}

/// Context for building a package definition via Starlark
#[derive(Debug, Clone, Default)]
pub struct PackageContext {
    pkg: PackageDef,
}

impl PackageContext {
    pub fn new() -> Self {
        Self {
            pkg: PackageDef::default(),
        }
    }

    pub fn into_package(self) -> PackageDef {
        self.pkg
    }

    pub fn pkg(&mut self, name: String, version: String) {
        self.pkg.name = name;
        self.pkg.version = version;
    }

    pub fn set_description(&mut self, desc: String) {
        self.pkg.description = Some(desc);
    }

    pub fn set_homepage(&mut self, url: String) {
        self.pkg.homepage = Some(url);
    }

    pub fn set_license(&mut self, lic: String) {
        self.pkg.license = Some(lic);
    }

    pub fn set_arch(&mut self, archs: Vec<String>) {
        self.pkg.arch = archs;
    }

    pub fn set_depends(&mut self, deps: Vec<String>) {
        self.pkg.depends = deps;
    }

    pub fn set_optdepends(&mut self, deps: Vec<String>) {
        self.pkg.optdepends = deps;
    }

    pub fn set_conflicts(&mut self, pkgs: Vec<String>) {
        self.pkg.conflicts = pkgs;
    }

    pub fn set_provides(&mut self, pkgs: Vec<String>) {
        self.pkg.provides = pkgs;
    }

    pub fn set_replaces(&mut self, pkgs: Vec<String>) {
        self.pkg.replaces = pkgs;
    }

    pub fn set_backup(&mut self, files: Vec<String>) {
        self.pkg.backup = files;
    }

    pub fn set_source(&mut self, srcs: Vec<String>) {
        self.pkg.source = srcs;
    }

    pub fn set_sha256sums(&mut self, sums: Vec<String>) {
        self.pkg.sha256sums = sums;
    }

    pub fn set_prepare(&mut self, script: String) {
        self.pkg.prepare = Some(script);
    }

    pub fn set_build(&mut self, script: String) {
        self.pkg.build = Some(script);
    }

    pub fn set_package(&mut self, script: String) {
        self.pkg.package = Some(script);
    }
}

impl fmt::Display for PackageContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PackageContext({})", self.pkg.name)
    }
}
