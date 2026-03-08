//! SolarSuite - Starlark-based package definition interpreter
//! 
//! This crate provides the core functionality for interpreting package definitions
//! written in Starlark (a Python-like dialect) for the Solar package manager.

use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

/// Errors that can occur during package definition parsing
#[derive(Error, Debug)]
pub enum SolarSuiteError {
    #[error("Failed to parse package file: {0}")]
    ParseError(String),
    #[error("Invalid package definition: {0}")]
    InvalidPackage(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

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

/// Simple Starlark parser for Solar package definitions
pub struct SolarSuite;

impl SolarSuite {
    /// Create a new SolarSuite interpreter
    pub fn new() -> Result<Self, SolarSuiteError> {
        Ok(Self)
    }

    /// Parse and evaluate a Starlark file, returning the package definition
    pub fn parse_file(&self, path: &Path) -> Result<PackageDef, SolarSuiteError> {
        let content = std::fs::read_to_string(path)?;
        self.parse_content(&content)
    }

    /// Parse Starlark content, returning the package definition
    pub fn parse_content(&self, content: &str) -> Result<PackageDef, SolarSuiteError> {
        let mut pkg = PackageDef::default();
        
        // Simple line-by-line parsing for Starlark-like syntax
        for line in content.lines() {
            let line = line.trim();
            
            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            // Parse pkg() call
            if let Some((name, version)) = self.parse_pkg_call(line) {
                pkg.name = name;
                pkg.version = version;
                continue;
            }
            
            // Parse description() call
            if let Some(value) = self.parse_string_func(line, "description") {
                pkg.description = Some(value);
                continue;
            }
            
            // Parse homepage() call
            if let Some(value) = self.parse_string_func(line, "homepage") {
                pkg.homepage = Some(value);
                continue;
            }
            
            // Parse license() call
            if let Some(value) = self.parse_string_func(line, "license") {
                pkg.license = Some(value);
                continue;
            }
            
            // Parse arch() call
            if let Some(value) = self.parse_list_func(line, "arch") {
                pkg.arch = value;
                continue;
            }
            
            // Parse depends() call
            if let Some(value) = self.parse_list_func(line, "depends") {
                pkg.depends = value;
                continue;
            }
            
            // Parse optdepends() call
            if let Some(value) = self.parse_list_func(line, "optdepends") {
                pkg.optdepends = value;
                continue;
            }
            
            // Parse conflicts() call
            if let Some(value) = self.parse_list_func(line, "conflicts") {
                pkg.conflicts = value;
                continue;
            }
            
            // Parse provides() call
            if let Some(value) = self.parse_list_func(line, "provides") {
                pkg.provides = value;
                continue;
            }
            
            // Parse replaces() call
            if let Some(value) = self.parse_list_func(line, "replaces") {
                pkg.replaces = value;
                continue;
            }
            
            // Parse backup() call
            if let Some(value) = self.parse_list_func(line, "backup") {
                pkg.backup = value;
                continue;
            }
            
            // Parse source() call
            if let Some(value) = self.parse_list_func(line, "source") {
                pkg.source = value;
                continue;
            }
            
            // Parse sha256sums() call
            if let Some(value) = self.parse_list_func(line, "sha256sums") {
                pkg.sha256sums = value;
                continue;
            }
            
            // Parse prepare() call
            if let Some(value) = self.parse_multiline_string(line, content, "prepare") {
                pkg.prepare = Some(value);
                continue;
            }
            
            // Parse build() call
            if let Some(value) = self.parse_multiline_string(line, content, "build") {
                pkg.build = Some(value);
                continue;
            }
            
            // Parse package() call
            if let Some(value) = self.parse_multiline_string(line, content, "package") {
                pkg.package = Some(value);
                continue;
            }
        }

        if pkg.name.is_empty() || pkg.version.is_empty() {
            return Err(SolarSuiteError::InvalidPackage(
                "Package must have name and version".to_string()
            ));
        }

        Ok(pkg)
    }

    /// Parse pkg("name", "version") call
    fn parse_pkg_call(&self, line: &str) -> Option<(String, String)> {
        if !line.starts_with("pkg(") {
            return None;
        }
        
        let inner = line.strip_prefix("pkg(")?.strip_suffix(')')?;
        let parts: Vec<&str> = inner.split(',').collect();
        
        if parts.len() >= 2 {
            let name = parts[0].trim().trim_matches('"').trim_matches('\'').to_string();
            let version = parts[1].trim().trim_matches('"').trim_matches('\'').to_string();
            Some((name, version))
        } else {
            None
        }
    }

    /// Parse function with single string argument: func("value")
    fn parse_string_func(&self, line: &str, func_name: &str) -> Option<String> {
        let prefix = format!("{}(", func_name);
        if !line.starts_with(&prefix) {
            return None;
        }
        
        let inner = line.strip_prefix(&prefix)?.strip_suffix(')')?;
        Some(inner.trim().trim_matches('"').trim_matches('\'').to_string())
    }

    /// Parse function with list argument: func(["a", "b", "c"])
    fn parse_list_func(&self, line: &str, func_name: &str) -> Option<Vec<String>> {
        let prefix = format!("{}(", func_name);
        if !line.starts_with(&prefix) {
            return None;
        }
        
        let inner = line.strip_prefix(&prefix)?.strip_suffix(')')?;
        let inner = inner.trim().trim_start_matches('[').trim_end_matches(']');
        
        let mut result = Vec::new();
        for item in inner.split(',') {
            let item = item.trim().trim_matches('"').trim_matches('\'');
            if !item.is_empty() {
                result.push(item.to_string());
            }
        }
        
        Some(result)
    }

    /// Parse multiline string function
    fn parse_multiline_string(&self, line: &str, _content: &str, func_name: &str) -> Option<String> {
        let prefix = format!("{}(", func_name);
        if !line.starts_with(&prefix) {
            return None;
        }
        
        // For now, just parse single-line strings
        let inner = line.strip_prefix(&prefix)?.strip_suffix(')')?;
        Some(inner.trim().trim_matches('"').trim_matches('\'').to_string())
    }
}

impl Default for SolarSuite {
    fn default() -> Self {
        Self::new().expect("Failed to create SolarSuite")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_package() {
        let suite = SolarSuite::new().unwrap();
        let content = r#"
pkg("testpkg", "1.0.0")
description("A test package")
license("MIT")
depends(["libc", "libgcc"])
"#;
        let result = suite.parse_content(content);
        assert!(result.is_ok());
        let pkg = result.unwrap();
        assert_eq!(pkg.name, "testpkg");
        assert_eq!(pkg.version, "1.0.0");
        assert_eq!(pkg.description, Some("A test package".to_string()));
        assert_eq!(pkg.license, Some("MIT".to_string()));
        assert_eq!(pkg.depends, vec!["libc", "libgcc"]);
    }
}
