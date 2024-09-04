#!/usr/bin/env sh

set -o errexit
set -o xtrace

cd "is_affected/end-to-end-tests/"
behave
