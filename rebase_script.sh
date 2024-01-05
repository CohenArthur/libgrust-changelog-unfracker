#!/bin/sh

set -e

lcu=$1
commit_script_check=$2

if !$(commit_script_check); then
    # Get the original commit message
    git log -1 --format="%B" > /tmp/commit

    # Format it properly
    $(lcu) --input "$(cat /tmp/commit)" > /tmp/fixed

    # Recommit it
    git commit --amend --file /tmp/fixed
fi
