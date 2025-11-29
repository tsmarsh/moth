#!/bin/bash

set -exuo pipefail

CURRENT_MD=$(basename "$( moth ls | grep doing -A1 | tail -n 1 | cut -d '[' -f 1 | xargs | xargs -t -I{} find . -iname '{}*' )")
CURRENT_TASK="${CURRENT_MD%.*}"

# first preserve the task in the main branch
git add .moth/doing
git commit --no-verify -m "Start work on $CURRENT_TASK"

git switch -c "$CURRENT_TASK"
