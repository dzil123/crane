#!/bin/bash

set -euxo pipefail

pushd "$(dirname "${BASH_SOURCE[0]}")/go"

go get -u all
go mod tidy

popd
