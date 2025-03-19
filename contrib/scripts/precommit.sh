#!/bin/bash

set -exuo pipefail

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"

"${DIR}/check-fmt.sh"       # Format the code
"${DIR}/check-crate.sh"     # Check crate
