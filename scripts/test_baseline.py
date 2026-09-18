#!/usr/bin/env python3
"""Maintain the success/failure sets of the local test scopes, by hash.

Why this exists
---------------
"This suite failed with 2 tests" is not reproducible information: the next run
may differ (order-dependent failures, machine state), and nobody can tell
whether a failure is *new* or was already there before the change under review.
The cost of that ambiguity is real -- attributing a pre-existing failure to your
own change wastes a full test round, and the reverse (wave through a real
regression as "probably flaky") is worse.

So each scope records three things and identifies them by hash:

  * `pass_hash`  -- sha1 of the sorted names of the tests that passed;
  * `fail`       -- the sorted names of the tests that failed (the failure set);
  * `state_hash` -- sha1 of `command + pass_hash + fail_hash`, i.e. the identity
    of "this scope, in this state". Quote the hash in a report instead of
    paraphrasing "the suite was green".

`--check` treats exactly one thing as a regression: a test that fails now and is
neither in the recorded failure set nor declared `flaky`. Everything else is
reported but tolerated -- new tests appearing, a count changing, a recorded
failure that now passes (report it, then `--update`).

Usage
-----
    python3 scripts/test_baseline.py --list
    python3 scripts/test_baseline.py --scope core-session --check
    python3 scripts/test_baseline.py --scope core-session --update
    python3 scripts/test_baseline.py --all --check

Exit status: 0 when no new failure; 1 on a new failure (or on `--check` for a
scope that has no recorded baseline yet -- record one first with `--update`).
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
DEFAULT_BASELINE = REPO / "scripts" / "test-baseline.json"

# The scopes runners keep re-running. `cwd` is relative to the repository root.
# `filter` is the single cargo test filter (cargo accepts one).
SCOPES: dict[str, dict[str, str]] = {
    "core-session": {"cwd": "codex-rs", "pkg": "codex-core", "filter": "session"},
    "core-unified-exec": {
        "cwd": "codex-rs",
        "pkg": "codex-core",
        "filter": "unified_exec",
    },
    "core-stdin-approval": {
        "cwd": "codex-rs",
        "pkg": "codex-core",
        "filter": "stdin_approval",
    },
    "core-guardian": {"cwd": "codex-rs", "pkg": "codex-core", "filter": "guardian"},
    "core-approvals": {"cwd": "codex-rs", "pkg": "codex-core", "filter": "approvals"},
    "core-thread-manager": {
        "cwd": "codex-rs",
        "pkg": "codex-core",
        "filter": "thread_manager",
    },
    "core-tools-handlers": {
        "cwd": "codex-rs",
        "pkg": "codex-core",
        "filter": "handlers",
    },
    "core-exec": {"cwd": "codex-rs", "pkg": "codex-core", "filter": "exec"},
}

TEST_LINE = re.compile(r"^test (\S+) \.\.\. (ok|FAILED|ignored)$")
SUMMARY = re.compile(r"^test result: (\w+)\. (\d+) passed; (\d+) failed; (\d+) ignored")


def command_for(spec: dict[str, str]) -> str:
    """The exact command a scope runs, with the stack size the repo requires."""
    return "RUST_MIN_STACK=16777216 cargo test -p {pkg} --lib {filter}".format(
        pkg=spec["pkg"], filter=spec["filter"]
    )


def sha1(text: str) -> str:
    return hashlib.sha1(text.encode("utf-8")).hexdigest()


def state_hash(command: str, pass_hash: str, fail: list[str]) -> str:
    return sha1(command + "|" + pass_hash + "|" + ",".join(sorted(fail)))


def run_scope(name: str, spec: dict[str, str]) -> dict:
    """Run one scope and return its observed sets and hashes."""
    command = command_for(spec)
    env = dict(os.environ)
    env.setdefault("RUST_MIN_STACK", "16777216")
    proc = subprocess.run(
        command,
        shell=True,
        cwd=REPO / spec["cwd"],
        capture_output=True,
        text=True,
        env=env,
        check=False,
    )
    output = proc.stdout + proc.stderr
    passed, failed = [], []
    for line in output.splitlines():
        match = TEST_LINE.match(line.strip())
        if not match:
            continue
        if match.group(2) == "ok":
            passed.append(match.group(1))
        elif match.group(2) == "FAILED":
            failed.append(match.group(1))
    counts = None
    for line in output.splitlines():
        match = SUMMARY.match(line.strip())
        if match:
            counts = {
                "passed": int(match.group(2)),
                "failed": int(match.group(3)),
                "ignored": int(match.group(4)),
            }
    pass_hash = sha1("\n".join(sorted(passed)))
    return {
        "command": command,
        "exit_code": proc.returncode,
        "counts": counts,
        "pass_hash": pass_hash,
        "pass_count": len(passed),
        "fail": sorted(failed),
        "fail_hash": sha1("\n".join(sorted(failed))),
        "state_hash": state_hash(command, pass_hash, sorted(failed)),
        "tail": "\n".join(output.strip().splitlines()[-4:]),
    }


def load(path: Path) -> dict:
    if not path.exists():
        return {}
    return json.loads(path.read_text(encoding="utf-8"))


def save(path: Path, data: dict) -> None:
    path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def report(
    name: str, observed: dict, recorded: dict | None, updating: bool = False
) -> bool:
    """Print the comparison; return True when the scope is a regression."""
    counts = observed["counts"] or {}
    print(
        f"[{name}] exit={observed['exit_code']} "
        f"passed={counts.get('passed', '?')} failed={counts.get('failed', '?')}"
    )
    print(f"    pass_hash  {observed['pass_hash']}")
    print(f"    fail_hash  {observed['fail_hash']}")
    print(f"    state_hash {observed['state_hash']}")
    if updating:
        print("    recording this run as the new baseline")
        if recorded is not None:
            print(f"    previous baseline {recorded.get('state_hash', '(none)')}")
        return False
    if recorded is None:
        print("    no baseline recorded -- run --update to record one")
        return observed["exit_code"] != 0
    flaky = set(recorded.get("flaky", []))
    new = [t for t in observed["fail"] if t not in recorded["fail"] and t not in flaky]
    flaked = [t for t in observed["fail"] if t in flaky]
    gone = [t for t in recorded["fail"] if t not in observed["fail"]]
    print(f"    baseline   {recorded.get('state_hash', '(none)')}")
    if flaked:
        notes = recorded.get("flaky_notes") or {}
        for name in flaked:
            note = notes.get(name)
            print(
                f"    known flaky, tolerated: {name}"
                + (f" -- {note}" if note else " -- (no note recorded)")
            )
    if gone:
        print(
            f"    was failing, now passing (re-record with --update): {', '.join(gone)}"
        )
    if new:
        print(f"    NEW FAILURES: {', '.join(new)}")
    if observed["pass_count"] != recorded.get("pass_count"):
        print(
            f"    pass set changed: {recorded.get('pass_count')} -> {observed['pass_count']}"
        )
        print(f"    (new pass_hash {observed['pass_hash']})")
    if not new and not gone and observed["pass_hash"] == recorded.get("pass_hash"):
        print("    identical success set and failure set")
    return bool(new)


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--scope", help="scope name (see --list)")
    parser.add_argument("--all", action="store_true", help="every scope in SCOPES")
    parser.add_argument(
        "--update", action="store_true", help="record the observed sets"
    )
    parser.add_argument("--check", action="store_true", help="compare (default action)")
    parser.add_argument("--baseline", type=Path, default=DEFAULT_BASELINE)
    parser.add_argument(
        "--list", action="store_true", help="list scopes and recorded hashes"
    )
    args = parser.parse_args(argv)

    if args.list:
        data = load(args.baseline)
        for name, spec in SCOPES.items():
            rec = data.get(name)
            hashed = (
                rec.get("state_hash", "(not recorded)") if rec else "(not recorded)"
            )
            print(f"{name:<22} {command_for(spec):<58} {hashed}")
        return 0

    names = list(SCOPES) if args.all else ([args.scope] if args.scope else [])
    if not names:
        parser.error("pass --scope <name> or --all (see --list)")
    for name in names:
        if name not in SCOPES:
            parser.error(f"unknown scope {name!r} (see --list)")

    data = load(args.baseline)
    regressions = []
    for name in names:
        observed = run_scope(name, SCOPES[name])
        recorded = data.get(name)
        is_regression = report(name, observed, recorded, updating=args.update)
        if is_regression and not args.update:
            regressions.append(name)
        if args.update:
            entry = dict(observed)
            entry.pop("tail", None)
            entry["flaky"] = (recorded or {}).get("flaky", [])
            data[name] = entry
    if args.update:
        save(args.baseline, data)
        print(f"\nrecorded {len(names)} scope(s) -> {args.baseline}")
    if regressions:
        print(f"\nREGRESSION in: {', '.join(regressions)}")
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
