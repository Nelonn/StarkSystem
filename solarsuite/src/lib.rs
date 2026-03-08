//! SolarSuite - Starlark-based package definition interpreter
//!
//! This crate provides the core functionality for interpreting package definitions
//! written in Starlark (a Python-like dialect) for the Solar package manager.
//!
//! # Example
//!
//! ```rust,no_run
//! use solarsuite::{SolarSuite, PackageDef};
//! use std::path::Path;
//!
//! let suite = SolarSuite::new().unwrap();
//! let pkg = suite.parse_file(Path::new("package.bazon")).unwrap();
//! println!("Package: {} v{}", pkg.name, pkg.version);
//! ```

pub mod package;
pub mod starlark_dialect;
pub mod interpreter;

pub use package::PackageDef;
pub use interpreter::{SolarSuite, SolarSuiteError};
