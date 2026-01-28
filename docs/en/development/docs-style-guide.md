# Documentation Style Guide


This page defines minimal documentation conventions for Prismctl to keep docs discoverable and bilingual pages consistent.

## Structure and locales

- Docs are organized by locale directories: `docs/zh-CN/` and `docs/en/`
- Keep both trees mirrored (same topics at the same relative paths)
- Shared assets live in `docs/_shared/` (e.g. images)
- mdBook UI assets (e.g. language switcher) live per locale under `docs/<locale>/_shared/mdbook/`

## Naming

- Use `kebab-case.md` inside `docs/`
- Each locale entry file is `README.md`
- Keep repo root English-first: `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `LICENSE`

## Cross-links

- Do not add per-page locale switch lines. Keep language selection at the docs entry points.

## Suggested page shape

Try to include:

1. Goal / scope
2. Copy-pastable examples
3. Key concepts and safety boundaries
4. FAQ / gotchas (optional)
