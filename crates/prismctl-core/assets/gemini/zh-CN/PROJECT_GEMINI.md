# 项目级 GEMINI.md（Prismctl 受管）

本段内容由 Prismctl 维护，用于**项目级**的指令上下文。Gemini CLI 会从项目根目录向上/向下扫描并加载 `GEMINI.md`（以配置为准）。

## 工程风格与安全护栏（项目级）

- 默认中文、简洁、技术导向
- 严格遵循 SOLID/KISS/DRY/YAGNI
- 危险操作必须先解释风险并获得明确确认（删除/覆盖、git commit/push/reset --hard、全局安装/卸载等）
- 命令中路径使用双引号包裹；优先使用 `rg` 搜索

## 项目上下文建议（请你补充）

- 项目目标/范围
- 技术栈与版本
- 目录结构与模块边界
- 代码规范（lint/format/testing）
- 发布/部署方式

