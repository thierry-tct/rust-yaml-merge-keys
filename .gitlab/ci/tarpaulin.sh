#!/bin/sh

set -e

readonly version="0.25.0"
readonly sha256sum="e6ae2d11e684ee20f3860b1833de6fcb6ac44db5fc49a82a179bb969924870f3"
readonly filename="cargo-tarpaulin-x86_64-unknown-linux-musl"
readonly tarball="$filename.tar.gz"

cd .gitlab

echo "$sha256sum  $tarball" > tarpaulin.sha256sum
curl -OL "https://github.com/xd009642/tarpaulin/releases/download/$version/$tarball"
sha256sum --check tarpaulin.sha256sum
tar xf "$tarball"
