#!/usr/bin/env bash

if [[ $# -ne 1 ]]; then
    printf "Usage: %s DEST_DIR\n" $(basename "$0")
    exit 1
fi

DEST=$1

if [[ ! -d "$DEST" ]]; then
    echo "\"$DEST\" is not a directory"
    exit 1
fi

echo "Creating symlink"
(cd $DEST/tests/inputs/d && rm -f b.csv && ln -s ../a/b/b.csv .)

echo "Done."
