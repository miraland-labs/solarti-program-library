#!/usr/bin/env bash
#
# Updates the miraland version in all the SPL crates
#

miraland_ver=$1
if [[ -z $miraland_ver ]]; then
  echo "Usage: $0 <new-miraland-version>"
  exit 1
fi

cd "$(dirname "$0")"
source ./ci/miraland-version.sh
old_miraland_ver=${miraland_version#v}

sed -i'' -e "s#miraland_version=v.*#miraland_version=v${miraland_ver}#" ./ci/miraland-version.sh
sed -i'' -e "s#miraland_version = \".*\"#miraland_version = \"${miraland_ver}\"#" ./Anchor.toml

declare tomls=()
while IFS='' read -r line; do tomls+=("$line"); done < <(find . -name Cargo.toml)

crates=(
  miraland-account-decoder
  miraland-banks-client
  miraland-banks-server
  miraland-bpf-loader-program
  miraland-clap-utils
  miraland-clap-v3-utils
  miraland-cli-config
  miraland-cli-output
  miraland-client
  miraland-core
  miraland-logger
  miraland-notifier
  miraland-program
  miraland-program-test
  miraland-remote-wallet
  miraland-runtime
  miraland-sdk
  miraland-stake-program
  miraland-test-validator
  miraland-transaction-status
  miraland-vote-program
  miraland-version
  miraland-zk-token-sdk
)

set -x
for crate in "${crates[@]}"; do
  sed -E -i'' -e "s:(${crate} = \")(=?)${old_miraland_ver}\".*:\1\2${miraland_ver}\":" "${tomls[@]}"
  sed -E -i'' -e "s:(${crate} = \{ version = \")(=?)${old_miraland_ver}(\".*):\1\2${miraland_ver}\3:" "${tomls[@]}"
done
