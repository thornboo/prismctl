# `prismctl project init`


为项目创建 Prismctl 约定的工作流目录，并初始化 Gemini CLI 的项目级记忆文件。

```bash
prismctl project init [--path <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply]
```

示例：

```bash
prismctl project init --path "/path/to/your/project" --apply
```

它会创建/更新：

- `<project>/.prismctl/plan/`：工作流落盘目录（`current/` 与 `history/`）
- `<project>/.gemini/GEMINI.md`：项目级记忆（受管块写入，保留块外内容）

详见：`../projects/project-init.md`。
