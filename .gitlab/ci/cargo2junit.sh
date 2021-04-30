#!/bin/sh

set -e

readonly version="master"
readonly version_full="$version-01041853b-20210429.0"
readonly sha256sum="b7b97e95d03c035020fdb23c3f0a86434c6e5025189e60cecc0f06c9919eba16"
readonly filename="cargo2junit-v$version-x86_64-unknown-linux-gnu"

cd .gitlab

echo "$sha256sum  $filename" > cargo2junit.sha256sum
curl -OL "https://gitlab.kitware.com/api/v4/projects/6955/packages/generic/cargo2junit/v$version_full/cargo2junit-v$version-x86_64-unknown-linux-gnu"
sha256sum --check cargo2junit.sha256sum
mv "$filename" cargo2junit
chmod +x cargo2junit
