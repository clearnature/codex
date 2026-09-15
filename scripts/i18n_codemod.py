#!/usr/bin/env python3
"""Apply a reviewed, mechanical i18n wrapping to one Rust file.

H2 of ``docs/plan/i18n-verification.md`` asks whether the TUI's copy can be
identified mechanically well enough to justify a codemod. This is the codemod
half of that answer, kept deliberately small: it does **not** decide what to
wrap or how to translate. The caller brings an explicit mapping of exact
``(old, new)`` source snippets -- produced by reading the file -- and this tool
applies them all or none.

Why exact snippets instead of a regex over string literals: the interesting
cases are ``format!`` templates, whose argument list has to be rewritten into a
``tr_with(current(), "… {0} …", &[..])`` call. A signature-based rewriter would
have to guess; a snippet list is a reviewed decision, and the review is the part
that matters.

Contract:

* every pair must match at least once, and a pair that expects an exact number
  of matches must match exactly that many -- otherwise nothing is written and the
  offending pairs are listed as MISSING/MISCOUNT;
* ``--dry-run`` prints a unified diff and writes nothing;
* the exit code is 0 only when the file was (or would be) changed with every
  pair satisfied. A silent partial application is the failure mode this tool
  exists to prevent: it would leave a file half-wrapped and the drift check
  would then blame the dictionary.

Mapping file format (JSON)::

    {"pairs": [
      {"old": "name: \"Show usage\".to_string(),",
       "new": "name: tr(current(), \"Show usage\").to_string()"},
      {"old": "...", "new": "...", "count": "all"},
      {"old": "...", "new": "...", "count": 3}
    ]}

``count`` defaults to 1; ``"all"`` means "at least once, replace everywhere".
"""

from __future__ import annotations

import argparse
import difflib
import json
import sys
from pathlib import Path


def load_pairs(map_path: Path) -> list[dict]:
    data = json.loads(map_path.read_text(encoding="utf-8"))
    pairs = data["pairs"] if isinstance(data, dict) else data
    if not pairs:
        raise SystemExit(f"{map_path}: no pairs")
    for pair in pairs:
        if "old" not in pair or "new" not in pair:
            raise SystemExit(f"{map_path}: every pair needs `old` and `new`")
        if pair["old"] == pair["new"]:
            raise SystemExit(f"{map_path}: pair is a no-op: {pair['old'][:60]!r}")
    return pairs


def apply_pairs(text: str, pairs: list[dict]) -> tuple[str, list[str]]:
    """Returns the rewritten text and a list of problems (empty when clean)."""
    problems: list[str] = []
    out = text
    for pair in pairs:
        old, new = pair["old"], pair["new"]
        expected = pair.get("count", 1)
        found = out.count(old)
        if found == 0:
            problems.append(f"MISSING: {old.strip().splitlines()[0][:70]!r}")
            continue
        if expected != "all" and found != expected:
            problems.append(
                f"MISCOUNT: expected {expected}, found {found}: "
                f"{old.strip().splitlines()[0][:70]!r}"
            )
            continue
        out = out.replace(old, new)
    return out, problems


def unified_diff(before: str, after: str, path: Path, context: int) -> str:
    return "".join(
        difflib.unified_diff(
            before.splitlines(keepends=True),
            after.splitlines(keepends=True),
            fromfile=str(path),
            tofile=str(path),
            n=context,
        )
    )


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--file", required=True, type=Path, help="Rust file to rewrite")
    parser.add_argument("--map", required=True, type=Path, help="JSON mapping of pairs")
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="print the diff and write nothing",
    )
    parser.add_argument(
        "--context",
        type=int,
        default=1,
        help="lines of context in the diff (default 1)",
    )
    args = parser.parse_args(argv)

    before = args.file.read_text(encoding="utf-8")
    pairs = load_pairs(args.map)
    after, problems = apply_pairs(before, pairs)

    for problem in problems:
        print(problem, file=sys.stderr)
    if problems:
        print(
            f"refusing to write: {len(problems)} of {len(pairs)} pairs are unfulfilled",
            file=sys.stderr,
        )
        return 2

    if after == before:
        print("no change", file=sys.stderr)
        return 1

    diff = unified_diff(before, after, args.file, args.context)
    sys.stdout.write(diff)
    changed = sum(
        1
        for line in diff.splitlines()
        if line.startswith(("+", "-")) and not line.startswith(("+++", "---"))
    )
    print(f"-- {len(pairs)} pairs, {changed} changed lines", file=sys.stderr)

    if args.dry_run:
        print("dry run: nothing written", file=sys.stderr)
        return 0

    args.file.write_text(after, encoding="utf-8")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
