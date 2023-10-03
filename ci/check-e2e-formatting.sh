#!/usr/bin/env sh

set -o errexit
set -o xtrace

find "./is_affected/end-to-end-tests/features" -type f -name "*.py" | xargs -I {} autopep8 --exit-code --diff --aggressive --aggressive --max-line-length 120 "{}"