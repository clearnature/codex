#!/usr/bin/env bash
#
# Prepare a local MUSL cross toolchain, following what the release workflow
# does on CI runners.
#
# Why this is more than "apt install musl-tools":
#   * the Rust linker must be musl-gcc (not zig), so the target needs
#     musl-tools;
#   * native dependencies (aws-lc-sys, boring, ...) are compiled with zig cc
#     wrappers, so Zig 0.14.0 must be on PATH;
#   * the distro libcap-dev is glibc-linked and therefore unusable for musl,
#     so libcap is rebuilt from source against musl-gcc;
#   * CI's .github/scripts/install-musl-build-tools.sh exports ~20 variables
#     -- and it writes them to $GITHUB_ENV, which does not exist locally.
#
# This script chains all of that and produces a sourceable env file.
#
# Usage:
#   scripts/setup-musl-toolchain.sh
#   scripts/setup-musl-toolchain.sh x86_64-unknown-linux-musl
#   eval "$(scripts/setup-musl-toolchain.sh --print-env)"
#
# After it succeeds:
#   set -a; . /tmp/codex-musl-env-<target>.sh; set +a
#   scripts/build-release-local.sh <target>
#
# The apt step needs sudo; everything else runs as your user.

set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

target=""
print_env=0
while [[ $# -gt 0 ]]; do
    case "$1" in
        --print-env) print_env=1 ;;
        -h | --help)
            sed -n '2,32p' "${BASH_SOURCE[0]}" | sed -e 's/^# \{0,1\}//' >&2
            exit 0
            ;;
        -*) printf 'error: unknown flag %s\n' "$1" >&2; exit 2 ;;
        *) target="$1" ;;
    esac
    shift
done

target="${target:-x86_64-unknown-linux-musl}"

case "$target" in
    x86_64-unknown-linux-musl) zig_arch="x86_64" ;;
    aarch64-unknown-linux-musl) zig_arch="aarch64" ;;
    *)
        printf 'error: %s is not a musl target this script knows\n' "$target" >&2
        exit 2
        ;;
esac

zig_version="0.14.0"
zig_dir="${CODEX_ZIG_DIR:-$HOME/.local/opt}/zig-linux-${zig_arch}-${zig_version}"
env_file="${CODEX_MUSL_ENV_FILE:-/tmp/codex-musl-env-${target}.sh}"

log() { printf '\n== %s ==\n' "$*" >&2; }

# ---------------------------------------------------------------------------
log "step 1/5: rustup target add $target"
# Run this from codex-rs/: rust-toolchain.toml there pins the toolchain Cargo
# will actually use, and `rustup target add` only installs for the *active*
# toolchain. Running it from the repo root installs into `stable` instead and
# the build later fails with "can't find crate for `core`" (E0463).
if (cd "$repo_root/codex-rs" && rustup target list --installed | grep -qx "$target"); then
    printf 'already installed for the pinned toolchain\n' >&2
else
    (cd "$repo_root/codex-rs" && rustup target add "$target")
fi

# ---------------------------------------------------------------------------
log "step 2/5: prerequisites"
missing_prereqs=()
for tool in curl tar xz make; do
    command -v "$tool" >/dev/null 2>&1 || missing_prereqs+=("$tool")
done
if ((${#missing_prereqs[@]})); then
    printf 'error: missing: %s\n' "${missing_prereqs[*]}" >&2
    printf '       (Debian/Ubuntu: sudo apt-get install -y curl xz-utils make)\n' >&2
    exit 1
fi
printf 'curl/tar/xz/make present\n' >&2

# ---------------------------------------------------------------------------
log "step 3/5: Zig $zig_version"
if [[ -x "$zig_dir/zig" ]]; then
    printf 'already installed at %s\n' "$zig_dir" >&2
else
    # Note the asset naming: zig-linux-<arch>-<version>.tar.xz. The newer
    # "zig-<arch>-linux-<version>.tar.xz" form 404s for 0.14.0.
    zig_archive="zig-linux-${zig_arch}-${zig_version}.tar.xz"
    zig_url="https://ziglang.org/download/${zig_version}/${zig_archive}"
    mkdir -p "$(dirname "$zig_dir")"
    printf 'downloading %s\n' "$zig_url" >&2
    curl -fsSL --retry 3 --max-time 900 "$zig_url" -o "/tmp/$zig_archive"
    tar -xJf "/tmp/$zig_archive" -C "$(dirname "$zig_dir")"
    rm -f "/tmp/$zig_archive"
fi
export PATH="$zig_dir:$PATH"
printf 'zig: %s\n' "$(zig version)" >&2

# Make zig visible to later shells too; the generated env file also exports
# PATH, but the build script may be run without sourcing it first.
if [[ -d "$HOME/.local/bin" && ! -e "$HOME/.local/bin/zig" ]]; then
    ln -s "$zig_dir/zig" "$HOME/.local/bin/zig"
    printf 'linked %s -> %s\n' "$HOME/.local/bin/zig" "$zig_dir/zig" >&2
fi

# ---------------------------------------------------------------------------
log "step 4/5: libcap rebuilt for musl + toolchain environment"

# musl-gcc is the Rust linker; it comes from the musl-tools package. This is
# the one and only part of the whole setup that needs sudo, so check it first
# and print the exact command instead of running it.
if ! command -v musl-gcc >/dev/null 2>&1 &&
    ! command -v "${zig_arch}-linux-musl-gcc" >/dev/null 2>&1; then
    {
        printf 'error: musl-gcc not found.\n'
        printf '\nThis is the only step that needs sudo -- run it yourself:\n\n'
        printf '  sudo apt-get update && sudo apt-get install -y \\\n'
        printf '    ca-certificates curl musl-tools pkg-config libcap-dev \\\n'
        printf '    g++ clang libc++-dev libc++abi-dev lld xz-utils binutils\n\n'
        printf 'Then rerun: %s\n' "${BASH_SOURCE[0]}"
    } >&2
    exit 1
fi

# The CI script does two things: it apt-installs (sudo), and it builds a musl
# libcap plus exports the toolchain environment. We have already satisfied the
# apt part, so run it with those two lines filtered out -- everything else
# stays byte-identical to CI, and no privileges are needed.
# It also insists on GITHUB_ENV, so point that at a temp file and convert it
# into a sourceable env file below.
ci_copy="$(mktemp --suffix=.sh)"
ci_env="$(mktemp)"
rm -f "$ci_env"
grep -vE '^[[:space:]]*sudo apt-get' \
    "$repo_root/.github/scripts/install-musl-build-tools.sh" >"$ci_copy"
TARGET="$target" GITHUB_ENV="$ci_env" bash "$ci_copy"
rm -f "$ci_copy"

# ---------------------------------------------------------------------------
log "step 5/5: exporting environment"
{
    printf '# generated by scripts/setup-musl-toolchain.sh\n'
    printf 'export AWS_LC_SYS_NO_JITTER_ENTROPY=1\n'
    printf 'export PATH="%s:$PATH"\n' "$zig_dir"
    # Values such as CMAKE_ARGS / CFLAGS contain spaces, so they must be
    # shell-quoted; a bare `export K=v w` would make source execute `w`.
    while IFS= read -r line; do
        printf 'export %s=%q\n' "${line%%=*}" "${line#*=}"
    done < <(grep -E '^[A-Za-z_][A-Za-z0-9_]*=' "$ci_env")
} >"$env_file"

if [[ "$print_env" == "1" ]]; then
    cat "$env_file"
else
    log "done"
    printf 'environment written to %s\n' "$env_file" >&2
    printf '\nnext:\n' >&2
    printf '  set -a; . %s; set +a\n' "$env_file" >&2
    printf '  scripts/build-release-local.sh %s\n' "$target" >&2
fi
