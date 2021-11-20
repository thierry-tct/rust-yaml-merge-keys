#!/bin/sh

set -e

readonly version="0.16.0"
readonly sha256sum="c8abe5afdba8fc206dcd1d18a6b3ba68378e07172ecbfe66576672d247eeb794"
readonly basename="cargo-audit-x86_64-unknown-linux-musl-v$version"
readonly filename="$basename.tgz"

cd .gitlab

echo "$sha256sum  $filename" > cargo-audit.sha256sum
curl -OL "https://github.com/rustsec/rustsec/releases/download/cargo-audit%2Fv$version/$filename"
sha256sum --check cargo-audit.sha256sum
tar --strip-components=1 -xf "$filename" "$basename/cargo-audit"
chmod +x cargo-audit
