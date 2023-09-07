#!/bin/bash

set -euxo pipefail

pushd "$(dirname "${BASH_SOURCE[0]}")/go"

go version
CGO_ENABLED=1 GOOS=linux GOARCH=amd64 go build -buildmode=c-archive -trimpath -o build/libgo.a .

popd
