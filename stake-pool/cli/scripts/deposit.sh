#!/usr/bin/env bash

# Script to deposit stakes and MLN into a stake pool, given the stake pool keyfile
# and a path to a file containing a list of validator vote accounts

cd "$(dirname "$0")" || exit
stake_pool_keyfile=$1
validator_list=$2
mln_amount=$3

create_keypair () {
  if test ! -f "$1"
  then
    miraland-keygen new --no-passphrase -s -o "$1"
  fi
}

create_user_stakes () {
  validator_list=$1
  mln_amount=$2
  authority=$3
  while read -r validator
  do
    create_keypair "$keys_dir/stake_$validator".json
    miraland create-stake-account "$keys_dir/stake_$validator.json" "$mln_amount" --withdraw-authority "$authority" --stake-authority "$authority"
  done < "$validator_list"
}

delegate_user_stakes () {
  validator_list=$1
  authority=$2
  while read -r validator
  do
    miraland delegate-stake --force "$keys_dir/stake_$validator.json" "$validator" --stake-authority "$authority"
  done < "$validator_list"
}

deposit_stakes () {
  stake_pool_pubkey=$1
  validator_list=$2
  authority=$3
  while read -r validator
  do
    stake=$(miraland-keygen pubkey "$keys_dir/stake_$validator.json")
    $spl_stake_pool deposit-stake "$stake_pool_pubkey" "$stake" --withdraw-authority "$authority"
  done < "$validator_list"
}

keys_dir=keys
stake_pool_pubkey=$(miraland-keygen pubkey "$stake_pool_keyfile")

spl_stake_pool=solarti-stake-pool
# Uncomment to use a locally build CLI
#spl_stake_pool=../../../target/debug/solarti-stake-pool

echo "Setting up keys directory $keys_dir"
mkdir -p $keys_dir
authority=$keys_dir/authority.json
echo "Setting up authority for deposited stake accounts at $authority"
create_keypair $authority

echo "Creating user stake accounts to deposit into the pool"
create_user_stakes "$validator_list" "$mln_amount" $authority
echo "Delegating user stakes so that deposit will work"
delegate_user_stakes "$validator_list" $authority

echo "Waiting for stakes to activate, this may take awhile depending on the network!"
echo "If you are running on localnet with 32 slots per epoch, wait 12 seconds..."
sleep 12
echo "Depositing stakes into stake pool"
deposit_stakes "$stake_pool_pubkey" "$validator_list" $authority
