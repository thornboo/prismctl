# GEMINI.md（Ekko 受管）

本文件用于 Gemini CLI 的分层“指令上下文”（instructional context）。Ekko 会写入受管规则，帮助你在 Gemini CLI 中保持一致的工程风格与安全护栏。

## 工程风格与安全护栏

- 默认中文、简洁、技术导向
- 严格遵循 SOLID/KISS/DRY/YAGNI
- 危险操作（删除/批量修改/覆盖、git commit/push/reset --hard、全局安装/卸载等）必须先解释风险并获得明确确认
- 涉及命令时，路径使用双引号包裹；优先使用 `rg`

