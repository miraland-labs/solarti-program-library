#!/usr/bin/env bash
#
# Patches the SPL crates for developing against a local miraland monorepo
#

miraland_dir=$1
if [[ -z $miraland_dir ]]; then
  echo "Usage: $0 <path-to-miraland-monorepo>"
  exit 1
fi

workspace_crates=(
  Cargo.toml
)

if [[ ! -r "$miraland_dir"/scripts/read-cargo-variable.sh ]]; then
  echo "$miraland_dir is not a path to the miraland monorepo"
  exit 1
fi

set -e

miraland_dir=$(cd "$miraland_dir" && pwd)
cd "$(dirname "$0")"

source "$miraland_dir"/scripts/read-cargo-variable.sh

# The toolchain file only exists in version >= 1.15
toolchain_file="$miraland_dir"/rust-toolchain.toml
if [[ -f "$toolchain_file" ]]; then
  cp "$toolchain_file" .
fi

# get version from Cargo.toml first. if it is empty, get it from other places.
miraland_ver="$(readCargoVariable version "$miraland_dir"/Cargo.toml)"
miraland_ver=${miraland_ver:-$(readCargoVariable version "$miraland_dir"/sdk/Cargo.toml)}

crates_map=()
crates_map+=("miraland-account-decoder account-decoder")
crates_map+=("miraland-banks-client banks-client")
crates_map+=("miraland-banks-server banks-server")
crates_map+=("miraland-bpf-loader-program programs/bpf_loader")
crates_map+=("miraland-clap-utils clap-utils")
crates_map+=("miraland-clap-v3-utils clap-v3-utils")
crates_map+=("miraland-cli-config cli-config")
crates_map+=("miraland-cli-output cli-output")
crates_map+=("miraland-client client")
crates_map+=("miraland-core core")
crates_map+=("miraland-logger logger")
crates_map+=("miraland-notifier notifier")
crates_map+=("miraland-remote-wallet remote-wallet")
crates_map+=("miraland-program sdk/program")
crates_map+=("miraland-program-test program-test")
crates_map+=("miraland-runtime runtime")
crates_map+=("miraland-sdk sdk")
crates_map+=("miraland-stake-program programs/stake")
crates_map+=("miraland-test-validator test-validator")
crates_map+=("miraland-transaction-status transaction-status")
crates_map+=("miraland-version version")
crates_map+=("miraland-vote-program programs/vote")
crates_map+=("miraland-zk-token-sdk zk-token-sdk")

patch_crates=()
for map_entry in "${crates_map[@]}"; do
  read -r crate_name crate_path <<<"$map_entry"
  full_path="$miraland_dir/$crate_path"
  if [[ -r "$full_path/Cargo.toml" ]]; then
    patch_crates+=("$crate_name = { path = \"$full_path\" }")
  fi
done

echo "Patching in $miraland_ver from $miraland_dir"
echo
for crate in "${workspace_crates[@]}"; do
  if grep -q '\[patch.crates-io\]' "$crate"; then
    echo "$crate is already patched"
  else
    cat >> "$crate" <<PATCH
[patch.crates-io]
$(printf "%s\n" "${patch_crates[@]}")
PATCH
  fi
done

./update-miraland-dependencies.sh "$miraland_ver"
