#!/usr/bin/env python3
"""Read-only scan of codex-rs/tui/src for translatable strings.

This is the instrument for H2 in ``docs/plan/i18n-verification.md``. H2 asks a
single question: can user-visible strings be told apart from internal ones
mechanically, well enough to justify a codemod?

The script enumerates Rust string literals, applies the rule set documented in
``classify`` below, and prints statistics plus a reproducible random sample for
manual review. It never writes to the scanned tree.

Measured on 2026-09-16 (feat/i18n @ rust-v0.154.0, seed 20260916):

* 39,218 literals in tui/src, of which 3,168 (8.1%) land in ``candidates``.
* ``candidates`` precision ~= 85-90% on a 40-item sample. The residual false
  positives are strings handed to the model (``json!`` tool payloads, IDE
  context prompts) and internal error-context strings -- both look exactly like
  UI text by shape, so H2 cannot separate them without an explicit exclusion.
* ``internal:identifier`` is accurate: 3/40 sampled items were really labels.
* ``internal:short`` is the weak rule: 9-11 of 80 sampled items were genuine UI
  labels (``Cancel``, ``Plugins``, ``Ready``, ``files``, ``Global``...). A
  length threshold cannot see short labels, so they need an explicit list
  rather than shape heuristics.

Two rule fixes came out of the first review round and are worth keeping: files
named ``tests.rs`` were not recognised as tests (7 of 40 first-round false
positives), and log macros spanning several lines hid their call from a
same-line check.

Usage:
    scripts/i18n_scan.py
    scripts/i18n_scan.py --sample 60 --seed 20260916
    scripts/i18n_scan.py --only candidates
"""

import argparse
import random
import re
import sys
from collections import Counter, defaultdict
from dataclasses import dataclass
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[1]
DEFAULT_ROOT = REPO_ROOT / "codex-rs" / "tui" / "src"

# A literal shorter than this is never a candidate: at that length the space
# heuristic cannot separate labels from keys, and the plan's threshold rule
# acknowledges the trade-off explicitly.
MIN_CANDIDATE_LEN = 8

RAW_STRING_START = re.compile(r'(?:b|c)?r(?P<hashes>#{0,255})"')
BYTE_STRING_START = re.compile(r'[bc]"')
CFG_TEST = re.compile(r"#\s*\[\s*cfg\s*\(\s*test\s*\)\s*\]")
URL = re.compile(r"^(?:[a-zA-Z][a-zA-Z0-9+.\-]*://|https?://|www\.)")
PATH_LIKE = re.compile(
    r"""^(?:
        [~./]                       # absolute / relative / home
        |[A-Za-z0-9_.\-]+/          # dir/file
        |\*?\.[A-Za-z0-9]+$         # bare extension
    )""",
    re.VERBOSE,
)
FILE_NAME = re.compile(
    r"^[\w.\-]+\.(?:rs|toml|json|md|jsonl|sock|lock|ya?ml|log|txt|snap)$"
)
SNAPSHOT_NAME = re.compile(r"^[a-z0-9_]+_snapshot$|^[a-z0-9_]{12,}$")
PLACEHOLDER_ONLY = re.compile(r"^[^\w]*(\{[^}]*\}[^\w]*)+$")
# A *quoted* key followed by a colon is a JSON payload entry -- data handed to
# the model or written to disk, not text shown to a user. A UI label like
# `name: "Yes, continue anyway"` has an unquoted key and stays a candidate.
JSON_ENTRY = re.compile(r'^\s*"[^"]*"\s*:')
# Literals used to *compare* against other text (or against protocol values)
# are never displayed, so they must not be wrapped by a codemod. Measured on
# the first 40-item candidate sample: 3 of 6 false positives were of this shape.
STRING_MATCH = re.compile(
    r"\.(?:contains|starts_with|ends_with|find|strip_prefix|strip_suffix)\s*\(\s*&?format!?\s*\(?"
    r"|\.(?:contains|starts_with|ends_with|find|strip_prefix|strip_suffix)\s*\(\s*\""
    r"|==\s*Some\s*\(\s*\"|!=\s*Some\s*\(\s*\"|==\s*\""
)
LOG_CALL = re.compile(
    r"(?:tracing::|debug!|info!|warn!|error!|trace!|instrument|span!|event!\s*\(|"
    r"log::|debug_assert|\.info\s*\(|\.warn\s*\(|\.error\s*\(|\.debug\s*\()"
)
ASSERT_CALL = re.compile(
    r"\b(?:assert\w*|panic!|unreachable!|expect\s*\(|unwrap\w*\s*\()"
)

# Macros that span several lines put the call above the literal, so log/assert
# context is searched over a small window rather than the literal's own line.
# Without this the 40-item candidate sample contained a logging call counted as
# user-visible text.
CONTEXT_LOOKBACK = 3


def has_whitespace(text):
    return any(ch.isspace() for ch in text)


def has_wide_text(text):
    """True if the string contains non-Latin text (would still need translating)."""
    return any(ord(ch) > 0x2000 and not ch.isspace() for ch in text)


@dataclass(frozen=True)
class Literal:
    line: int
    value: str
    kind: str


def mask_source(text):
    """Blank out comments, string literals and char literals, preserving offsets.

    Returns ``(masked, literal_spans)`` where ``literal_spans`` maps to nothing
    for the caller that only needs structural analysis.
    """
    out = list(text)
    i = 0
    n = len(text)
    while i < n:
        two = text[i : i + 2]
        if two == "//":
            j = text.find("\n", i)
            j = n if j < 0 else j
            for k in range(i, j):
                if out[k] != "\n":
                    out[k] = " "
            i = j
            continue
        if two == "/*":
            depth = 1
            i += 2
            while i < n and depth:
                if text.startswith("/*", i):
                    depth += 1
                    i += 2
                elif text.startswith("*/", i):
                    depth -= 1
                    i += 2
                else:
                    if out[i] != "\n":
                        out[i] = " "
                    i += 1
            continue
        raw = RAW_STRING_START.match(text, i)
        if raw and not (i > 0 and (text[i - 1].isalnum() or text[i - 1] == "_")):
            hashes = raw.group("hashes")
            close = '"' + hashes
            start = raw.end()
            end = text.find(close, start)
            end = n if end < 0 else end + len(close)
            for k in range(i, end):
                if out[k] != "\n":
                    out[k] = " "
            i = end
            continue
        if text[i] == '"':
            j = i + 1
            while j < n:
                if text[j] == "\\":
                    j += 2
                    continue
                if text[j] == '"':
                    j += 1
                    break
                j += 1
            for k in range(i, min(j, n)):
                if out[k] != "\n":
                    out[k] = " "
            i = j
            continue
        if text[i] == "'":
            # Char literal or lifetime; only blank the former.
            m = re.match(r"'(?:\\.|\\u\{[0-9A-Fa-f_]+\}|[^'\\])'", text[i:])
            if m:
                for k in range(i, i + m.end()):
                    if out[k] != "\n":
                        out[k] = " "
                i += m.end()
                continue
        i += 1
    return "".join(out)


def iter_literals(text):
    """Yield every string literal in ``text`` with its 1-based line number."""
    i = 0
    n = len(text)
    line = 1
    while i < n:
        c = text[i]
        if c == "\n":
            line += 1
            i += 1
            continue
        two = text[i : i + 2]
        if two == "//":
            j = text.find("\n", i)
            i = n if j < 0 else j
            continue
        if two == "/*":
            depth = 1
            i += 2
            while i < n and depth:
                if text.startswith("/*", i):
                    depth += 1
                    i += 2
                elif text.startswith("*/", i):
                    depth -= 1
                    i += 2
                else:
                    line += 1 if text[i] == "\n" else 0
                    i += 1
            continue
        raw = RAW_STRING_START.match(text, i)
        if raw and not (i > 0 and (text[i - 1].isalnum() or text[i - 1] == "_")):
            hashes = raw.group("hashes")
            close = '"' + hashes
            start = raw.end()
            end = text.find(close, start)
            if end < 0:
                break
            value = text[start:end]
            yield Literal(line, value, "raw")
            line += value.count("\n")
            i = end + len(close)
            continue
        if c == '"' or (BYTE_STRING_START.match(text, i) and text[i] != '"'):
            start = i + 1 if c == '"' else i + 2
            j = start
            buf = []
            while j < n:
                if text[j] == "\\":
                    buf.append(text[j : j + 2])
                    j += 2
                    continue
                if text[j] == '"':
                    break
                buf.append(text[j])
                j += 1
            value = "".join(buf)
            yield Literal(line, value, "str")
            line += value.count("\n")
            i = j + 1
            continue
        if text[i] == "'":
            m = re.match(r"'(?:\\.|\\u\{[0-9A-Fa-f_]+\}|[^'\\])'", text[i:])
            if m:
                i += m.end()
                continue
        i += 1


def cfg_test_ranges(text):
    """Line ranges covered by ``#[cfg(test)] mod ... { ... }`` blocks."""
    masked = mask_source(text)
    ranges = []
    for match in CFG_TEST.finditer(masked):
        brace = masked.find("{", match.end())
        if brace < 0:
            continue
        depth = 0
        for pos in range(brace, len(masked)):
            if masked[pos] == "{":
                depth += 1
            elif masked[pos] == "}":
                depth -= 1
                if depth == 0:
                    start_line = masked.count("\n", 0, match.start()) + 1
                    end_line = masked.count("\n", 0, pos) + 1
                    ranges.append((start_line, end_line))
                    break
    return ranges


def is_test_file(path):
    parts = path.parts
    name = path.name
    return (
        "tests" in parts
        or "snapshots" in parts
        or name.endswith("_tests.rs")
        or name.endswith("_test.rs")
        or name == "tests.rs"
        or name.startswith("test_")
    )


def classify(literal, context, path, in_test_module, test_file, lookback=""):
    """Return a bucket name.

    ``candidates`` is "looks user-visible"; everything else names the rule that
    rejected it, so the manual review can check each rule separately.
    """
    value = literal.value
    stripped = value.strip()
    if test_file or in_test_module:
        return "internal:test"
    if URL.match(stripped):
        return "internal:url"
    if FILE_NAME.match(stripped) or PATH_LIKE.match(stripped):
        return "internal:path"
    if SNAPSHOT_NAME.match(stripped):
        return "internal:name"
    if PLACEHOLDER_ONLY.match(stripped):
        return "internal:placeholder"
    if JSON_ENTRY.match(context):
        return "internal:data"
    if STRING_MATCH.search(context):
        return "internal:match"
    if LOG_CALL.search(lookback):
        return "internal:log"
    if ASSERT_CALL.search(lookback):
        return "internal:assert"
    if len(value) < MIN_CANDIDATE_LEN:
        return "internal:short"
    if not (has_whitespace(value) or has_wide_text(value)):
        return "internal:identifier"
    return "candidates"


def module_of(path, root):
    rel = path.relative_to(root)
    return rel.parts[0] if len(rel.parts) > 1 else "<top>"


def display_path(path):
    """A location for the report: relative to REPO_ROOT when that is possible.

    ``--root`` may point outside the repository (scanning a copy, or a caller
    passing ``.``), and a path that cannot be made relative must not turn a
    statistics run into a traceback. An absolute path is still a usable
    location, so that is the fallback.
    """
    try:
        return str(path.relative_to(REPO_ROOT))
    except ValueError:
        return str(path)


def scan(root):
    findings = []
    for path in sorted(root.rglob("*.rs")):
        text = path.read_text(encoding="utf-8", errors="replace")
        ranges = cfg_test_ranges(text)
        test_file = is_test_file(path)
        lines = text.splitlines()
        for literal in iter_literals(text):
            context = lines[literal.line - 1] if literal.line - 1 < len(lines) else ""
            lookback = "\n".join(
                lines[max(0, literal.line - 1 - CONTEXT_LOOKBACK) : literal.line]
            )
            in_test_module = any(lo <= literal.line <= hi for lo, hi in ranges)
            bucket = classify(
                literal, context, path, in_test_module, test_file, lookback
            )
            findings.append(
                {
                    "path": display_path(path),
                    "module": module_of(path, root),
                    "line": literal.line,
                    "value": literal.value,
                    "bucket": bucket,
                    "context": context.strip(),
                }
            )
    return findings


def print_report(findings, sample_size, seed, only):
    buckets = Counter(f["bucket"] for f in findings)
    total = len(findings)
    print(f"string literals extracted : {total}")
    print(
        f"candidate (user-visible)  : {buckets['candidates']} "
        f"({buckets['candidates'] * 100 / total:.1f}%)"
    )
    print(f"internal                  : {total - buckets['candidates']}")
    print()
    print("internal breakdown:")
    for bucket, count in buckets.most_common():
        if bucket == "candidates":
            continue
        print(f"  {bucket.removeprefix('internal:'):<12} {count:>7}")
    print()
    by_module = defaultdict(Counter)
    for f in findings:
        by_module[f["module"]][f["bucket"]] += 1
    print(f"{'module':<28}{'candidates':>12}{'internal':>10}{'total':>9}")
    for module in sorted(by_module, key=lambda m: -sum(by_module[m].values())):
        counts = by_module[module]
        cand = counts["candidates"]
        tot = sum(counts.values())
        print(f"{module:<28}{cand:>12}{tot - cand:>10}{tot:>9}")
    print()
    if not sample_size:
        return
    rng = random.Random(seed)
    for bucket in ("candidates", "internal:short", "internal:identifier"):
        if only and only != bucket.replace("internal:", ""):
            continue
        pool = [f for f in findings if f["bucket"] == bucket]
        if not pool:
            continue
        picked = rng.sample(pool, min(sample_size, len(pool)))
        print(f"===== sample: {bucket} (n={len(picked)}, seed={seed}) =====")
        for f in sorted(picked, key=lambda f: (f["path"], f["line"])):
            print(f"{f['path']}:{f['line']}")
            print(f"    value   : {f['value']!r}")
            print(f"    context : {f['context'][:150]}")
        print()


def main(argv):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=DEFAULT_ROOT)
    parser.add_argument("--sample", type=int, default=0)
    parser.add_argument("--seed", type=int, default=20260916)
    parser.add_argument("--only", default="", help="candidates | short | identifier")
    args = parser.parse_args(argv)
    if not args.root.is_dir():
        print(f"error: {args.root} is not a directory", file=sys.stderr)
        return 1
    print_report(scan(args.root), args.sample, args.seed, args.only)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
