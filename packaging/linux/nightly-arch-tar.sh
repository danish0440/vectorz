#!/usr/bin/env bash
# Stage the VectorZ Nightly-only handoff for the Arch packager.
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
source_tree="$1"
work="$2"
destination="$3"
basename="$4"
stage="$work/$basename"

mkdir -p "$work"
cp -R "$source_tree" "$stage"
mkdir "$stage/bin/.vectorz-nightly"
# The GUI goes into the hidden folder; `bin/vectorz` below is the launcher that starts it.
mv "$stage/bin/vectorz-bin" "$stage/bin/.vectorz-nightly/vectorz-bin"
cp "$here/nightly-launch.sh" "$stage/bin/vectorz"
chmod 755 "$stage/bin/vectorz"
tar -C "$work" -czf "$destination/$basename.tar.gz" "$basename"
echo "wrote $destination/$basename.tar.gz"
