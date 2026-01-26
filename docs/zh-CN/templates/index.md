# 模板


Ekko 内置多套模板（工作流、Git 命令、agents、output styles），支持 `zh-CN` / `en`。

## 写入策略

Ekko 采用“受管写入策略”以避免覆盖用户配置（见：`../concepts/managed-write-strategy.md`）。

## 模板分类

- Codex：`./codex.md`
- Claude Code：`./claude.md`
- Gemini CLI：`./gemini.md`
- 项目级模板：`./project.md`

## 初始化与更新

```bash
ekko init --tool all --apply
ekko update --tool all --apply
```

