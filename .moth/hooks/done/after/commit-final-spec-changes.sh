#!/bin/bash

set -exuo pipefail

CURRENT_MD=$(basename "$( moth ls | grep doing -A1 | tail -n 1 | cut -d '[' -f 1 | xargs | xargs -t -I{} find . -iname '{}*' )")
CURRENT_TASK="${CURRENT_MD%.*}"

git add .moth/done
git commit --no-verify -m "Finished work on $CURRENT_TASK"