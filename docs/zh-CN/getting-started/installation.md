# 安装


## 通过 crates.io（推荐）

前置要求：已安装 Rust 工具链（推荐使用 rustup）。

```bash
cargo install prismctl
```

升级：

```bash
cargo install prismctl --force
```

## Shell 安装（GitHub Releases）

前置要求：`curl`（或 `wget`）、`tar`。

安装最新版本到 `~/.local/bin`：

```bash
curl -fsSL "https://raw.githubusercontent.com/thornboo/prismctl/HEAD/install.sh" | sh
```

安装指定版本：

```bash
PRISMCTL_VERSION="v0.1.0" curl -fsSL "https://raw.githubusercontent.com/thornboo/prismctl/HEAD/install.sh" | sh
```

自定义安装目录：

```bash
PRISMCTL_INSTALL_DIR="$HOME/bin" curl -fsSL "https://raw.githubusercontent.com/thornboo/prismctl/HEAD/install.sh" | sh
```

## 从源码安装（开发/本地修改）

```bash
git clone "https://github.com/thornboo/prismctl.git"
cd "prismctl"
cargo install --path "crates/prismctl-cli"
```

## 验证安装

```bash
prismctl doctor
```

## 卸载（可选）

```bash
cargo uninstall prismctl
```
