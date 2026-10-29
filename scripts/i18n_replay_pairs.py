#!/usr/bin/env python3
"""Generate i18n codemod pair drafts from an ours-vs-worktree diff.

Phase R replay driver: for each differing hunk between the ours commit and the
working tree (which carries the upstream merge skeleton), emit an exact
``(old, new)`` snippet pair with the ours-side hunk as ``new`` and the
worktree-side hunk as ``old``. The caller reviews the draft (drop pairs that
are pure upstream additions, keep wrapping pairs), then applies them through
``i18n_codemod.py`` like any other reviewed mapping.

Usage:
    python3 scripts/i18n_replay_pairs.py \\
        --ours f9fd6c1dee --ours-file <path@ours> \\
        --file <working-tree path> \\
        [--out <pairs.json>] [--limit N]

Only hunks whose ours side contains a ``tr(`` / ``tr_with(`` call are kept:
those are the wrapping pairs. Upstream-only hunks (no tr in ours) are reported
as ``skip_upstream`` and left out of the pair list, which the reviewer should
sanity-check rather than trust blindly.
"""

from __future__ import annotations

import argparse
import difflib
import json
import subprocess
import sys
from pathlib import Path


def ours_content(path: str, rev: str) -> str:
    out = subprocess.run(
        ["git", "show", f"{rev}:{path}"], capture_output=True, check=True, text=True
    )
    return out.stdout


def has_tr(lines: list[str]) -> bool:
    return any("tr(" in line or "tr_with(" in line for line in lines)


def make_pairs(ours: str, ours_path: str, worktree: str) -> tuple[list[dict], int]:
    ours_lines = ours.splitlines(keepends=True)
    wt_lines = worktree.splitlines(keepends=True)
    matcher = difflib.SequenceMatcher(a=wt_lines, b=ours_lines, autojunk=False)
    pairs: list[dict] = []
    skipped = 0
    for tag, a0, a1, b0, b1 in matcher.get_opcodes():
        if a0 == a1 and b0 == b1:
            continue
        old = wt_lines[a0:a1]
        new = ours_lines[b0:b1]
        if not new:
            skipped += 1
            continue
        if not has_tr(new):
            skipped += 1
            continue
        pairs.append(
            {
                "old": "".join(old),
                "new": "".join(new),
                "source": ours_path,
            }
        )
    return pairs, skipped


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--ours", required=True, help="ours revision (e.g. f9fd6c1dee)")
    ap.add_argument("--ours-file", required=True, help="path at the ours revision")
    ap.add_argument("--file", required=True, help="working-tree path")
    ap.add_argument("--out", default=None, help="output JSON path")
    ap.add_argument("--limit", type=int, default=0, help="cap pair count (0 = all)")
    args = ap.parse_args()

    wt_path = Path(args.file)
    ours = ours_content(args.ours_file, args.ours)
    wt = wt_path.read_text()
    pairs, skipped = make_pairs(ours, args.ours_file, wt)
    if args.limit and args.limit < len(pairs):
        pairs = pairs[: args.limit]
    if not pairs:
        print(f"{args.file}: no tr-bearing hunks (skipped {skipped} upstream-only)")
        return 0
    payload = {"pairs": pairs}
    out = args.out
    if out is None:
        out = f"/tmp/replay-{wt_path.parent.name}-{wt_path.stem}.json"
    Path(out).write_text(json.dumps(payload, indent=1))
    print(
        f"{args.file}: {len(pairs)} pairs (skipped {skipped} upstream-only hunks) -> {out}"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
