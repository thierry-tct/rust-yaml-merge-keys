#!/bin/sh

set -e

readonly version="0.11.1"
readonly sha256sum="b51d5c233e1145036c50168d50a4fb3a9f09c19511a04da866f662ed7217eb5f"
readonly filename="cargo-tarpaulin-$version-travis"
readonly tarball="$filename.tar.gz"

cd .gitlab

echo "$sha256sum  $tarball" > tarpaulin.sha256sum
curl -OL "https://github.com/xd009642/tarpaulin/releases/download/$version/$tarball"
sha256sum --check tarpaulin.sha256sum
tar xf "$tarball"
