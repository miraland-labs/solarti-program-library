#!/usr/bin/env bash
#
# Patch in a Miraland monorepo that supports 2x transactions for testing the
# Solarti Token 2022 Confidential Transfer extension
#

set -e

here="$(dirname "$0")"
cd "$here"

if [[ ! -d twoxtx-miraland ]]; then
  if [[ -n $CI ]]; then
    git config --global user.email "you@example.com"
    git config --global user.name "Your Name"
    git clone https://github.com/miraland-labs/miraland.git twoxtx-miraland
  else
    git clone git@github.com:miraland-labs/miraland.git twoxtx-miraland
  fi
fi

if [[ ! -f twoxtx-miraland/.twoxtx-patched ]]; then
  git -C twoxtx-miraland am -3 "$PWD"/twoxtx.patch
  touch twoxtx-miraland/.twoxtx-patched
fi

../patch.crates-io.sh twoxtx-miraland

exit 0
