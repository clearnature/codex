#!/usr/bin/env bash
# i18n 平台受限站点的类型级复核脚本（docs/plan/i18n-platform-ci-checklist.md）。
#
# 43 处受控站点全部位于 codex-cli（debug_sandbox.rs、desktop_app/mac.rs、
# desktop_app/windows.rs），因此对各目标三元组跑 `cargo clippy -p codex-cli`
# 即覆盖全部站点；无需上游企业级 CI 组件，任何装了 rustup 的 macOS/
# Windows 主机或 GitHub 标准 runner（macos-15 / windows-2022）可直接执行。
#
# 用法：scripts/i18n-macos-verify.sh [target ...]
#   缺省校验两个 Apple target；Windows 站点传 x86_64-pc-windows-msvc（需在
#   Windows 主机上跑，MSVC 不能从 macOS 交叉编译）。

set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)/codex-rs"
cd "$root"

targets=("$@")
if [ ${#targets[@]} -eq 0 ]; then
  targets=(aarch64-apple-darwin x86_64-apple-darwin)
fi

for target in "${targets[@]}"; do
  echo "==> rustup target add $target"
  rustup target add "$target"
  echo "==> cargo clippy -p codex-cli --all-targets --target $target -- -D warnings"
  cargo clippy -p codex-cli --all-targets --target "$target" -- -D warnings
done

echo "OK: 平台受限站点类型复核通过（target: ${targets[*]}）"
