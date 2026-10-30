#!/usr/bin/env python3
"""Generate `codex-rs/i18n/src/dict_zh_hant.rs` from `dict_zh.rs`.

The Traditional Chinese dictionary is a build-time derivative of the
Simplified one (the trha reference uses the same shape: one zh dictionary plus
a conversion step). OpenCC's `s2twp` profile is applied to every value, which
gives Taiwan-style vocabulary (設定/介面/伺服器) in line with the qwen zh-TW
reference the glossary already follows. Keys are copied verbatim, so the two
dictionaries are strict-parity by construction; `codex-i18n-check` enforces
that parity so editing `dict_zh.rs` without re-running this script fails CI.

Usage::

    python3 scripts/i18n_gen_hant.py          # regenerate dict_zh_hant.rs

Requires the `opencc-python-reimplemented` package (pure Python).
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

try:
    from opencc import OpenCC
except ImportError:
    print(
        "error: opencc-python-reimplemented is required: "
        "pip3 install --user --break-system-packages opencc-python-reimplemented",
        file=sys.stderr,
    )
    raise SystemExit(2)

REPO = Path(__file__).resolve().parent.parent
SRC = REPO / "codex-rs/i18n/src/dict_zh.rs"
DST = REPO / "codex-rs/i18n/src/dict_zh_hant.rs"

PAIR = re.compile(r'\(\s*"((?:[^"\\]|\\.)*)"\s*,\s*"((?:[^"\\]|\\.)*)"\s*,?\s*\)', re.S)

HEADER = """//! Traditional Chinese dictionary -- GENERATED from `dict_zh.rs` by
//! `scripts/i18n_gen_hant.py` (OpenCC `s2twp`, Taiwan vocabulary). Do not edit
//! by hand: keys must stay in strict parity with `dict_zh.rs`, which
//! `codex-i18n-check` enforces. Regenerate after any `dict_zh.rs` change.

use std::collections::HashMap;
use std::sync::LazyLock;

/// English source text -> Traditional Chinese (Taiwan vocabulary).
pub(crate) static ENTRIES: &[(&str, &str)] = &[
"""


def unescape(text: str) -> str:
    return (
        text.replace("\\'", "'")
        .replace('\\"', '"')
        .replace("\\n", "\n")
        .replace("\\\\", "\\")
    )


def escape(text: str) -> str:
    return text.replace("\\", "\\\\").replace('"', '\\"').replace("\n", "\\n")


def main() -> int:
    cc = OpenCC("s2twp")
    text = SRC.read_text("utf-8")
    body = text[text.index("static ENTRIES") :]
    # Strip `//` comment lines first: dict_zh.rs documents entry shapes in
    # comments with the same `("english", "中文")` shape, and matching them
    # would produce phantom keys (caught in practice by the parity check).
    body = "".join(
        line
        for line in body.splitlines(keepends=True)
        if not line.lstrip().startswith("//")
    )
    pairs = PAIR.findall(body)
    if not pairs:
        print("error: no entries found in dict_zh.rs", file=sys.stderr)
        return 2

    out = [HEADER]
    for key, value in pairs:
        hant = cc.convert(unescape(value))
        out.append(
            f'    (\n        "{escape(unescape(key))}",\n        "{escape(hant)}",\n    ),\n'
        )
    out.append("];\n")
    out.append(
        "\n/// English source text -> Traditional Chinese.\npub(crate) static DICT_ZH_HANT: "
        "LazyLock<HashMap<&'static str, &'static str>> =\n    LazyLock::new(|| ENTRIES.iter().copied().collect());\n"
    )
    DST.write_text("".join(out), "utf-8")
    print(f"dict_zh_hant.rs: {len(pairs)} entries written")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())


# NOTE: regenerate 后必须跑 `just fmt`（rustfmt 对齐 ENTRIES 排版），否则云端 A 类脚本套件的 formatter gate 会挂。
