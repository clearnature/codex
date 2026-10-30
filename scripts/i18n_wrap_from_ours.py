#!/usr/bin/env python3
"""Generate literal-wrap i18n pairs from the ours revision, structure-safe.

Phase R replay, second generation: instead of replaying whole diff hunks
(which drags ours-era branch structure into the 165 skeleton and caused the
regressions seen on heavily-refactored files), this extracts **call-level**
wraps only. For every `tr(current(), "KEY")` / `tr_with(current(), "KEY",
&[...])` call in the ours revision, it emits a pair that replaces the bare
`"KEY"` string literal in the working tree with the full ours call form.

Merge principle enforced: the 165 branch structure is never touched; only
user-facing literals get wrapped. Keys absent from the zh dictionary are
reported and skipped (their translation must land first).

Usage:
    python3 scripts/i18n_wrap_from_ours.py --ours f9fd6c1dee \\
        --file <path> --out <pairs.json> [--dict <dict_zh.rs path>]
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path


CALL_RE = re.compile(
    r'tr(?:_with)?\(\s*current\(\),\s*"(?P<key>[^"]+)"'
    r'(?:,\s*(?P<args>&\[(?:[^\[\]]|\[(?:[^\[\]])*\])*?\]))?\s*\)',
    re.S,
)


def ours_text(rev: str, path: str) -> str:
    r = subprocess.run(
        ["git", "show", f"{rev}:{path}"], capture_output=True, text=True, check=True
    )
    return r.stdout


def dict_keys(dict_path: Path) -> set[str]:
    if not dict_path.exists():
        return set()
    text = dict_path.read_text()
    return set(re.findall(r'^\s*\(\s*"((?:[^"\\]|\\.)*)"', text, re.M))


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--ours", required=True)
    ap.add_argument("--file", required=True)
    ap.add_argument("--out", required=True)
    ap.add_argument("--dict", default="codex-rs/i18n/src/dict_zh.rs")
    args = ap.parse_args()

    src = ours_text(args.ours, args.file)
    work = Path(args.file).read_text()
    keys = dict_keys(Path(args.dict))

    pairs = []
    skipped_no_dict = 0
    skipped_no_literal = 0
    skipped_ambiguous = 0
    for m in CALL_RE.finditer(src):
        key = m.group("key")
        call = m.group(0)
        if key not in keys:
            skipped_no_dict += 1
            continue
        # call must wrap the literal itself (sanity)
        if f'"{key}"' not in call:
            continue
        literal = f'"{key}"'
        n = work.count(literal)
        if n == 0:
            skipped_no_literal += 1
            continue
        if n > 1:
            skipped_ambiguous += 1
            continue
        # only wrap when the working tree occurrence is NOT already a tr call
        idx = work.index(literal)
        before = work[max(0, idx - 40): idx]
        if re.search(r'tr(?:_with)?\(\s*$', before):
            skipped_ambiguous += 1
            continue
        pairs.append({"old": literal, "new": call, "source": args.file})

    if not pairs:
        print(
            f"{args.file}: 0 pairs (no_dict={skipped_no_dict} "
            f"no_literal={skipped_no_literal} ambiguous={skipped_ambiguous})"
        )
        return 0
    Path(args.out).write_text(json.dumps({"pairs": pairs}))
    print(
        f"{args.file}: {len(pairs)} wrap pairs (no_dict={skipped_no_dict} "
        f"no_literal={skipped_no_literal} ambiguous={skipped_ambiguous}) -> {args.out}"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
