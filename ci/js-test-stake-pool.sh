#!/usr/bin/env bash

set -ex
cd "$(dirname "$0")/.."
source ./ci/miraland-version.sh install

cd stake-pool/js
npm install
npm run lint
npm run build
npm run test
