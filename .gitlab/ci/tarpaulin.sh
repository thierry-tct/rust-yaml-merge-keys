#!/bin/sh

set -e

readonly version="0.20.1"
readonly sha256sum="ec7c17e6a1406cc9dfb6ccd1a2319a7414bff84266372997be8d760524416eb0"
readonly filename="cargo-tarpaulin-$version-travis"
readonly tarball="$filename.tar.gz"

cd .gitlab

echo "$sha256sum  $tarball" > tarpaulin.sha256sum
curl -OL "https://github.com/xd009642/tarpaulin/releases/download/$version/$tarball"
sha256sum --check tarpaulin.sha256sum
tar xf "$tarball"
