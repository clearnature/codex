#!/usr/bin/env python3
"""Apply one i18n batch from a JSON spec: rewrite call sites, append dictionary
entries, and register exemptions -- with every match asserted before any write.

Why this exists
---------------
Every batch used to re-implement the same fragile rewriting logic by hand, and
the same three classes of mistake recurred (see
`docs/plan/i18n-verification.md` §12.58/§12.60 and journals `j-mu7yxybn-crgn` /
`j-mu80hswy-bbol`):

1. patterns written against the *pre-rustfmt* shape of a call, which no longer
   match once rustfmt expands it;
2. hand-typed dictionary keys that drift from the source literal (truncated
   values, kept repr quotes, `#`-prefixed rows read as comments);
3. argument shapes guessed instead of read (`&x` on an `&str` is a
   `clippy::needless-borrow` *error* in this repo, not a warning).

So this tool always derives the literal, the key and the argument count **from
the file on disk**, asserts every match (`count`, line number, placeholder
count) before writing anything, and never writes partially: `--plan` (default)
is a dry run, `--apply` writes.

Spec (`--spec <file.json>`)
---------------------------
    {
      "file": "codex-rs/cli/src/login.rs",
      "root": "codex-rs/cli/src",              // scan root for `i18n_todo.py`
      "translate": {
        "<line>": {"kind": "println|eprintln|bail|anyhow|ensure|context|format|to_string|arg",
                   "args": ["<rust expr yielding &str>", ...],   // one per placeholder
                   "zh": "<translation>"}                         // `null` => fill it in later
      },
      "extra_edits": [{"find": "...", "replace": "...", "count": 1, "note": "..."}],
      "register": {"<line>": "<why this site is not translated>"}
    }

Kinds
-----
* `println` / `eprintln` -> `name!("{}", <tr …>)`
* `bail` / `anyhow`      -> `name!(<tr …>)`
* `ensure`               -> `ensure!(cond, <tr …>)`  (first argument kept)
* `context`              -> `.context(<tr …>)`
* `format`               -> `<tr …>` (the wrapper is dropped: `format!` around a
                            single `String` trips `clippy::useless_format`)
* `to_string`            -> `"LIT".to_string()` becomes `tr(…).to_string()`
* `arg`                  -> the bare literal becomes `tr(current(), "LIT")`
                            (for literals passed *into* a rendering helper)

Follow-up (the tool prints this too): `just fmt` in `codex-rs`, then
`python3 scripts/i18n_dossier_lines.py --fix`, then re-check that every
registration row's site still holds its literal, then the gates.
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
DICT = REPO / "codex-rs/i18n/src/dict_zh.rs"
TSV = REPO / "codex-rs/i18n/not-translated-unwrapped.tsv"
IMPORT_TEMPLATE = REPO / "codex-rs/cli/src/state_db_recovery.rs"


def import_name(line: str) -> str:
    """`use codex_i18n::tr_with;` -> `tr_with`。"""
    return line.rstrip().rstrip(";").rsplit("::", 1)[-1]


def _used_imports(spec: dict) -> set[str]:
    """只插入实际用得到的 import —— 无条件插入会产生 unused_imports 告警（实测两次）。"""
    used = {"current"}
    for entry in spec.get("translate", {}).values():
        used.add("tr_with" if entry.get("args") else "tr")
    for e in spec.get("extra_edits", []):
        body = e.get("replace", "")
        if "tr_with(" in body:
            used.add("tr_with")
        if "tr(" in body or "tr_with(" in body:
            used.add("tr")
    return used


def read_lines(path: Path) -> list[str]:
    return path.read_text(encoding="utf-8").splitlines()


def literal_at(lines: list[str], line_no: int) -> tuple[int, int, str]:
    """Column span (start, end) of the first string literal on `line_no`."""
    ln = lines[line_no - 1]
    i = ln.find('"')
    if i < 0:
        raise AssertionError(f":{line_no} 行内没有字符串字面量")
    j = i + 1
    while j < len(ln):
        if ln[j] == "\\":
            j += 2
            continue
        if ln[j] == '"':
            break
        j += 1
    if j >= len(ln):
        raise AssertionError(f":{line_no} 的字面量未闭合")
    return i, j, ln[i + 1 : j]


def abs_pos(lines: list[str], line_no: int, col: int) -> int:
    return sum(len(x) + 1 for x in lines[: line_no - 1]) + col


def find_call_span(text: str, lit_start: int) -> tuple[int, int]:
    """Enclosing call parentheses around `lit_start`, string-aware."""
    depth = 0
    k = lit_start - 1
    while k >= 0:
        c = text[k]
        if c == ")":
            depth += 1
        elif c == "(":
            if depth == 0:
                open_paren = k
                break
            depth -= 1
        k -= 1
    else:
        raise AssertionError("找不到包围的调用括号")
    depth = 0
    m = open_paren
    while m < len(text):
        c = text[m]
        if c == '"':
            m += 1
            while m < len(text) and text[m] != '"':
                m += 2 if text[m] == "\\" else 1
        elif c == "(":
            depth += 1
        elif c == ")":
            depth -= 1
            if depth == 0:
                return open_paren, m
        m += 1
    raise AssertionError("括号未闭合")


def callee_of(text: str, open_paren: int) -> tuple[str, str]:
    k = open_paren - 1
    while k >= 0 and text[k].isspace():
        k -= 1
    if k >= 0 and text[k] == "!":
        j = k - 1
        while j >= 0 and (text[j].isalnum() or text[j] == "_"):
            j -= 1
        return text[j + 1 : k], "macro"
    j = k
    while j >= 0 and (text[j].isalnum() or text[j] == "_"):
        j -= 1
    return text[j + 1 : k + 1], "method"


def split_top(text: str) -> list[str]:
    out: list[str] = []
    depth = 0
    cur = ""
    i = 0
    while i < len(text):
        c = text[i]
        if c == '"':
            cur += c
            i += 1
            while i < len(text) and text[i] != '"':
                cur += text[i]
                i += 1
            cur += '"'
        elif c in "([{":
            depth += 1
            cur += c
        elif c in ")]}":
            depth -= 1
            cur += c
        elif c == "," and depth == 0:
            out.append(cur.strip())
            cur = ""
        else:
            cur += c
        i += 1
    if cur.strip():
        out.append(cur.strip())
    return out


def plan(spec: dict) -> tuple[list[tuple[int, int, str, str]], dict]:
    src = REPO / spec["file"]
    text = src.read_text(encoding="utf-8")
    lines = text.splitlines()
    edits: list[tuple[int, int, str, str]] = []
    keys: dict[str, str] = {}
    notes: list[str] = []

    for line_s, entry in sorted(
        spec.get("translate", {}).items(), key=lambda kv: int(kv[0])
    ):
        line_no = int(line_s)
        kind = entry["kind"]
        exprs = entry.get("args", [])
        zh = entry.get("zh")
        i, j, lit = literal_at(lines, line_no)
        ai, aj = abs_pos(lines, line_no, i), abs_pos(lines, line_no, j)
        ph = re.findall(r"\{([A-Za-z_][A-Za-z0-9_]*)?\}", lit)
        if len(ph) != len(exprs):
            raise AssertionError(
                f":{line_no} 占位符 {len(ph)} 个 vs 给的表表达式 {len(exprs)} 个：{lit[:70]!r}"
            )
        counter = [0]

        def repl(_m: re.Match) -> str:
            n = counter[0]
            counter[0] += 1
            return "{%d}" % n

        key = re.sub(r"\{[A-Za-z_][A-Za-z0-9_]*\}", repl, lit)
        key = re.sub(r"\{\}", repl, key)
        assert counter[0] == len(ph), f":{line_no} 占位符替换计数不一致"
        if key in keys and zh is not None and keys[key] is not None and keys[key] != zh:
            raise AssertionError(f":{line_no} 同键译文不一致：{keys[key]!r} vs {zh!r}")
        keys.setdefault(key, zh)

        tr_expr = (
            f'tr_with(current(), "{key}", &[{", ".join(exprs)}])'
            if exprs
            else f'tr(current(), "{key}")'
        )
        if kind == "to_string":
            seg_end = text.find(".to_string()", aj)
            if seg_end != aj + 1:
                raise AssertionError(f':{line_no} 期望 `"LIT".to_string()`')
            edits.append(
                (
                    ai,
                    seg_end + len(".to_string()"),
                    f"{tr_expr}.to_string()",
                    f":{line_no}",
                )
            )
            continue
        if kind == "arg":
            edits.append((ai, aj + 1, tr_expr, f":{line_no} arg"))
            continue
        open_paren, close_paren = find_call_span(text, ai)
        callee, ckind = callee_of(text, open_paren)
        if ckind == "macro":
            args = split_top(text[open_paren + 1 : close_paren])
            if callee in ("println", "eprintln"):
                if kind != callee:
                    raise AssertionError(
                        f":{line_no} kind={kind} 与调用 {callee}! 不符"
                    )
                new = f'{callee}!("{{}}", {tr_expr})'
            elif kind in ("bail", "anyhow"):
                if callee != kind:
                    raise AssertionError(
                        f":{line_no} kind={kind} 与调用 {callee}! 不符"
                    )
                new = f"{callee}!({tr_expr})"
            elif kind == "ensure":
                if callee != "ensure" or len(args) != 2:
                    raise AssertionError(f":{line_no} ensure! 形态不符：{args}")
                new = f"ensure!({args[0]}, {tr_expr})"
            elif kind == "format":
                if callee != "format":
                    raise AssertionError(
                        f":{line_no} kind=format 与调用 {callee}! 不符"
                    )
                new = tr_expr
            else:
                raise AssertionError(f":{line_no} 未知 kind={kind} 在宏 {callee}!")
        else:
            if callee != "context" or kind != "context":
                raise AssertionError(f":{line_no} 期望 .context(…)，实际 .{callee}(…)")
            new = f".context({tr_expr})"
        start = open_paren - len(callee) - 1
        edits.append((start, close_paren + 1, new, f":{line_no}"))

    for e in spec.get("extra_edits", []):
        count = text.count(e["find"])
        if count != e.get("count", 1):
            raise AssertionError(
                f"extra_edit 命中 {count} 次（期望 {e.get('count', 1)}）：{e['find'][:60]!r}"
            )
        pos = 0
        for _ in range(count):
            a = text.index(e["find"], pos)
            edits.append(
                (a, a + len(e["find"]), e["replace"], f"extra:{e.get('note', '')[:24]}")
            )
            pos = a + 1

    for k, v in spec.get("extra_dict", {}).items():
        if k in keys and keys[k] is not None and keys[k] != v:
            raise AssertionError(f"extra_dict 与 translate 的译文冲突：{k!r}")
        keys.setdefault(k, v)

    edits.sort()
    for a, b, _, _ in edits:
        if a >= b:
            raise AssertionError("编辑区间非法")
    for (a1, b1, _, l1), (a2, b2, _, l2) in zip(edits, edits[1:]):
        if b1 > a2:
            raise AssertionError(f"编辑区间重叠：{l1} 与 {l2}")
    spans = [
        (text[:a].count("\n") + 1, text[:b].count("\n") + 1) for a, b, _, _ in edits
    ]
    for entry in spec.get("translate", {}):
        ln = int(entry)
        if not any(s <= ln <= e for s, e in spans):
            raise AssertionError(f":{ln} 不在任何编辑区间内")
    return edits, {"text": text, "keys": keys, "notes": notes, "spans": spans}


def dump_rows(spec: dict) -> dict[int, str]:
    out = subprocess.run(
        [
            "python3",
            "scripts/i18n_todo.py",
            "--root",
            spec["root"],
            "--file",
            spec["file"].split("/", 1)[1],
            "--dump-rows",
        ],
        cwd=REPO,
        capture_output=True,
        text=True,
    )
    rows: dict[int, str] = {}
    for row in out.stdout.splitlines():
        parts = row.split("\t")
        site = next((p for p in parts if ".rs:" in p), None)
        if site:
            rows[int(site.rsplit(":", 1)[1])] = parts[0]
    return rows


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--spec")
    ap.add_argument(
        "--specs", help='批量：JSON 文件里放 {"batches": [<spec>, ...]}，逐个跑 --spec'
    )
    ap.add_argument("--apply", action="store_true", help="写入（默认只做 plan）")
    args = ap.parse_args()
    if args.specs:
        batches = json.loads(Path(args.specs).read_text(encoding="utf-8"))["batches"]
        rc = 0
        for i, one in enumerate(batches):
            tmp = f"/tmp/i18n_batch_{i}.json"
            Path(tmp).write_text(json.dumps(one, ensure_ascii=False), encoding="utf-8")
            cmd = [sys.executable, __file__, "--spec", tmp] + (
                ["--apply"] if args.apply else []
            )
            print(f"---- batch {i + 1}/{len(batches)}: {one.get('file')}")
            rc |= subprocess.call(cmd)
        return rc
    if not args.spec:
        ap.error("需要 --spec 或 --specs")
    spec = json.loads(Path(args.spec).read_text(encoding="utf-8"))

    edits, meta = plan(spec)
    text = meta["text"]
    keys: dict[str, str] = meta["keys"]
    dt = DICT.read_text(encoding="utf-8")
    new_keys = {k: v for k, v in keys.items() if f'("{k}"' not in dt}
    existing = sorted(k for k in keys if k not in new_keys)
    rows_now = dump_rows(spec)
    reg_missing = [ln for ln in spec.get("register", {}) if int(ln) not in rows_now]
    unfilled = sorted(k for k, v in keys.items() if v is None)

    print(
        f"== {spec['file']}：{len(spec.get('translate', {}))} 个翻译站点 / "
        f"{len(spec.get('extra_edits', []))} 处额外编辑 / {len(spec.get('register', {}))} 条登记"
    )
    for a, b, new, label in edits:
        s, e = text[:a].count("\n") + 1, text[:b].count("\n") + 1
        print(f"{label:>26} | 行 {s}-{e} | {new[:92]}")
    print(
        f"新词条 {len(new_keys)} 条；字典已有（只包站点、不新增）{len(existing)} 条：{[k[:44] for k in existing]}"
    )
    if unfilled:
        print(f"⚠ 缺译文（填 zh 后再 --apply）：{unfilled}")
    if reg_missing:
        print(f"⚠ 登记站点不在候选表里（会被 --audit-rows 当无效）：{reg_missing}")
    print("将按需插入的 import：", sorted(_used_imports(spec)))
    if not args.apply:
        print("（plan 模式，未写入）")
        return 0

    if unfilled:
        print("拒绝写入：仍有缺译文的键")
        return 2
    new_text = text
    for a, b, new, _ in sorted(edits, reverse=True):
        new_text = new_text[:a] + new + new_text[b:]
    imp = [
        line
        for line in read_lines(IMPORT_TEMPLATE)
        if line.startswith("use codex_i18n")
        and import_name(line) in _used_imports(spec)
    ]
    if imp and "use codex_i18n" not in new_text:
        lines = new_text.splitlines()
        idx = next(
            (
                n
                for n, line in enumerate(lines)
                if line.startswith("use ") and line > "use codex_i18n::"
            ),
            None,
        )
        if idx is None:
            raise AssertionError("找不到 import 插入点")
        lines[idx:idx] = imp
        new_text = "\n".join(lines) + "\n"
        print("已插入 import：", imp)
    (REPO / spec["file"]).write_text(new_text, encoding="utf-8")

    entries = "".join(
        '    (\n        "%s",\n        "%s",\n    ),\n'
        % (
            k,  # 键取自源码字面量本身，转义已正确，不能再转义一遍
            v.replace("\\", "\\\\").replace("\n", "\\n").replace('"', '\\"'),
        )
        for k, v in new_keys.items()
    )
    m = re.search(r"\n\];\n", dt)
    if not m:
        raise AssertionError("找不到 ENTRIES 闭合行")
    DICT.write_text(
        dt[: m.start()] + "\n" + entries.rstrip("\n") + "\n];\n" + dt[m.end() :],
        encoding="utf-8",
    )

    tsv = TSV.read_text(encoding="utf-8")
    if not tsv.endswith("\n"):
        tsv += "\n"
    added = 0
    for ln_s, reason in sorted(
        spec.get("register", {}).items(), key=lambda kv: int(kv[0])
    ):
        ln = int(ln_s)
        site = f"{spec['file']}:{ln}"
        if f"\t{site}\t" in tsv:
            print("登记行已存在，跳过：", site)
            continue
        tsv += f"{rows_now[ln]}\t{site}\t{reason}\n"
        added += 1
    TSV.write_text(tsv, encoding="utf-8")
    print(f"已写入：{len(edits)} 处编辑 / {len(new_keys)} 条词条 / {added} 条登记行")
    print(
        "下一步：cd codex-rs && just fmt；然后 python3 scripts/i18n_dossier_lines.py --fix，"
        "再逐条核对登记行命中源码，最后跑 fmt-check / i18n-check / clippy + 合并自检"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
