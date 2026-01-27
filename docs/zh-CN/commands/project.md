# `ekko project init`


为项目创建 Ekko 约定的工作流目录，并初始化 Gemini CLI 的项目级记忆文件。

```bash
ekko project init [--path <PATH>] [--lang <zh-CN|en>] [--dry-run|--apply]
```

示例：

```bash
ekko project init --path "/path/to/your/project" --apply
```

它会创建/更新：

- `<project>/.ekko/plan/`：工作流落盘目录（`current/` 与 `history/`）
- `<project>/.gemini/GEMINI.md`：项目级记忆（受管块写入，保留块外内容）

详见：`../projects/project-init.md`。
