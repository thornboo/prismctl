# Gemini CLI 模板


## 写入位置

```
~/.gemini/
├── .env                # Gemini CLI 环境变量
├── GEMINI.md           # 全局记忆（受管块写入）
└── ekko/
    └── WORKFLOWS.md    # 说明性文档（命名空间文件）
```

## 受管块（GEMINI.md）

Ekko 只更新标记块内的内容，块外内容完全保留：

```markdown
<!-- ekko:start -->
Ekko 管理的内容
<!-- ekko:end -->
```

## 记忆加载层级（Gemini CLI）

Gemini CLI 会按目录层级加载 `GEMINI.md` 作为上下文：

1. 全局：`~/.gemini/GEMINI.md`
2. 项目：`<project>/.gemini/GEMINI.md`
3. 子目录：`<subdir>/.gemini/GEMINI.md`
