#!/usr/bin/env bash
# Package superd + super CLI into super-{version}-{platform}.tar.gz
set -euo pipefail

version="${1:?version required}"
platform="${2:?platform required (e.g. linux-amd64)}"
target="${3:-}"

if [[ ! -f dashboard/dist/index.html ]]; then
  echo "dashboard/dist/index.html missing — build the frontend first" >&2
  exit 1
fi

if [[ -n "$target" ]]; then
  cargo build --release --target "$target" -p superd -p super-cli
  bin_dir="target/${target}/release"
else
  cargo build --release -p superd -p super-cli
  bin_dir="target/release"
fi

root="super-${version}-${platform}"
mkdir -p "${root}/bin"
cp "${bin_dir}/superd" "${bin_dir}/super" "${root}/bin/"
chmod +x "${root}/bin/"*

if [[ -f LICENSE ]]; then
  cp LICENSE "${root}/"
fi

bash .github/scripts/write-release-readme.sh "${version}" "${platform}" "${root}"

tar -czf "${root}.tar.gz" "${root}"
echo "Created ${root}.tar.gz"
