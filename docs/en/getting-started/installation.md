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
