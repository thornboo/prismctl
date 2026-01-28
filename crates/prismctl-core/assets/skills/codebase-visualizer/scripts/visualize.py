#!/usr/bin/env python3
import argparse
import html
import os
import sys
import webbrowser
from pathlib import Path


DEFAULT_IGNORES = {
    ".git",
    "node_modules",
    "target",
    ".next",
    ".turbo",
    ".venv",
    "__pycache__",
}


def iter_dir(path: Path, ignores: set[str]) -> list[Path]:
    try:
        entries = list(path.iterdir())
    except OSError:
        return []
    entries = [p for p in entries if p.name not in ignores]
    entries.sort(key=lambda p: (p.is_file(), p.name.lower()))
    return entries


def render_node(path: Path, root: Path, ignores: set[str]) -> str:
    rel = path.relative_to(root)
    label = html.escape(str(rel))
    if path.is_dir():
        children = iter_dir(path, ignores)
        if not children:
            return f"<li class='dir empty'>{label}/</li>"
        rendered = "\n".join(render_node(c, root, ignores) for c in children)
        return (
            "<li class='dir'>"
            f"<details open><summary>{label}/</summary>"
            f"<ul>{rendered}</ul>"
            "</details>"
            "</li>"
        )
    return f"<li class='file'>{label}</li>"


def build_html(root: Path, ignores: set[str]) -> str:
    root_name = html.escape(root.resolve().as_posix())
    top = "\n".join(render_node(p, root, ignores) for p in iter_dir(root, ignores))
    return f"""<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>Codebase Map</title>
  <style>
    body {{ font-family: ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace; margin: 24px; }}
    h1 {{ font-size: 18px; margin: 0 0 16px; }}
    ul {{ list-style: none; padding-left: 18px; margin: 0; }}
    li {{ margin: 2px 0; }}
    summary {{ cursor: pointer; }}
    .file {{ color: #1f2937; }}
    .dir > details > summary {{ color: #111827; }}
    .empty {{ color: #6b7280; }}
  </style>
</head>
<body>
  <h1>Codebase Map: {root_name}</h1>
  <ul>{top}</ul>
</body>
</html>
"""


def main() -> int:
    parser = argparse.ArgumentParser(description="Generate an interactive HTML tree visualization of a codebase.")
    parser.add_argument("path", nargs="?", default=".", help="Project root path (default: .)")
    parser.add_argument("-o", "--output", default="codebase-map.html", help="Output HTML file (default: codebase-map.html)")
    parser.add_argument("--no-open", action="store_true", help="Do not open the generated HTML in a browser")
    args = parser.parse_args()

    root = Path(args.path).resolve()
    if not root.exists() or not root.is_dir():
        print(f"error: not a directory: {root}", file=sys.stderr)
        return 2

    out_path = Path(args.output).resolve()
    out_path.write_text(build_html(root, DEFAULT_IGNORES), encoding="utf-8")
    print(f"wrote: {out_path}")

    if not args.no_open:
        webbrowser.open(out_path.as_uri())

    return 0


if __name__ == "__main__":
    raise SystemExit(main())

