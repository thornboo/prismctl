# 项目初始化


`ekko project init` 用于为项目创建 Ekko 约定的工作流目录，并初始化 Gemini CLI 的项目级记忆文件。

## 快速使用

```bash
# 在项目根目录执行（dry-run 预览）
cd "/path/to/your/project"
ekko project init

# 实际写入
ekko project init --apply

# 指定其他目录
ekko project init --path "/path/to/other/project" --apply

# 使用英文模板
ekko project init --lang en --apply
```

## 创建内容

### 目录结构

```
<project>/
├── .ekko/
│   └── plan/
│       ├── current/        # 进行中的任务计划
│       ├── history/        # 已完成任务归档
│       └── README.md       # 目录约定说明
└── .gemini/
    └── GEMINI.md           # 项目级记忆（受管块）
```

### .ekko/plan/ 目录

| 子目录/文件 | 用途 |
|------------|------|
| `current/` | 存放进行中的任务计划（如 `current/add-auth.md`） |
| `history/` | 已完成任务的归档（如 `history/2026-01-26-add-auth.md`） |
| `README.md` | 目录约定说明，供团队成员参考 |

### .gemini/GEMINI.md

Gemini CLI 的项目级记忆文件，采用受管块写入：

```markdown
<!-- ekko:start -->
# Ekko 项目上下文

## 工作流目录
- `.ekko/plan/current/` - 进行中的任务
- `.ekko/plan/history/` - 已完成任务归档

## 使用说明
...
<!-- ekko:end -->
```

如果文件已存在，Ekko 只会更新标记块内的内容，保留其他用户自定义内容。

## 为什么需要项目初始化

### 1. 统一工作流落盘位置

Ekko 内置的 Codex/Claude 工作流模板会引用 `.ekko/plan/*` 作为计划落盘位置。

### 2. 团队共享上下文

`.gemini/GEMINI.md` 可以提交到版本控制，以共享项目上下文与约定。

### 3. 任务追溯

`history/` 目录保留历史任务计划，便于回顾决策过程与复用方案。

## Gemini CLI 记忆层级

Gemini CLI 会分层加载 `GEMINI.md`：

```
优先级（低到高）：
1. ~/.gemini/GEMINI.md           # 全局记忆
2. <project>/.gemini/GEMINI.md   # 项目级记忆 ← ekko project init 创建
3. <subdir>/.gemini/GEMINI.md    # 子目录记忆
```

使用 `/memory refresh` 命令可重新扫描并加载所有层级的记忆文件。

## 与版本控制集成（建议）

- 推荐提交：`.ekko/plan/README.md`、`.ekko/plan/history/`、`.gemini/GEMINI.md`
- `current/` 是否提交取决于团队协作习惯

## 下一步

- 模板：`../templates/index.md`
- 命令参考：`../commands/index.md`

