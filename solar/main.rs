//! Solar Package Manager CLI

use clap::{Parser, Subcommand};

use solar::{Solar, SolarError};
use std::path::PathBuf;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[derive(Parser)]
#[command(name = "solar")]
#[command(author = "Solar Team")]
#[command(version = "0.1.0")]
#[command(about = "Solar Linux Package Manager", long_about = None)]
struct Cli {
    /// Root directory
    #[arg(short, long, global = true)]
    root: Option<PathBuf>,

    /// Verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// No color output
    #[arg(long, global = true)]
    no_color: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install a package
    Install {
        /// Package name(s)
        #[arg(required = true)]
        packages: Vec<String>,

        /// Install as dependency
        #[arg(short, long)]
        asdeps: bool,

        /// Don't install dependencies
        #[arg(short = 'D', long)]
        nodeps: bool,
    },

    /// Remove a package
    Remove {
        /// Package name(s)
        #[arg(required = true)]
        packages: Vec<String>,

        /// Remove dependencies
        #[arg(short, long)]
        deps: bool,

        /// Force removal
        #[arg(short, long)]
        force: bool,
    },

    /// Query packages
    Query {
        /// Package name(s)
        #[arg()]
        packages: Vec<String>,

        /// Query installed packages
        #[arg(short, long)]
        installed: bool,

        /// List files owned by package
        #[arg(short, long)]
        list: bool,

        /// Show package info
        #[arg(short, long)]
        info: bool,
    },

    /// Search for packages
    Search {
        /// Search pattern
        pattern: String,

        /// Search in descriptions too
        #[arg(short, long)]
        desc: bool,
    },

    /// Update package databases
    Update {
        /// Force refresh
        #[arg(short, long)]
        force: bool,
    },

    /// Upgrade packages
    Upgrade {
        /// Package name(s) to upgrade (all if not specified)
        packages: Vec<String>,
    },

    /// Build a package
    Build {
        /// Path to package definition (solar.lua)
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Install after building
        #[arg(short, long)]
        install: bool,

        /// Clean build directory after build
        #[arg(short, long)]
        clean: bool,
    },

    /// Show package info
    Info {
        /// Package name(s)
        #[arg(required = true)]
        packages: Vec<String>,
    },

    /// List installed packages
    List {
        /// Filter by pattern
        pattern: Option<String>,
    },

    /// Show dependencies
    Deps {
        /// Package name
        package: String,

        /// Show reverse dependencies
        #[arg(short, long)]
        reverse: bool,
    },

    /// Clean cache
    Clean {
        /// Clean build cache
        #[arg(short, long)]
        build: bool,

        /// Clean package cache
        #[arg(short, long)]
        pkgs: bool,

        /// Clean all caches
        #[arg(short, long)]
        all: bool,
    },

    /// Initialize solar
    Init {
        /// Create default configuration
        #[arg(short, long)]
        config: bool,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Initialize logging
    let filter = if cli.verbose {
        "debug"
    } else {
        "info"
    };

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::new(filter))
        .init();

    // Run command
    if let Err(e) = run(cli).await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

async fn run(cli: Cli) -> Result<(), SolarError> {
    match cli.command {
        Commands::Install { packages, asdeps: _, nodeps } => {
            let mut solar = Solar::new(cli.root)?;
            
            for pkg in packages {
                if nodeps {
                    // TODO: Install without dependencies
                    println!("Installing {} (no deps)...", pkg);
                } else {
                    solar.install(&pkg).await?;
                    println!("Installed: {}", pkg);
                }
            }
        }

        Commands::Remove { packages, deps: _, force: _ } => {
            let mut solar = Solar::new(cli.root)?;
            
            for pkg in packages {
                solar.remove(&pkg)?;
                println!("Removed: {}", pkg);
            }
        }

        Commands::Query { packages, installed, list, info } => {
            let solar = Solar::new(cli.root)?;
            
            if installed || packages.is_empty() {
                let pattern = packages.first().map(|s| s.as_str());
                let pkgs = solar.query(pattern)?;
                
                if list {
                    for pkg in pkgs {
                        println!("{}", pkg.name);
                    }
                } else if info {
                    for pkg in pkgs {
                        println!("Name        : {}", pkg.name);
                        println!("Version     : {}", pkg.version);
                        if let Some(desc) = &pkg.description {
                            println!("Description : {}", desc);
                        }
                        println!();
                    }
                } else {
                    for pkg in pkgs {
                        println!("{} {}", pkg.name, pkg.version);
                    }
                }
            }
        }

        Commands::Search { pattern, desc: _ } => {
            let solar = Solar::new(cli.root)?;
            let results = solar.search(&pattern)?;
            
            for pkg in results {
                println!("{} {}", pkg.name, pkg.version);
                if let Some(desc) = &pkg.description {
                    println!("    {}", desc);
                }
            }
        }

        Commands::Update { force: _ } => {
            let mut solar = Solar::new(cli.root)?;
            solar.update().await?;
            println!("Package databases updated.");
        }

        Commands::Upgrade { packages } => {
            let mut solar = Solar::new(cli.root)?;
            
            if packages.is_empty() {
                solar.upgrade().await?;
                println!("System upgraded.");
            } else {
                for pkg in packages {
                    solar.install(&pkg).await?;
                    println!("Upgraded: {}", pkg);
                }
            }
        }

        Commands::Build { path, install, clean } => {
            // TODO: Implement build command
            println!("Building package from: {:?}", path);
            if install {
                println!("Will install after build");
            }
            if clean {
                println!("Will clean after build");
            }
        }

        Commands::Info { packages } => {
            let solar = Solar::new(cli.root)?;
            
            for pkg_name in packages {
                // Check installed first
                if let Some(pkg) = solar.database().get(&pkg_name) {
                    println!("Name        : {}", pkg.name);
                    println!("Version     : {}", pkg.version);
                    if let Some(desc) = &pkg.description {
                        println!("Description : {}", desc);
                    }
                    println!("Installed   : Yes");
                    println!();
                } else {
                    // Search repos
                    let results = solar.search(&pkg_name)?;
                    if let Some(pkg) = results.first() {
                        println!("Name        : {}", pkg.name);
                        println!("Version     : {}", pkg.version);
                        if let Some(desc) = &pkg.description {
                            println!("Description : {}", desc);
                        }
                        println!("Installed   : No");
                        println!();
                    } else {
                        eprintln!("Package not found: {}", pkg_name);
                    }
                }
            }
        }

        Commands::List { pattern } => {
            let solar = Solar::new(cli.root)?;
            let pkgs = solar.query(pattern.as_deref())?;
            
            for pkg in pkgs {
                println!("{} {}", pkg.name, pkg.version);
            }
        }

        Commands::Deps { package, reverse } => {
            let solar = Solar::new(cli.root)?;
            
            if reverse {
                // Find packages that depend on this one
                println!("Reverse dependencies of {}:", package);
                // TODO: Implement reverse dependency lookup
            } else {
                if let Some(pkg) = solar.database().get(&package) {
                    println!("Dependencies of {}:", package);
                    for dep in &pkg.depends {
                        println!("  {}", dep);
                    }
                } else {
                    eprintln!("Package not installed: {}", package);
                }
            }
        }

        Commands::Clean { build, pkgs, all } => {
            let solar = Solar::new(cli.root)?;
            
            if all || (build && pkgs) {
                std::fs::remove_dir_all(solar.config().build_dir())?;
                std::fs::remove_dir_all(solar.config().cache_dir())?;
                println!("Cleaned all caches.");
            } else if build {
                std::fs::remove_dir_all(solar.config().build_dir())?;
                println!("Cleaned build cache.");
            } else if pkgs {
                std::fs::remove_dir_all(solar.config().cache_dir())?;
                println!("Cleaned package cache.");
            } else {
                println!("Nothing to clean. Use --build, --pkgs, or --all");
            }
        }

        Commands::Init { config } => {
            let solar = Solar::new(cli.root)?;
            
            if config {
                let config_path = solar.root().join("etc/solar/config.toml");
                solar.config().save(&config_path)?;
                println!("Configuration created at: {:?}", config_path);
            }
            
            println!("Solar initialized at: {:?}", solar.root());
        }
    }

    Ok(())
}
