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
 miraland-banks-interface
 miraland-banks-server
 miraland-bloom
 miraland-bucket-map
 miraland-clap-utils
 miraland-clap-v3-utils
 miraland-cli-config
 miraland-cli-output
 miraland-client
 miraland-connection-cache
 miraland-core
 miraland-entry
 miraland-faucet
 miraland-frozen-abi
 miraland-frozen-abi-macro
 miraland-geyser-plugin-interface
 miraland-geyser-plugin-manager
 miraland-gossip
 miraland-ledger
 miraland-logger
 miraland-measure
 miraland-merkle-tree
 miraland-metrics
 miraland-net-utils
 miraland-perf
 miraland-poh
 miraland-program-runtime
 miraland-program-test
 miraland-address-lookup-table-program
 miraland-bpf-loader-program
 miraland-compute-budget-program
 miraland-config-program
 miraland-stake-program
 miraland-vote-program
 miraland-zk-token-proof-program
 miraland-pubsub-client
 miraland-quic-client
 miraland-rayon-threadlimit
 miraland-remote-wallet
 miraland-rpc
 miraland-rpc-client
 miraland-rpc-client-api
 miraland-rpc-client-nonce-utils
 miraland-runtime
 miraland-sdk
 miraland-sdk-macro
 miraland-program
 miraland-send-transaction-service
 miraland-storage-bigtable
 miraland-storage-proto
 miraland-streamer
 miraland-test-validator
 miraland-thin-client
 miraland-tpu-client
 miraland-transaction-status
 miraland-udp-client
 miraland-version
 miraland-zk-token-sdk
)

set -x
for crate in "${crates[@]}"; do
  sed -E -i'' -e "s:(${crate} = \")(=?)${old_miraland_ver}\".*:\1\2${miraland_ver}\":" "${tomls[@]}"
  sed -E -i'' -e "s:(${crate} = \{ version = \")(=?)${old_miraland_ver}(\".*):\1\2${miraland_ver}\3:" "${tomls[@]}"
done
