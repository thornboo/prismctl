---
description: Interactively rollback Git branch to historical version; lists branches, versions, then executes reset/revert after confirmation
allowed-tools: Read(**), Exec(git fetch, git branch, git tag, git log, git reflog, git checkout, git reset, git revert, git switch), Write()
argument-hint: [--branch <branch>] [--target <rev>] [--mode reset|revert] [--depth <n>] [--dry-run] [--yes]
# examples:
#   - /prompts:git-rollback                # Full interactive mode, dry-run
#   - /prompts:git-rollback --branch dev   # Select dev directly, other interactive
#   - /prompts:git-rollback --branch dev --target v1.2.0 --mode reset --yes
---

# Codex Prompt: Git Rollback

**Purpose**: Safely and visually rollback a specified branch to an older version.
Defaults to **read-only preview (`--dry-run`)**; actual execution requires `--yes` or interactive confirmation.

---

## Usage

```bash
# Pure interactive: list branches → select branch → list recent 20 versions → select target → choose reset or revert → confirm
/prompts:git-rollback

# Specify branch, other interactive
/prompts:git-rollback --branch feature/calculator

# Specify branch and target commit, execute with hard-reset in one go (dangerous)
/prompts:git-rollback --branch main --target 1a2b3c4d --mode reset --yes

# Generate revert commit only (non-destructive rollback), preview
/prompts:git-rollback --branch release/v2.1 --target v2.0.5 --mode revert --dry-run
```

### Options

| Option                 | Description                                                                                                        |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------ |
| `--branch <branch>`    | Branch to rollback; interactively selected if omitted.                                                             |
| `--target <rev>`       | Target version (commit hash, tag, or reflog reference); interactively selects recent `--depth` entries if omitted. |
| `--mode reset\|revert` | `reset`: Hard rollback history; `revert`: Generate reverse commits keeping history intact. Prompts by default.     |
| `--depth <n>`          | List recent n versions in interactive mode (default 20).                                                           |
| `--dry-run`            | **Enabled by default**, only preview commands to be executed.                                                      |
| `--yes`                | Skip all confirmations and execute directly, suitable for CI/CD scripts.                                           |

---

## Interactive Flow

1. **Sync remote** → `git fetch --all --prune`
2. **List branches** → `git branch -a` (local + remote, filter protected branches)
3. **Select branch** → User input or parameter
4. **List versions** → `git log --oneline -n <depth>` + `git tag --merged` + `git reflog -n <depth>`
5. **Select target** → User inputs commit hash / tag
6. **Select mode** → `reset` or `revert`
7. **Final confirmation** (unless `--yes`)
8. **Execute rollback**
   - `reset`: `git switch <branch> && git reset --hard <target>`
   - `revert`: `git switch <branch> && git revert --no-edit <target>..HEAD`
9. **Push suggestion** → Prompt whether to `git push --force-with-lease` (reset) or regular `git push` (revert)

---

## Safety Guards

- **Backup**: Automatically records current HEAD in reflog before execution, recoverable with `git switch -c backup/<timestamp>`.
- **Protected branches**: If protected branches like `main` / `master` / `production` are detected with `reset` mode enabled, requires additional confirmation.
- **--dry-run enabled by default**: Prevents accidental operations.
- **--force prohibited**: No `--force` provided; if force push needed, manually enter `git push --force-with-lease`.

---

## Use Case Examples

| Scenario                                                                    | Command Example                                                  |
| --------------------------------------------------------------------------- | ---------------------------------------------------------------- |
| Hotfix patch deployed with bug, need to rollback to tag `v1.2.0`            | `/prompts:git-rollback --branch release/v1 --target v1.2.0 --mode reset` |
| Ops colleague pushed debug logs by mistake, need to generate reverse commit | `/prompts:git-rollback --branch main --target 3f2e7c9 --mode revert`     |
| Research historical bugs, guide newcomers through branch history            | `/prompts:git-rollback` (full interactive, dry-run)                      |

---

## Notes

1. **reset vs revert**
   - **reset** changes history, requires force push and may affect other collaborators, use with caution.
   - **revert** is safer, generates new commits preserving history, but adds one more record.
2. **Embedded repositories** often have large binary files; ensure LFS/submodule state consistency before rollback.
3. If repository has CI forced validation, rollback may trigger pipelines automatically; confirm control policies to avoid accidental deployment of old versions.

---
