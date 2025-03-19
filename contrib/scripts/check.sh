#!/bin/bash

set -exuo pipefail

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

"${DIR}/check-fmt.sh" check        # Check if Rust code is formatted
"${DIR}/check-crate.sh"            # Check crates
