#!/usr/bin/env python3
"""Look an English UI string up in the reference implementations' catalogues.

Two references are wired in, both of them production CLIs with a Chinese
catalogue, and both measured on 2026-09-17:

* ``qwen-code`` -- ``packages/cli/src/i18n/locales/{en,zh}.js``, 1632 keys,
  English-source-text-as-key (same strategy as ours). Overlap with our
  dictionary: **4 keys**.
* ``DeepSeek-Reasonix-studio`` -- ``internal/i18n/messages_{en,zh}.go``, 519
  typed fields joined by field name. Its TUI is modelled on Codex, so the copy
  is closer, but the overlap is still only **2 keys** (+3 after normalisation).

That measurement is the point of this tool: **neither reference is a translation
memory**. They are terminology and style references -- how does a shipped zh CLI
phrase "worktree", "hook", "reasoning effort"? -- and they are the source the
glossary's term table was decided from. Two practical caveats when borrowing:

* placeholders differ: we use positional ``{0}`` (``tr_with``), qwen uses
  ``{{name}}``, studio uses ``%s``/``%d`` -- copying a whole sentence means
  rewriting its placeholders;
* studio's own catalogue notes that status lines must be *whole* strings
  ("the count sits in a different place in each language, and one built from
  fragments reads well in none of them") -- independent confirmation of our
  "fragment sentence" special case in the design doc §3.6.

Usage::

    python3 scripts/i18n_ref.py --stats                # overlap with both
    python3 scripts/i18n_ref.py --key "Remove marketplace"
    python3 scripts/i18n_ref.py --contains worktree --source studio
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
QWEN = Path("/data/training/cli/qwen-code/packages/cli/src/i18n/locales")
STUDIO = Path("/home/yanli/work/DeepSeek-Reasonix-studio-v2.13.0/internal/i18n")
OUR_DICT = REPO / "codex-rs/i18n/src/dict_zh.rs"

JS_PAIR = re.compile(r"'((?:[^'\\]|\\.)*)'\s*:\s*'((?:[^'\\]|\\.)*)'", re.S)
GO_PAIR = re.compile(r'^\t([A-Z]\w*):\s*"((?:[^"\\]|\\.)*)",\s*$', re.M)
RUST_PAIR = re.compile(
    r'\(\s*"((?:[^"\\]|\\.)*)"\s*,\s*"((?:[^"\\]|\\.)*)"\s*,?\s*\)', re.S
)


def unescape(text: str) -> str:
    return (
        text.replace("\\'", "'")
        .replace('\\"', '"')
        .replace("\\n", "\n")
        .replace("\\\\", "\\")
    )


def read_js_pairs(path: Path) -> dict[str, str]:
    return {
        unescape(k): unescape(v) for k, v in JS_PAIR.findall(path.read_text("utf-8"))
    }


def read_go_fields(path: Path) -> dict[str, str]:
    return {k: unescape(v) for k, v in GO_PAIR.findall(path.read_text("utf-8"))}


def read_qwen(locale: str) -> dict[str, str]:
    path = QWEN / f"{locale}.js"
    return read_js_pairs(path) if path.exists() else {}


def read_studio(locale: str) -> dict[str, str]:
    en = read_go_fields(STUDIO / "messages_en.go")
    other = read_go_fields(STUDIO / f"messages_{locale}.go")
    return {en[k]: other[k] for k in en if k in other}


def our_dictionary() -> dict[str, str]:
    text = OUR_DICT.read_text("utf-8")
    body = text[text.index("static ENTRIES") :]
    return {unescape(k): unescape(v) for k, v in RUST_PAIR.findall(body)}


def normalise(text: str) -> str:
    return re.sub(r"[^a-z0-9]+", " ", text.lower()).strip()


def report_stats(locale: str, sources: dict[str, dict[str, str]]) -> None:
    ours = our_dictionary()
    print(f"our dictionary : {OUR_DICT.name} ({len(ours)} entries)")
    for name, ref in sources.items():
        exact = sorted(k for k in ours if k in ref)
        same = [k for k in exact if ours[k] == ref[k]]
        index = {normalise(k): (k, v) for k, v in ref.items()}
        near = [
            (k, index[normalise(k)])
            for k in ours
            if k not in ref and normalise(k) in index
        ]
        print(
            f"\n{name:<14}: {len(ref)} entries | exact overlap {len(exact)} ({len(same)} identical) | near {len(near)}"
        )
        for key in exact:
            print(f"  [{'same' if ours[key] == ref[key] else 'DIFF'}] {key!r}")
            if ours[key] != ref[key]:
                print(f"      ours  : {ours[key]}")
                print(f"      {name:<6}: {ref[key]}")
        for key, (ref_key, ref_value) in near[:10]:
            print(f"  [near] {key!r} ~ {ref_key!r}")
            print(f"      ours  : {ours[key]}")
            print(f"      {name:<6}: {ref_value}")


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--key", help="exact English key to look up")
    parser.add_argument("--contains", help="substring to search for (case-insensitive)")
    parser.add_argument("--stats", action="store_true", help="measure the overlap")
    parser.add_argument("--locale", default="zh", help="reference locale (default zh)")
    parser.add_argument(
        "--source",
        choices=["all", "qwen", "studio"],
        default="all",
        help="which reference to read (default all)",
    )
    args = parser.parse_args(argv)

    sources = {}
    if args.source in ("all", "qwen"):
        qwen = read_qwen(args.locale)
        if qwen:
            sources["qwen-code"] = qwen
    if args.source in ("all", "studio"):
        studio = read_studio(args.locale)
        if studio:
            sources["studio"] = studio
    if not sources:
        print("no reference available", file=sys.stderr)
        return 2

    if args.stats or not (args.key or args.contains):
        report_stats(args.locale, sources)
        return 0

    if args.key:
        found = False
        for name, ref in sources.items():
            if args.key in ref:
                print(f"{name:<14}: {ref[args.key]}")
                found = True
        if not found:
            print("not in any reference", file=sys.stderr)
            return 1
        return 0

    needle = args.contains.lower()
    for name, ref in sources.items():
        matches = {k: v for k, v in ref.items() if needle in k.lower()}
        print(f"{name}: {len(matches)} entries contain {args.contains!r}")
        for key, value in list(matches.items())[:20]:
            print(f"  {key}")
            print(f"      {value}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
