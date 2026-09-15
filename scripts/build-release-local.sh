#!/usr/bin/env bash
#
# Official release build, run locally, with a real version baked into the
# binaries.
#
# This solves two problems.
#
# 1. rusty_v8
#    Plain `cargo build` reaches `v8 v150.4.0` with `v8_enable_sandbox`
#    enabled (via code-mode-runtime) and tries to fetch a prebuilt archive
#    that denoland never published for that profile, so the download 404s.
#    Codex's CI fixes this with .github/actions/setup-rusty-v8; we reuse
#    scripts/setup-rusty-v8.sh for the same effect.
#
# 2. version number
#    Upstream pins `[workspace.package].version` to the literal "0.0.0" on
#    main, and writes the real value only inside the one-off commit that a
#    `rust-vX.Y.Z` tag points at. So anything built from a branch reports
#    `codex-cli 0.0.0`.
#
#    That value is not cosmetic. It flows through
#    `env!("CARGO_PKG_VERSION")` (101 call sites) into the `codex_version`
#    turn metadata sent to the server, the HTTP User-Agent, telemetry, and
#    the TUI.
#
#    Cargo offers no way to override `CARGO_PKG_VERSION` at build time, so
#    the manifest is the only lever. This script writes the real version into
#    the manifest for the duration of the build and restores the original
#    bytes on exit -- including Ctrl-C / SIGTERM -- so the produced binaries
#    carry the real version while the worktree ends up untouched.
#
# Usage:
#   scripts/build-release-local.sh                        # host target
#   scripts/build-release-local.sh <rust-target-triple>
#   scripts/build-release-local.sh --version 0.154.0
#   CODEX_BUILD_VERSION=0.154.0 scripts/build-release-local.sh
#
# Version resolution order: --version / CODEX_BUILD_VERSION, otherwise the
# nearest reachable stable `rust-vX.Y.Z` tag.
#
# Caveat: the release pipeline ships MUSL-linked Linux binaries. Those need
# Zig plus .github/scripts/install-musl-build-tools.sh; this script checks
# for them up front instead of failing late.

set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
self_path="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/$(basename "${BASH_SOURCE[0]}")"
cd "$repo_root/codex-rs"

log() { printf '\n== %s ==\n' "$*" >&2; }

requested_version=""
target=""
while [[ $# -gt 0 ]]; do
    case "$1" in
        --version) requested_version="${2:-}"; shift 2 ;;
        --version=*) requested_version="${1#--version=}"; shift ;;
        -h | --help)
            sed -n '2,40p' "$self_path" | sed -e 's/^# \{0,1\}//' >&2
            exit 0
            ;;
        -*) log "error: unknown flag $1"; exit 2 ;;
        *) target="$1"; shift ;;
    esac
done

target="${target:-$(rustc -vV | awk '/^host:/ { print $2 }')}"
binaries=(codex codex-code-mode-host codex-responses-api-proxy)

case "$target" in
    *-unknown-linux-musl)
        missing_musl=()
        command -v zig >/dev/null 2>&1 || missing_musl+=(zig)
        if ! command -v musl-gcc >/dev/null 2>&1 &&
            ! command -v x86_64-linux-musl-gcc >/dev/null 2>&1; then
            missing_musl+=(musl-gcc)
        fi
        if ((${#missing_musl[@]})); then
            printf 'error: %s needs a musl cross toolchain; missing: %s\n' \
                "$target" "${missing_musl[*]}" >&2
            printf '       run: scripts/setup-musl-toolchain.sh %s\n' "$target" >&2
            exit 1
        fi
        export AWS_LC_SYS_NO_JITTER_ENTROPY=1
        ;;
    *-unknown-linux-gnu)
        if ! pkg-config --exists libcap 2>/dev/null; then
            printf 'warning: libcap dev headers not found; bwrap may fail to build.\n' >&2
            printf '         (Debian/Ubuntu: sudo apt-get install -y libcap-dev)\n' >&2
        fi
        ;;
esac

# ---------------------------------------------------------------------------
# Resolve the version to bake in.
# ---------------------------------------------------------------------------

if [[ -z "$requested_version" ]]; then
    requested_version="${CODEX_BUILD_VERSION:-}"
fi

if [[ -z "$requested_version" ]]; then
    if tag="$(git -C "$repo_root" describe --tags --match 'rust-v[0-9]*' --abbrev=0 2>/dev/null)"; then
        candidate="${tag#rust-v}"
        if [[ "$candidate" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
            requested_version="$candidate"
            log "derived version $requested_version from tag $tag"
        else
            log "note: nearest tag $tag is not a stable rust-vX.Y.Z release"
        fi
    fi
fi

if [[ -n "$requested_version" && ! "$requested_version" =~ ^[0-9]+\.[0-9]+\.[0-9]+([-+].*)?$ ]]; then
    log "error: '$requested_version' is not a valid semver version"
    exit 2
fi

cargo_toml="$repo_root/codex-rs/Cargo.toml"
cargo_lock="$repo_root/codex-rs/Cargo.lock"
backup_dir="$(mktemp -d)"
restored=0

# Only one run may own the manifest at a time. Without this, a second
# concurrent run snapshots the first run's baked version as its "original"
# and restores that, leaving the worktree dirty. mkdir is atomic on every
# POSIX platform (flock is not portable); the lock is released by the EXIT
# trap below, so an interrupted run does not leave it behind.
lock_dir="$repo_root/codex-rs/target/.codex-build-release-local.lock"
mkdir -p "$repo_root/codex-rs/target"
if ! mkdir "$lock_dir" 2>/dev/null; then
    log "error: another build-release-local.sh run is in progress"
    log "       if that is wrong (stale lock), remove: $lock_dir"
    rm -rf "$backup_dir"
    exit 1
fi

restore_manifest() {
    [[ "$restored" == "1" ]] && return 0
    restored=1
    cp "$backup_dir/Cargo.toml" "$cargo_toml"
    [[ -f "$backup_dir/Cargo.lock" ]] && cp "$backup_dir/Cargo.lock" "$cargo_lock"
    rm -rf "$backup_dir"
    rmdir "$lock_dir" 2>/dev/null || true
    log "restored Cargo.toml / Cargo.lock (worktree untouched)"
}
# Exit codes must still tell the truth when interrupted, hence the explicit
# 130 (SIGINT) / 143 (SIGTERM) instead of letting bash continue.
trap restore_manifest EXIT
trap 'restore_manifest; exit 130' INT
trap 'restore_manifest; exit 143' TERM

cp "$cargo_toml" "$backup_dir/Cargo.toml"
[[ -f "$cargo_lock" ]] && cp "$cargo_lock" "$backup_dir/Cargo.lock"

current_version="$(grep -m1 '^version' "$cargo_toml" | sed -E 's/version *= *"([^"]+)".*/\1/')"

if [[ -n "$requested_version" && "$requested_version" != "$current_version" ]]; then
    log "baking version $current_version -> $requested_version for this build"
    python3 - "$cargo_toml" "$requested_version" <<'PY'
import pathlib, re, sys

path = pathlib.Path(sys.argv[1])
version = sys.argv[2]
lines = path.read_text(encoding="utf-8").splitlines(keepends=True)

in_section = False
for index, line in enumerate(lines):
    stripped = line.strip()
    if stripped == "[workspace.package]":
        in_section = True
        continue
    if in_section and stripped.startswith("["):
        break
    if in_section and re.match(r"^version\s*=", stripped):
        indent = line[: len(line) - len(line.lstrip())]
        newline = "\n" if line.endswith("\n") else ""
        lines[index] = f'{indent}version = "{version}"{newline}'
        path.write_text("".join(lines), encoding="utf-8")
        sys.exit(0)

sys.exit(f"could not find [workspace.package].version in {path}")
PY
    (cd "$repo_root/codex-rs" && cargo update --workspace --quiet)
else
    log "version already $current_version; no manifest rewrite needed"
fi

# Release builds resolve git deps through the git CLI, like CI.
export CARGO_NET_GIT_FETCH_WITH_CLI=true
export STABLE_GIT_COMMIT="$(git -C "$repo_root" rev-parse HEAD)"

log "step 1/3: configure rusty_v8 artifact overrides"
eval "$("$repo_root/scripts/setup-rusty-v8.sh" "$target")"
log "RUSTY_V8_ARCHIVE=$RUSTY_V8_ARCHIVE"

log "step 2/3: build bwrap, strip it, export CODEX_BWRAP_SHA256"
cargo build --target "$target" --release --bin bwrap

bwrap_path="target/$target/release/bwrap"
[[ "$target" == *-pc-windows-msvc ]] && bwrap_path="$bwrap_path.exe"
if [[ ! -f "$bwrap_path" ]]; then
    log "error: bwrap binary not found at $bwrap_path"
    exit 1
fi

# The digest covers the exact bytes the release packages, so strip first.
strip --strip-debug --strip-unneeded "$bwrap_path"
export CODEX_BWRAP_SHA256="$(sha256sum "$bwrap_path" | awk '{print $1}')"
log "CODEX_BWRAP_SHA256=$CODEX_BWRAP_SHA256"
log "STABLE_GIT_COMMIT=$STABLE_GIT_COMMIT"

log "step 3/3: cargo build --release"
bin_args=()
for bin in "${binaries[@]}"; do
    bin_args+=(--bin "$bin")
done
cargo build --target "$target" --release "${bin_args[@]}"

log "artifacts under target/$target/release"
ls -la "target/$target/release/" | grep -E 'codex|bwrap' || true

# Prove the binary actually carries the version before restoring the manifest.
built_version="$("./target/$target/release/codex" --version 2>/dev/null || true)"
log "built binary reports: ${built_version:-<could not execute>}"
if [[ -n "$requested_version" && "$built_version" != *"$requested_version"* ]]; then
    log "error: expected the binary to report $requested_version"
    exit 1
fi
