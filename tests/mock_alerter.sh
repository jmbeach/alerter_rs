#!/bin/bash
# Mock alerter binary for testing.
# Echoes args as plain text or JSON depending on --json flag.
# Supports --remove and --list flags.

HAS_JSON=false
HAS_LIST=false
HAS_REMOVE=false
MESSAGE=""
LIST_GROUP=""
REMOVE_GROUP=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --json) HAS_JSON=true; shift ;;
        --message) MESSAGE="$2"; shift 2 ;;
        --list) HAS_LIST=true; LIST_GROUP="$2"; shift 2 ;;
        --remove) HAS_REMOVE=true; REMOVE_GROUP="$2"; shift 2 ;;
        *) shift ;;
    esac
done

if $HAS_REMOVE; then
    exit 0
fi

if $HAS_LIST; then
    echo "group: $LIST_GROUP"
    exit 0
fi

if $HAS_JSON; then
    echo '{"activationType": "@CONTENTCLICKED", "activationValue": null}'
else
    echo "@CONTENTCLICKED"
fi
