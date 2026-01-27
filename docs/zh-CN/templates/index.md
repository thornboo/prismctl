# 模板


Ekko 内置多套模板（工作流、Git 命令、agents、output styles），支持 `zh-CN` / `en`。

## 写入策略

Ekko 采用“受管写入策略”以避免覆盖用户配置（见：`../concepts/managed-write-strategy.md`）。

一句话原则：

- Ekko 会完全管理 `ekko/` 命名空间内的模板文件（可覆盖更新）
- 对少数“共享文件”只更新受管块（保留块外内容）
- 对不可安全合并的文件要求 `--yes` 并自动备份

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

## 我应该在哪里改模板？

建议把 Ekko 模板当作“上游发行版”来用：

- 想跟随 Ekko 升级：不要直接改 `ekko/` 命名空间文件
- 想定制：复制一份到你自己的目录/命名空间，再在工具侧引用你的版本
