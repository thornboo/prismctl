# Installation


## From crates.io (recommended)

Prerequisite: Rust toolchain installed (rustup recommended).

```bash
cargo install prismctl
```

Upgrade:

```bash
cargo install prismctl --force
```

## Shell install (GitHub Releases)

Prerequisites: `curl` (or `wget`), `tar`.

Install the latest release to `~/.local/bin`:

```bash
curl -fsSL "https://raw.githubusercontent.com/thornboo/prismctl/HEAD/install.sh" | sh
```

Install a specific version:

```bash
PRISMCTL_VERSION="v0.1.0" curl -fsSL "https://raw.githubusercontent.com/thornboo/prismctl/HEAD/install.sh" | sh
```

Custom install dir:

```bash
PRISMCTL_INSTALL_DIR="$HOME/bin" curl -fsSL "https://raw.githubusercontent.com/thornboo/prismctl/HEAD/install.sh" | sh
```

## From source (development)

```bash
git clone "https://github.com/thornboo/prismctl.git"
cd "prismctl"
cargo install --path "crates/prismctl-cli"
```

## Verify

```bash
prismctl doctor
```

## Uninstall (optional)

```bash
cargo uninstall prismctl
```
