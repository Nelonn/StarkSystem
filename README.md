# Solar Linux Package Manager

A modern Linux package manager written in Rust, using Starlark (a Python dialect) for package definitions.

## Features

- **Starlark-based package definitions**: Package build scripts use Starlark syntax, a Python-like dialect that is simple and secure
- **Dependency resolution**: Automatic dependency installation and conflict detection
- **Repository support**: Multiple repository support with syncing capabilities
- **Build system**: Integrated build system for compiling packages from source
- **Database tracking**: SQLite-like database for tracking installed packages and files

## Installation

```bash
cargo build --release
```

## Usage

### Initialize Solar

```bash
solar init --config
```

### Install a Package

```bash
solar install <package-name>
```

### Remove a Package

```bash
solar remove <package-name>
```

### Search for Packages

```bash
solar search <pattern>
```

### Update Package Databases

```bash
solar update
```

### Upgrade Installed Packages

```bash
solar upgrade
```

### Build a Package

```bash
cd packages/bash
solar build
```

### List Installed Packages

```bash
solar list
```

### Query Package Info

```bash
solar info <package-name>
```

## Package Definition Format

Packages are defined using `solar.lua` files with Starlark syntax:

```python
pkg("bash", "5.2.15")
description("The GNU Bourne Again Shell")
homepage("https://www.gnu.org/software/bash/")
license("GPL-3.0-or-later")
arch(["x86_64", "aarch64"])
depends(["glibc", "ncurses", "readline"])
source(["https://ftp.gnu.org/gnu/bash/bash-5.2.15.tar.gz"])
sha256sums(["abc123..."])

prepare("""
    # Prepare build environment
    ./configure --prefix=/usr
""")

build("""
    make
""")

package("""
    make install
""")
```

## Project Structure

```
StarkSystem/
├── Cargo.toml          # Main project configuration
├── solarsuite/         # Starlark interpreter for package definitions
│   ├── Cargo.toml
│   └── src/lib.rs
├── src/
│   ├── main.rs         # CLI entry point
│   ├── lib.rs          # Core library
│   ├── config.rs       # Configuration management
│   ├── database.rs     # Package database
│   ├── package.rs      # Package definitions
│   ├── repo.rs         # Repository handling
│   ├── build.rs        # Build system
│   └── error.rs        # Error types
└── packages/           # Package definitions
    ├── bash/
    │   └── solar.lua
    └── ...
```

## Architecture

- **solar**: Main package manager binary and library
- **solarsuite**: Starlark-based package definition interpreter (uses starlark-rust)

## Dependencies

- `clap` - CLI argument parsing
- `tokio` - Async runtime
- `reqwest` - HTTP client (with rustls for TLS)
- `serde` - Serialization
- `toml` - Configuration files
- `tar`, `flate2` - Archive handling
- `sha2` - Checksum verification
- `tracing` - Logging

## License

MIT
