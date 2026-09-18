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


ENCLOSING_CALL = re.compile(r"([A-Za-z_][A-Za-z0-9_]*)\s*$")


def line_start_offset(text: str, line_number: int) -> int | None:
    """Character offset of the first character of a 1-based line number."""
    pos = 0
    for _ in range(line_number - 1):
        pos = text.find("\n", pos)
        if pos < 0:
            return None
        pos += 1
    return pos


def is_wrapped_precise(
    source: str, masked: str, lines: list[str], finding: dict
) -> bool:
    """True when the literal is a key of the `tr`/`tr_with` call enclosing it.

    `is_wrapped` only looks at the surrounding lines, so a literal that merely
    sits *next to* a `tr(..)` call -- a sibling field of the same struct
    literal, or a match arm printed below a wrapped one -- counts as wrapped.
    Measured on `chatwidget/model_popups.rs:625` that leniency hid a real gap:
    a `format!` fragment one line under `name: tr(current(), "More reasoning…")`.

    This rule walks backwards from the literal and stops at the innermost call
    that encloses it, which is what "wrapped in `tr`" actually means. It is
    opt-in (`--precise`) so the default report stays comparable over time.
    """
    start = line_start_offset(source, finding["line"])
    if start is None:
        return False
    raw = lines[finding["line"] - 1] if 0 < finding["line"] <= len(lines) else ""
    # Locate the literal itself, not the first occurrence of its text: the same
    # text also appears *outside* quotes when an identifier repeats it
    # (`SubAgentActivityKind::Started => (tr(current(), "Started ")`), and
    # anchoring there reports an already-wrapped literal as remaining work. The
    # scanner stores the raw source between the quotes, so the quoted form is
    # the exact needle.
    column = raw.find(f'"{finding["value"]}"')
    if column < 0:
        column = raw.find(finding["value"])
    if column < 0:
        column = raw.find('"')
    if column < 0:
        return False
    depth = 0
    index = start + column - 1
    while 0 <= index < len(masked):
        char = masked[index]
        if char == ")":
            depth += 1
        elif char == "(":
            if depth == 0:
                enclosing = ENCLOSING_CALL.search(masked[:index])
                return bool(enclosing and enclosing.group(1) in ("tr", "tr_with"))
            depth -= 1
        elif char in ";{}" and depth == 0:
            return False
        index -= 1
    return False


# Macros whose string literals are legitimately internal: a literal *really*
# inside one of these is not a hidden user-visible message, so `--suspect`
# must not report it. Names are matched without their path (`tracing::warn`
# and `warn` are the same macro here).
BENIGN_ENCLOSING = re.compile(
    r"^(?:debug|info|warn|error|trace)$"  # log macros (incl. tracing::)
    r"|^(?:assert|assert_eq|assert_ne|debug_assert|debug_assert_eq|debug_assert_ne"
    r"|panic|unreachable|expect|matches)$"  # invariant / assertion text
    r"|^(?:instrument|span|event)$"  # span names
)


def literal_offset(lines: list[str], line_number: int, source: str = "") -> int:
    """Offset of the literal that starts on `line_number` (1-based).

    The opening quote, not the line start: a macro written as
    `warn!("...")` puts its `(` *before* the quote on the same line, and
    `enclosing_macro` searches only the text before this offset -- using the
    line start would hide exactly the macro we are trying to identify.
    """
    start = sum(len(line) + 1 for line in lines[: line_number - 1])
    if source:
        quote = source.find('"', start)
        if 0 <= quote - start <= 400:
            return quote
    return start


def enclosing_macro(text: str, pos: int) -> str | None:
    """Name of the innermost `ident!(...)` whose parentheses span `pos`.

    `None` when the literal is not inside any macro invocation (a plain
    `format!(..)` argument reaches its own macro, an attribute `#[allow(..,
    reason = "..")]` reaches none and is reported as `<attribute/proximity>`).
    """
    best: tuple[int, str] | None = None
    for match in re.finditer(r"([A-Za-z_][\w:]*)\s*!\s*\(", text[:pos]):
        start = match.end() - 1
        depth = 0
        index = start
        while index < len(text):
            if text[index] == "(":
                depth += 1
            elif text[index] == ")":
                depth -= 1
                if depth == 0:
                    break
            index += 1
        if start < pos < index and (best is None or start > best[0]):
            best = (start, match.group(1).split("::")[-1])
    return best[1] if best else None


def is_inside_attribute(text: str, pos: int) -> bool:
    """True when `pos` sits inside a `#[...]` attribute.

    `#[instrument(name = "session.flush_rollout")]` and clippy's
    `#[allow(.., reason = "..")]` carry string literals that are span names and
    lint justifications -- developer-facing, never rendered -- yet they land in
    the demoted buckets because `classify` matches macro-shaped context around
    them. Reported as `enclosing=<attribute/proximity>` otherwise.
    """
    opened = text.rfind("#[", 0, pos)
    if opened < 0:
        return False
    return text.rfind("]", opened, pos) < 0


def read_not_translated(path: Path) -> tuple[set[str], set[str]]:
    """Returns (keys, sites): rows are `key<TAB>site<TAB>reason`.

    A row with an **empty key** means "exempt this *site*", which is the only
    way to exempt a literal spanning several lines: the value cannot be written
    in a one-line TSV field, but its position can.
    """
    if not path.exists():
        return set(), set()
    keys: set[str] = set()
    sites: set[str] = set()
    for line in path.read_text(encoding="utf-8").splitlines():
        line = line.rstrip()
        if not line or line.startswith("#"):
            continue
        key, _, rest = line.partition("\t")
        site, _, reason = rest.partition("\t")
        if not site or not reason.strip():
            raise SystemExit(
                f"{path}: row for {key!r} needs `key<TAB>file:line<TAB>reason`"
            )
        if key:
            keys.add(key)
        else:
            sites.add(site.rsplit(":", 1)[-1] if False else site)
    return keys, sites


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--top", type=int, default=10, help="how many files to list")
    parser.add_argument("--file", help="list every remaining literal in one file")
    parser.add_argument(
        "--precise",
        action="store_true",
        help=(
            "judge a literal by the call that encloses it, and list the ones the "
            "lenient same-line lookback hides (additive: the default report is "
            "unchanged)"
        ),
    )
    parser.add_argument(
        "--suspect",
        action="store_true",
        help=(
            "list the undecided literals the candidate bucket hides "
            "(internal:assert / internal:log); a `unwrap*()`/`assert*()`/log macro "
            "in the three lines above demotes a user-visible literal out of the "
            "candidate bucket, so this is the sweep that catches that blind spot"
        ),
    )
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
    scanned = [f for fs in roots for f in scanner.scan(fs)]
    findings = [f for f in scanned if f["bucket"] == "candidates"]
    findings = [f for f in findings if "/dict_zh.rs" not in f["path"]]

    # Judged-not-translatable literals, same shape as
    # `codex-rs/i18n/not-translated.tsv` but a different scope: that one filters
    # `codex-i18n-check`'s *rendered* keys (literals already wrapped in `tr`),
    # this one filters the *unwrapped* candidates listed here. Same object,
    # two tools -- one sees it, the other does not.
    not_translated = read_not_translated(
        REPO / "codex-rs" / "i18n" / "not-translated-unwrapped.tsv"
    )

    cache: dict[str, tuple[str, str, list[str]]] = {}
    remaining = []
    # Literals the lenient rule calls wrapped although no `tr` call encloses
    # them. Filled only under `--precise`; this is the triage list.
    hidden = []
    for finding in findings:
        path = finding["path"]
        if path not in cache:
            source = (REPO / path).read_text(encoding="utf-8", errors="replace")
            cache[path] = (source, scanner.mask_source(source), source.splitlines())
        source, masked, lines = cache[path]
        lenient = is_wrapped(lines, finding["line"])
        wrapped = (
            is_wrapped_precise(source, masked, lines, finding)
            if args.precise
            else lenient
        )
        if wrapped:
            continue
        site = f"{finding['path']}:{finding['line']}"
        if finding["value"] in not_translated[0] or site in not_translated[1]:
            continue
        remaining.append(finding)
        if args.precise and lenient:
            hidden.append(finding)

    if args.suspect:
        suspects = []
        for finding in scanned:
            if finding["bucket"] not in ("internal:assert", "internal:log"):
                continue
            path = finding["path"]
            if path not in cache:
                source = (REPO / path).read_text(encoding="utf-8", errors="replace")
                cache[path] = (source, scanner.mask_source(source), source.splitlines())
            source, _masked, lines = cache[path]
            if is_wrapped(lines, finding["line"]):
                continue
            site = f"{finding['path']}:{finding['line']}"
            if finding["value"] in not_translated[0] or site in not_translated[1]:
                continue
            if len(finding["value"].strip()) < scanner.MIN_CANDIDATE_LEN:
                continue
            # Only *proximity* demotions are interesting. A literal that really is
            # inside a log macro / assert / `#[instrument]` name / clippy
            # `reason = ".."` is legitimately internal, and listing those drowned
            # the signal (56 of 57 in `session/mod.rs` were of that shape, against
            # exactly one real find). `enclosing_macro` names the innermost
            # `ident!(...)` spanning the literal.
            macro = enclosing_macro(
                source, literal_offset(lines, finding["line"], source)
            )
            inside_attribute = is_inside_attribute(
                source, literal_offset(lines, finding["line"], source)
            )
            if inside_attribute:
                continue
            if macro is not None and BENIGN_ENCLOSING.match(macro):
                continue
            suspects.append((finding, macro))
        wanted_suspects = [
            (f, m)
            for f, m in suspects
            if not args.file or f["path"].endswith(args.file)
        ]
        scope = f"*{args.file}" if args.file else "the scanned roots"
        print(
            f"{len(wanted_suspects)} undecided literals in internal:assert/internal:log ({scope})"
        )
        for finding, macro in wanted_suspects:
            where = f"enclosing={macro or '<attribute/proximity>'}"
            print(
                f"  {finding['path']}:{finding['line']}: {finding['value'][:80]!r}  [{where}]"
            )
        return 0

    if args.file:
        wanted = [f for f in remaining if f["path"].endswith(args.file)]
        # Report the exemptions *per file* too: without it, a file whose
        # remaining candidates are all exemptions prints `0 unwrapped` and the
        # reader cannot tell "nothing left" from "everything was exempted".
        skipped = sum(
            1
            for finding in findings
            if finding["path"].endswith(args.file)
            and (
                finding["value"] in not_translated[0]
                or f"{finding['path']}:{finding['line']}" in not_translated[1]
            )
            and not is_wrapped(
                (REPO / finding["path"])
                .read_text(encoding="utf-8", errors="replace")
                .splitlines(),
                finding["line"],
            )
        )
        print(f"{len(wanted)} unwrapped candidates in *{args.file}")
        if skipped:
            print(f"  ({skipped} exempted by not-translated-unwrapped.tsv)")
        hidden_sites = {f"{f['path']}:{f['line']}" for f in hidden}
        for finding in wanted:
            site = f"{finding['path']}:{finding['line']}"
            mark = "   [the lenient rule hid this]" if site in hidden_sites else ""
            print(f"  {site}: {finding['value'][:90]!r}{mark}")
        return 0

    by_file = Counter(f["path"] for f in remaining)
    by_module = Counter(f["module"] for f in remaining)

    # Literals the *candidate* bucket hides. `classify` rejects a literal when a
    # `unwrap*()`/`assert*()`/log macro appears in the three lines above it, so a
    # genuinely user-visible string can land in `internal:assert` / `internal:log`
    # and never show up as a candidate -- "0 unwrapped candidates" then does not
    # mean "nothing left to look at". Measured on `guardian/review.rs`: the
    # `ReviewDecision::denied(..)` reason sat right under an `unwrap_or(..)` and
    # reaches the user through `ToolError::Rejected` (tools/approvals.rs:456).
    # This lists the ones still undecided (not wrapped, not declared).
    print(f"unwrapped candidates : {len(remaining)}")
    print(
        f"declared not-translatable (skipped): {len(not_translated[0]) + len(not_translated[1])}"
    )
    print(f"all candidates       : {len(findings)}")
    print(f"wrapped so far       : {len(findings) - len(remaining)}")
    if args.precise:
        print(f"hidden by the lenient rule (no `tr` encloses them): {len(hidden)}")
        for finding in hidden:
            print(f"  {finding['path']}:{finding['line']}: {finding['value'][:90]!r}")
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
