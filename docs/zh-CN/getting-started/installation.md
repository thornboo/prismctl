# 安装


## 通过 crates.io（推荐）

前置要求：已安装 Rust 工具链（推荐使用 rustup）。

```bash
cargo install ekko
```

升级：

```bash
cargo install ekko --force
```

## 从源码安装（开发/本地修改）

```bash
git clone "https://github.com/thornboo/ekko.git"
cd "ekko"
cargo install --path "crates/ekko-cli"
```

## 验证安装

```bash
ekko doctor
```

## 卸载（可选）

```bash
cargo uninstall ekko
```
