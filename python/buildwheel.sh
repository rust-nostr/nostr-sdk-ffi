#!/bin/bash

# Build a wheel. This script is used from the Dockerfile!

set -exuo pipefail

# Check if required arguments are provided
if [ -z "$PLAT_NAME" ]; then
    echo "ERROR: PLAT_NAME environment variable is required"
    exit 1
fi

echo "Building wheel for platform: $PLAT_NAME"

# Copy binaries to python directory if they exist
if [ "$(ls -A /build/binaries)" ]; then
    cp -r /build/binaries/* /build/python/src/nostr-sdk/
    echo "Copied binaries to Python package directory"
else
    echo "No binaries found in /build/binaries"
    exit 1
fi

# Copy generated binding file to the correct location if it exists
if [ -f "/build/binding/nostr_sdk.py" ]; then
    # Make sure the target directory exists
    cp /build/binding/nostr_sdk.py /build/python/src/nostr-sdk/
    echo "Copied binding file (nostr_sdk.py) to src/nostr-sdk/ directory"
else
    echo "WARNING: Binding file (nostr_sdk.py) not found in /build/binding"
    exit 1
fi


# Enter Python package directory
cd /build/python

# Build the wheel with Python 3.9 ABI3 compatibility
python setup.py bdist_wheel --plat-name "$PLAT_NAME" --python-tag cp39.abi3

# Copy wheel to the output directory
cp dist/*.whl /build/dist/
