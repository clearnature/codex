#!/usr/bin/env python3
"""List the translatable strings that are *not* wrapped in `tr` yet.

`i18n_scan.py` answers "how much of the UI is translatable"; `i18n-check` answers
"is every wrapped string translated". Neither answers the question the rollout
actually needs: **what is left to wrap?** This script does, and it is the only
honest source for a coverage claim.

It reuses the scanner's classification (importing it, so the rule set stays in
one place) and then removes the literals that already sit inside a
``tr(..)`` / ``tr_with(..)`` call, by looking at the same line for the call.

Usage::

    python3 scripts/i18n_todo.py                 # per-module remaining counts
    python3 scripts/i18n_todo.py --top 15        # busiest files
    python3 scripts/i18n_todo.py --file history_cell/mcp.rs   # line-by-line
"""

from __future__ import annotations

import argparse
import importlib.util
import re
import sys
from collections import Counter
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
SCANNER = REPO / "scripts/i18n_scan.py"


def load_scanner():
    spec = importlib.util.spec_from_file_location("i18n_scan", SCANNER)
    module = importlib.util.module_from_spec(spec)
    assert spec.loader is not None
    spec.loader.exec_module(module)
    return module


WRAPPED = re.compile(r"\btr(?:_with)?\s*\(")

# How far back to look for the `tr(` that opens the call the literal sits in.
# Multi-line calls are the norm once rustfmt splits them, so a same-line check
# would report already-wrapped literals as remaining work -- measured on
# `plugin_catalog.rs`, a same-line-only rule inflated "remaining" by ~15%.
LOOKBACK = 4


def is_wrapped(lines: list[str], line_number: int) -> bool:
    """True when the literal on this line is the key of a `tr(..)` call.

    Looks at the line and the few lines above it: a call this script cares about
    is written as `tr(current(), "...")`, possibly broken across lines. The rule
    is still a heuristic and deliberately lenient -- a literal that merely *looks*
    wrapped is treated as wrapped, which can only under-report remaining work,
    never over-report it.
    """
    if not 1 <= line_number <= len(lines):
        return False
    start = max(1, line_number - LOOKBACK)
    for line in lines[start - 1 : line_number]:
        if WRAPPED.search(line):
            return True
    return False


def read_not_translated(path: Path) -> dict[str, str]:
    """Rows of `key<TAB>file:line<TAB>reason`; the reason is mandatory.

    Deliberately identical in shape to the file `codex-i18n-check` reads, so a
    verdict lives in one place per *kind* of check and both stay auditable.
    """
    if not path.exists():
        return {}
    out: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8").splitlines():
        line = line.rstrip()
        if not line or line.startswith("#"):
            continue
        key, _, rest = line.partition("\t")
        site, _, reason = rest.partition("\t")
        if not site or not reason.strip():
            raise SystemExit(f"{path}: row for {key!r} needs `file:line<TAB>reason`")
        out[key] = site
    return out


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--top", type=int, default=10, help="how many files to list")
    parser.add_argument("--file", help="list every remaining literal in one file")
    parser.add_argument(
        "--root",
        type=Path,
        action="append",
        help="directory to scan (repeatable); defaults to the scanner's TUI root",
    )
    args = parser.parse_args(argv)

    scanner = load_scanner()
    # Scan the TUI only, and use the scanner's own default root: passing the
    # repository root would sweep `target/` build artifacts and the dictionary
    # itself (its Chinese strings look like candidates by shape), which is how
    # this script first produced a headline number that was 20x the real one.
    # Widening it later (cli / exec / core) means adding roots here on purpose.
    roots = args.root or [scanner.DEFAULT_ROOT]
    findings = [
        f for fs in roots for f in scanner.scan(fs) if f["bucket"] == "candidates"
    ]
    findings = [f for f in findings if "/dict_zh.rs" not in f["path"]]

    # Judged-not-translatable literals, same shape as
    # `codex-rs/i18n/not-translated.tsv` but a different scope: that one filters
    # `codex-i18n-check`'s *rendered* keys (literals already wrapped in `tr`),
    # this one filters the *unwrapped* candidates listed here. Same object,
    # two tools -- one sees it, the other does not.
    not_translated = read_not_translated(REPO / "codex-rs" / "i18n" / "not-translated-unwrapped.tsv")

    cache: dict[str, list[str]] = {}
    remaining = []
    for finding in findings:
        path = finding["path"]
        if path not in cache:
            cache[path] = (
                (REPO / path).read_text(encoding="utf-8", errors="replace").splitlines()
            )
        if is_wrapped(cache[path], finding["line"]):
            continue
        if finding["value"] in not_translated:
            continue
        remaining.append(finding)

    if args.file:
        wanted = [f for f in remaining if f["path"].endswith(args.file)]
        # Report the exemptions *per file* too: without it, a file whose
        # remaining candidates are all exemptions prints `0 unwrapped` and the
        # reader cannot tell "nothing left" from "everything was exempted".
        skipped = sum(
            1
            for finding in findings
            if finding["path"].endswith(args.file)
            and finding["value"] in not_translated
            and not is_wrapped(
                (REPO / finding["path"]).read_text(encoding="utf-8", errors="replace").splitlines(),
                finding["line"],
            )
        )
        print(f"{len(wanted)} unwrapped candidates in *{args.file}")
        if skipped:
            print(f"  ({skipped} exempted by not-translated-unwrapped.tsv)")
        for finding in wanted:
            print(f"  {finding['path']}:{finding['line']}: {finding['value'][:90]!r}")
        return 0

    by_file = Counter(f["path"] for f in remaining)
    by_module = Counter(f["module"] for f in remaining)
    print(f"unwrapped candidates : {len(remaining)}")
    print(f"declared not-translatable (skipped): {len(not_translated)}")
    print(f"all candidates       : {len(findings)}")
    print(f"wrapped so far       : {len(findings) - len(remaining)}")
    print()
    print("module                        remaining")
    for module, count in by_module.most_common(None):
        print(f"{module:<30}{count}")
    print()
    print(f"busiest files (top {args.top}):")
    for path, count in by_file.most_common(args.top):
        print(f"  {count:>4}  {path}")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
