# Installation


## From crates.io (recommended)

Prerequisite: Rust toolchain installed (rustup recommended).

```bash
cargo install ekko
```

Upgrade:

```bash
cargo install ekko --force
```

## Shell install (GitHub Releases)

Prerequisites: `curl` (or `wget`), `tar`.

Install the latest release to `~/.local/bin`:

```bash
curl -fsSL "https://raw.githubusercontent.com/thornboo/ekko/HEAD/install.sh" | sh
```

Install a specific version:

```bash
EKKO_VERSION="v0.1.0" curl -fsSL "https://raw.githubusercontent.com/thornboo/ekko/HEAD/install.sh" | sh
```

Custom install dir:

```bash
EKKO_INSTALL_DIR="$HOME/bin" curl -fsSL "https://raw.githubusercontent.com/thornboo/ekko/HEAD/install.sh" | sh
```

## From source (development)

```bash
git clone "https://github.com/thornboo/ekko.git"
cd "ekko"
cargo install --path "crates/ekko-cli"
```

## Verify

```bash
ekko doctor
```

## Uninstall (optional)

```bash
cargo uninstall ekko
```
