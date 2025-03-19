#!/bin/bash

set -euo pipefail

cargo check
cargo test
cargo clippy -- -D warnings
