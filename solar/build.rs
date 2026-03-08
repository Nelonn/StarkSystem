//! Package building functionality

use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug)]
enum BuildError {
    Io(std::io::Error),
    ScriptFailed { code: Option<i32>, stderr: String },
    Json(serde_json::Error),
}

impl From<std::io::Error> for BuildError {
    fn from(e: std::io::Error) -> Self {
        BuildError::Io(e)
    }
}

impl From<serde_json::Error> for BuildError {
    fn from(e: serde_json::Error) -> Self {
        BuildError::Json(e)
    }
}

type Result<T> = std::result::Result<T, BuildError>;

/// Package definition for build script
#[derive(serde::Serialize, serde::Deserialize)]
struct Package {
    name: String,
    version: String,
    description: Option<String>,
    prepare: Option<String>,
    build: Option<String>,
    package: Option<String>,
    depends: Vec<String>,
}

impl Package {
    fn filename(&self) -> String {
        format!("{}-{}-{}.pkg.tar.gz", self.name, self.version, std::env::consts::ARCH)
    }
}

/// Package builder
pub struct Builder {
    build_dir: PathBuf,
    work_dir: PathBuf,
    pkg_dir: PathBuf,
    arch: String,
    jobs: usize,
}

impl Builder {
    /// Create a new builder
    pub fn new(build_dir: &Path, arch: &str, jobs: usize) -> Result<Self> {
        let work_dir = build_dir.join("work");
        let pkg_dir = build_dir.join("pkg");

        std::fs::create_dir_all(&work_dir)?;
        std::fs::create_dir_all(&pkg_dir)?;

        Ok(Self {
            build_dir: build_dir.to_path_buf(),
            work_dir,
            pkg_dir,
            arch: arch.to_string(),
            jobs,
        })
    }

    /// Build a package
    pub fn build(&self, pkg: &Package) -> Result<PathBuf> {
        let pkg_build_dir = self.work_dir.join(&pkg.name);
        std::fs::create_dir_all(&pkg_build_dir)?;

        // Run prepare script if present
        if let Some(prepare) = &pkg.prepare {
            self.run_script(prepare, &pkg_build_dir)?;
        }

        // Run build script if present
        if let Some(build) = &pkg.build {
            self.run_script(build, &pkg_build_dir)?;
        }

        // Run package script if present
        if let Some(package) = &pkg.package {
            let install_dir = self.pkg_dir.join(&pkg.name);
            std::fs::create_dir_all(&install_dir)?;
            self.run_script(package, &install_dir)?;
        }

        // Create the package archive
        self.create_package(pkg)
    }

    /// Run a build script
    fn run_script(&self, script: &str, dir: &Path) -> Result<()> {
        let mut cmd = Command::new("bash");
        cmd.arg("-c")
            .arg(script)
            .current_dir(dir)
            .env("MAKEFLAGS", format!("-j{}", self.jobs))
            .env("ARCH", &self.arch);

        let output = cmd.output()
            .map_err(BuildError::Io)?;

        if !output.status.success() {
            return Err(BuildError::ScriptFailed {
                code: output.status.code(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }

        Ok(())
    }

    /// Create the package archive
    fn create_package(&self, pkg: &Package) -> Result<PathBuf> {
        let pkg_path = self.build_dir.join(pkg.filename());

        // Create tar archive
        let file = std::fs::File::create(&pkg_path)?;
        let encoder = flate2::write::GzEncoder::new(file, flate2::Compression::default());
        let mut archive = tar::Builder::new(encoder);

        // Add package metadata
        let metadata = serde_json::to_string_pretty(pkg)?;

        let mut header = tar::Header::new_gnu();
        header.set_path(".PKGINFO").unwrap();
        header.set_size(metadata.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        archive.append(&header, metadata.as_bytes())?;

        // Add installed files
        let install_dir = self.pkg_dir.join(&pkg.name);
        if install_dir.exists() {
            archive.append_dir_all(".", &install_dir)?;
        }

        archive.finish()?;

        Ok(pkg_path)
    }

    /// Clean build directories
    pub fn clean(&self) -> Result<()> {
        if self.work_dir.exists() {
            std::fs::remove_dir_all(&self.work_dir)?;
        }
        if self.pkg_dir.exists() {
            std::fs::remove_dir_all(&self.pkg_dir)?;
        }
        std::fs::create_dir_all(&self.work_dir)?;
        std::fs::create_dir_all(&self.pkg_dir)?;
        Ok(())
    }
}

fn main() {
    // Build script entry point - can be used for custom build logic
    println!("cargo:rerun-if-changed=build.rs");
}
