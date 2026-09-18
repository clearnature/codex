#!/usr/bin/env python3
"""Keep the i18n dossier's site column honest.

`codex-rs/i18n/not-translated-unwrapped.tsv` matches by **value** (column 1), so
a stale `path:line` never breaks the exemption -- but it does mislead whoever
reads the row to check a judgement. Any edit to a registered file shifts the
lines after it, and this project registers by the hundred, so drift is routine:
one sweep found 27 stale rows, the next found 7.

    python3 scripts/i18n_dossier_lines.py            # report only, exit 1 on drift
    python3 scripts/i18n_dossier_lines.py --fix      # rewrite the site column

Rows it cannot decide are reported separately and never rewritten:

*   a value with no text form to search for (empty key, real newlines, escaped
    quotes) -- those are matched by site alone;
*   a value that now appears on several lines, so "which one is *the* site" is
    ambiguous;
*   a value that appears nowhere in the file (the literal moved out, or the row
    is stale in a way only a human can settle).
"""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
DOSSIER = REPO / "codex-rs" / "i18n" / "not-translated-unwrapped.tsv"
SITE = re.compile(r"^(?P<path>.+):(?P<line>\d+)$")


def unescape(value: str) -> str:
    """Undo the two escapes the dossier uses so the text can be searched for."""
    return value.replace("\\n", "\n").replace('\\"', '"')


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--fix", action="store_true", help="rewrite the site column")
    parser.add_argument("--dossier", type=Path, default=DOSSIER)
    args = parser.parse_args(argv)

    lines = args.dossier.read_text(encoding="utf-8").splitlines(True)
    sources: dict[str, list[str]] = {}
    output: list[str] = []
    in_place, corrected, undecided = 0, [], []

    for raw in lines:
        if raw.startswith("#") or "\t" not in raw:
            output.append(raw)
            continue
        cells = raw.rstrip("\n").split("\t")
        key, site = cells[0], cells[1]
        match = SITE.match(site)
        if not key or not match:
            output.append(raw)
            continue
        path, line = match.group("path"), int(match.group("line"))
        target = Path(path)
        if not target.exists():
            undecided.append((site, "file gone"))
            output.append(raw)
            continue
        if path not in sources:
            sources[path] = target.read_text(
                encoding="utf-8", errors="replace"
            ).splitlines()
        source = sources[path]

        # Containment, not a prefix: a 40-character prefix matches unrelated
        # lines and silently "confirms" a row that has in fact drifted.
        wanted = unescape(key)
        if line <= len(source) and wanted in source[line - 1]:
            in_place += 1
            output.append(raw)
            continue
        found = [index + 1 for index, text in enumerate(source) if wanted in text]
        if len(found) == 1:
            cells[1] = f"{path}:{found[0]}"
            corrected.append((site, cells[1], key[:44]))
            output.append("\t".join(cells) + "\n")
        else:
            undecided.append((site, f"{len(found)} candidate line(s)"))
            output.append(raw)

    print(f"site column correct : {in_place}")
    print(f"site column drifted : {len(corrected)}")
    for old, new, key in corrected:
        print(f"    {old} -> {new.split(':')[-1]}  {key}")
    print(f"undecidable rows    : {len(undecided)}")
    for site, why in undecided[:10]:
        print(f"    {site}  ({why})")

    if corrected and args.fix:
        args.dossier.write_text("".join(output), encoding="utf-8")
        print(f"rewrote {len(corrected)} row(s) in {args.dossier}")
        return 0
    if corrected:
        print("re-run with --fix to rewrite them")
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
