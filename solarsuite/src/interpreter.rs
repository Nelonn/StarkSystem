//! Solar Starlark Interpreter
//!
//! This module provides the main interpreter for evaluating Starlark-based
//! package definitions using the starlark-rust parser.

use crate::package::PackageDef;
use crate::starlark_dialect::StarlarkParser;
use std::path::Path;
use thiserror::Error;

/// Errors that can occur during Starlark evaluation
#[derive(Error, Debug)]
pub enum SolarSuiteError {
    #[error("Failed to parse package file: {0}")]
    ParseError(String),
    #[error("Invalid package definition: {0}")]
    InvalidPackage(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Solar Starlark Interpreter
pub struct SolarSuite;

impl SolarSuite {
    /// Create a new Solar Starlark interpreter
    pub fn new() -> Result<Self, SolarSuiteError> {
        Ok(Self)
    }

    /// Parse and evaluate a Starlark package file
    pub fn parse_file(&self, path: &Path) -> Result<PackageDef, SolarSuiteError> {
        let content = std::fs::read_to_string(path)?;
        self.parse_content(&content, path.to_str().unwrap_or("unknown"))
    }

    /// Parse and evaluate Starlark content
    pub fn parse_content(&self, content: &str, filepath: &str) -> Result<PackageDef, SolarSuiteError> {
        // Parse using starlark-rust based parser
        let ctx = StarlarkParser::parse(content, filepath)
            .map_err(|e| SolarSuiteError::ParseError(e))?;

        let pkg = ctx.into_package();

        // Validate the package
        if pkg.name.is_empty() || pkg.version.is_empty() {
            return Err(SolarSuiteError::InvalidPackage(
                "Package must have name and version".to_string()
            ));
        }

        Ok(pkg)
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
    fn test_create_interpreter() {
        let suite = SolarSuite::new();
        assert!(suite.is_ok());
    }

    #[test]
    fn test_parse_simple_package() {
        let suite = SolarSuite::new().unwrap();
        let content = r#"
pkg("testpkg", "1.0.0")
description("A test package")
license("MIT")
depends(["libc", "libgcc"])
"#;
        let result = suite.parse_content(content, "test.bazon");
        assert!(result.is_ok());
        let pkg = result.unwrap();
        assert_eq!(pkg.name, "testpkg");
        assert_eq!(pkg.version, "1.0.0");
        assert_eq!(pkg.description, Some("A test package".to_string()));
        assert_eq!(pkg.license, Some("MIT".to_string()));
        assert_eq!(pkg.depends, vec!["libc", "libgcc"]);
    }

    #[test]
    fn test_parse_bash_package() {
        let suite = SolarSuite::new().unwrap();
        let content = r#"
pkg("bash", "5.2.15")
description("The GNU Bourne Again Shell")
homepage("https://www.gnu.org/software/bash/")
license("GPL-3.0-or-later")
arch(["x86_64", "aarch64", "i686"])
depends(["glibc", "ncurses", "readline"])
optdepends(["bash-completion: for tab completion"])
provides(["sh"])
source(["https://ftp.gnu.org/gnu/bash/bash-5.2.15.tar.gz"])
sha256sums(["83e2164d0e79304f6a5600dbb60ba4fc0c75a9e1450a94aa5f0d24be0e680494"])

prepare = """
./configure --prefix=/usr
"""

build = """
make
"""

package = """
make install
"""
"#;
        let result = suite.parse_content(content, "bash.bazon");
        assert!(result.is_ok());
        let pkg = result.unwrap();
        assert_eq!(pkg.name, "bash");
        assert_eq!(pkg.version, "5.2.15");
        assert!(pkg.description.is_some());
        assert_eq!(pkg.depends.len(), 3);
        assert!(pkg.prepare.is_some());
        assert!(pkg.build.is_some());
        assert!(pkg.package.is_some());
    }

    #[test]
    fn test_parse_package_with_variable_syntax() {
        let suite = SolarSuite::new().unwrap();
        let content = r#"
pkg("testpkg", "2.0.0")
description = "A test package with variable syntax"
homepage = "https://example.com"
license = "Apache-2.0"
depends(["libc"])

build = """
make build
"""
"#;
        let result = suite.parse_content(content, "test.bazon");
        assert!(result.is_ok());
        let pkg = result.unwrap();
        assert_eq!(pkg.name, "testpkg");
        assert_eq!(pkg.version, "2.0.0");
        assert_eq!(pkg.description, Some("A test package with variable syntax".to_string()));
        assert_eq!(pkg.homepage, Some("https://example.com".to_string()));
        assert_eq!(pkg.license, Some("Apache-2.0".to_string()));
        assert_eq!(pkg.depends, vec!["libc"]);
        assert!(pkg.build.is_some());
    }
}
