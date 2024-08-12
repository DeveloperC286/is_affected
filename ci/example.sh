#!/usr/bin/env sh

set -o errexit
set -o xtrace

if /usr/local/cargo/bin/is_affected --from-reference "origin/${{ github.base_ref }}" --affects "is_affected/"; then
    echo "Was affected."
else
    echo "Was not affected."
fi

echo "Cleanup etc."
