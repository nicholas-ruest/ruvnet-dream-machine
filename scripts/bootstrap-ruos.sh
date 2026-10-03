#!/usr/bin/env bash
# User-scoped executor setup; no credentials, daemon, server or publication.
set -euo pipefail
test "$(uname -s)" = Linux
test "$(uname -m)" = x86_64
runtime_dir="${DREAM_RUNTIME_DIR:-${HOME}/.local/share/dream-machine/runtime}"
node_version="24.21.0"
node_archive="node-v${node_version}-linux-x64.tar.xz"
node_sha="fd8e59d5a511510f6a298afb548f18c7d2b1be404d8b4a27d94fbe49f56cb2d6"
mkdir -p "$runtime_dir"
if ! test -x "$runtime_dir/node-v${node_version}-linux-x64/bin/node"; then
  curl --fail --location --silent --show-error --max-time 120 \
    "https://nodejs.org/dist/v${node_version}/${node_archive}" -o "$runtime_dir/$node_archive"
  printf '%s  %s\n' "$node_sha" "$runtime_dir/$node_archive" | sha256sum --check --status
  tar -xJf "$runtime_dir/$node_archive" -C "$runtime_dir"
fi
export PATH="$runtime_dir/node-v${node_version}-linux-x64/bin:${HOME}/.cargo/bin:$PATH"
node --version
npm --version
cargo --version
rustc --version
cargo clippy --version
cargo fmt --version
printf 'Executor verified. Use PATH=%s/node-v%s-linux-x64/bin:%s/.cargo/bin:$PATH for each scheduled command.\n' \
  "$runtime_dir" "$node_version" "$HOME"
