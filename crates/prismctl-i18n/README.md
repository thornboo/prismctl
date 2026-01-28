# prismctl-i18n

Internationalization support for Prismctl.

## Usage

```rust
use prismctl_i18n::{keys, t, tf};

println!("{}", t!(keys::MENU_TITLE));
println!("{}", tf!(keys::ERROR_INVALID_CHOICE, "choice" => "abc"));
```

## Adding translations

1. Add key-value pairs to `i18n/zh-CN/*.ftl` and `i18n/en/*.ftl`
2. Rebuild to regenerate type-safe keys via `build.rs`
3. Use `keys::YOUR_NEW_KEY` in code

## Changing language

```bash
PRISMCTL_LANG=en cargo run -p prismctl --
PRISMCTL_LANG=zh-CN cargo run -p prismctl --
```
