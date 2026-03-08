//! Starlark dialect for Solar package definitions
//!
//! This module provides a simple parser for Starlark-like package definitions.
//! The full starlark-rust integration can be added later.

use crate::package::PackageDef;

/// Context for building a package definition
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
}

/// Simple Starlark parser for Solar package definitions
pub struct StarlarkParser;

impl StarlarkParser {
    /// Parse Starlark content, returning the package definition
    pub fn parse(content: &str) -> Result<PackageDef, String> {
        let mut pkg = PackageDef::default();

        // Simple line-by-line parsing for Starlark-like syntax
        for line in content.lines() {
            let line = line.trim();

            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Parse pkg("name", "version") call
            if let Some((name, version)) = Self::parse_pkg_call(line) {
                pkg.name = name;
                pkg.version = version;
                continue;
            }

            // Parse description() call
            if let Some(value) = Self::parse_string_func(line, "description") {
                pkg.description = Some(value);
                continue;
            }

            // Parse homepage() call
            if let Some(value) = Self::parse_string_func(line, "homepage") {
                pkg.homepage = Some(value);
                continue;
            }

            // Parse license() call
            if let Some(value) = Self::parse_string_func(line, "license") {
                pkg.license = Some(value);
                continue;
            }

            // Parse arch() call
            if let Some(value) = Self::parse_list_func(line, "arch") {
                pkg.arch = value;
                continue;
            }

            // Parse depends() call
            if let Some(value) = Self::parse_list_func(line, "depends") {
                pkg.depends = value;
                continue;
            }

            // Parse optdepends() call
            if let Some(value) = Self::parse_list_func(line, "optdepends") {
                pkg.optdepends = value;
                continue;
            }

            // Parse conflicts() call
            if let Some(value) = Self::parse_list_func(line, "conflicts") {
                pkg.conflicts = value;
                continue;
            }

            // Parse provides() call
            if let Some(value) = Self::parse_list_func(line, "provides") {
                pkg.provides = value;
                continue;
            }

            // Parse replaces() call
            if let Some(value) = Self::parse_list_func(line, "replaces") {
                pkg.replaces = value;
                continue;
            }

            // Parse backup() call
            if let Some(value) = Self::parse_list_func(line, "backup") {
                pkg.backup = value;
                continue;
            }

            // Parse source() call
            if let Some(value) = Self::parse_list_func(line, "source") {
                pkg.source = value;
                continue;
            }

            // Parse sha256sums() call
            if let Some(value) = Self::parse_list_func(line, "sha256sums") {
                pkg.sha256sums = value;
                continue;
            }

            // Parse prepare = "..." or prepare = """..."""
            if let Some(value) = Self::parse_assignment(line, content, "prepare") {
                pkg.prepare = Some(value);
                continue;
            }

            // Parse build = "..." or build = """..."""
            if let Some(value) = Self::parse_assignment(line, content, "build") {
                pkg.build = Some(value);
                continue;
            }

            // Parse package = "..." or package = """..."""
            if let Some(value) = Self::parse_assignment(line, content, "package") {
                pkg.package = Some(value);
                continue;
            }
        }

        if pkg.name.is_empty() || pkg.version.is_empty() {
            return Err("Package must have name and version".to_string());
        }

        Ok(pkg)
    }

    /// Parse pkg("name", "version") call
    fn parse_pkg_call(line: &str) -> Option<(String, String)> {
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
    fn parse_string_func(line: &str, func_name: &str) -> Option<String> {
        let prefix = format!("{}(", func_name);
        if !line.starts_with(&prefix) {
            return None;
        }

        let inner = line.strip_prefix(&prefix)?.strip_suffix(')')?;
        Some(inner.trim().trim_matches('"').trim_matches('\'').to_string())
    }

    /// Parse function with list argument: func(["a", "b", "c"])
    fn parse_list_func(line: &str, func_name: &str) -> Option<Vec<String>> {
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

    /// Parse multiline string assignment: func = "..." or func = """..."""
    fn parse_assignment(line: &str, content: &str, func_name: &str) -> Option<String> {
        let prefix = format!("{} =", func_name);
        if !line.starts_with(&prefix) {
            return None;
        }

        let value_part = line.strip_prefix(&prefix)?.trim();

        // Handle triple-quoted strings (multiline)
        if value_part.starts_with("\"\"\"") {
            // Check if it's a single-line triple-quoted string
            if let Some(rest) = value_part.strip_prefix("\"\"\"") {
                if let Some(end_idx) = rest.find("\"\"\"") {
                    return Some(rest[..end_idx].to_string());
                }
                // Multiline: find the closing """ in the content
                let line_start = content.find(line)?;
                let search_start = line_start + line.len();
                if let Some(rest_content) = content.get(search_start..) {
                    if let Some(end_idx) = rest_content.find("\"\"\"") {
                        let script = rest_content[..end_idx].trim().to_string();
                        // Remove leading/trailing newlines and common indentation
                        return Some(script.lines()
                            .skip_while(|l| l.trim().is_empty())
                            .map(|l| l.trim_start())
                            .collect::<Vec<_>>()
                            .join("\n"));
                    }
                }
            }
        }

        // Handle single-quoted strings
        if value_part.starts_with('"') && value_part.ends_with('"') {
            return value_part.strip_prefix('"')?.strip_suffix('"').map(|s| s.to_string());
        }

        if value_part.starts_with('\'') && value_part.ends_with('\'') {
            return value_part.strip_prefix('\'')?.strip_suffix('\'').map(|s| s.to_string());
        }

        None
    }
}
