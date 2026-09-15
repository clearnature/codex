#!/usr/bin/env bash
#
# Bootstrap the Codex-built rusty_v8 artifacts that local Cargo builds need.
#
# Plain `cargo build` in this workspace reaches `v8 v150.4.0` with
# `v8_enable_sandbox` enabled (via code-mode-runtime), and that build script
# fetches `<profile>_release` archives from denoland/rusty_v8. That variant is
# never published there, so the download 404s:
#
#   https://github.com/denoland/rusty_v8/releases/download/v150.4.0/
#     librusty_v8_ptrcomp_sandbox_release_x86_64-unknown-linux-gnu.a.gz  -> 404
#
# Codex's own CI works around this with .github/actions/setup-rusty-v8, which
# pulls the artifacts from the openai/codex `rusty-v8-v<version>` release and
# exports RUSTY_V8_ARCHIVE / RUSTY_V8_SRC_BINDING_PATH. This script does the
# same thing locally, with the same checksum verification.
#
# Usage:
#   eval "$(scripts/setup-rusty-v8.sh)"                 # host target
#   eval "$(scripts/setup-rusty-v8.sh)"; cargo build    # then build
#   eval "$(scripts/setup-rusty-v8.sh aarch64-unknown-linux-musl)"
#
# Diagnostics go to stderr so stdout stays safe to `eval`.

set -euo pipefail

target="${1:-$(rustc -vV | awk '/^host:/ { print $2 }')}"
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

version="$(python3 "$repo_root/.github/scripts/rusty_v8_bazel.py" resolved-v8-crate-version)"
profile="ptrcomp_sandbox_release"
cache_dir="${CODEX_V8_CACHE:-$HOME/.cache/codex-rusty-v8}/v$version"

if [[ "$target" == *-pc-windows-msvc ]]; then
    archive_name="rusty_v8_${profile}_${target}.lib.gz"
else
    archive_name="librusty_v8_${profile}_${target}.a.gz"
fi
binding_name="src_binding_${profile}_${target}.rs"
checksums_name="rusty_v8_${profile}_${target}.sha256"

base_url="https://github.com/openai/codex/releases/download/rusty-v8-v${version}"
trusted_manifests="$repo_root/third_party/v8/rusty_v8_${version//./_}_release_manifests.sha256"

log() { printf '%s\n' "$*" >&2; }

if [[ ! -f "$trusted_manifests" ]]; then
    log "error: missing trusted manifest: $trusted_manifests"
    exit 1
fi

mkdir -p "$cache_dir"

log "rusty_v8 ${version} / target ${target}"
log "cache: $cache_dir"

for name in "$checksums_name" "$archive_name" "$binding_name"; do
    if [[ -s "$cache_dir/$name" ]]; then
        log "  cached  $name"
        continue
    fi
    log "  fetch   $name"
    curl -fsSL --retry 3 --max-time 600 "$base_url/$name" -o "$cache_dir/$name"
done

# 1. The checksums file itself must match the copy recorded in the repo.
expected_manifest_checksum="$(grep -F "  $checksums_name" "$trusted_manifests" | cut -d ' ' -f 1)"
actual_manifest_checksum="$(sha256sum "$cache_dir/$checksums_name" | cut -d ' ' -f 1)"
if [[ "$expected_manifest_checksum" != "$actual_manifest_checksum" ]]; then
    log "error: checksum mismatch for $checksums_name"
    log "       expected $expected_manifest_checksum"
    log "       actual   $actual_manifest_checksum"
    exit 1
fi

# 2. The artifacts themselves must match that checksums file. Windows-built
#    release manifests use CRLF line endings, so normalize before checking.
if ! (cd "$cache_dir" && tr -d '\r' < "$checksums_name" | sha256sum --check --quiet -); then
    log "error: artifact checksum verification failed in $cache_dir"
    exit 1
fi
log "  verified checksums"

printf 'export RUSTY_V8_ARCHIVE=%q\n' "$cache_dir/$archive_name"
printf 'export RUSTY_V8_SRC_BINDING_PATH=%q\n' "$cache_dir/$binding_name"
