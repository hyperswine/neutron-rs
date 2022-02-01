#!/bin/bash

NAME=$1

if [ $# -ne 1 ]; then
    echo "No argument supplied, calling the archive 'archive'"
    NAME="archive"
fi

DATE=$(date "+%Y-%m-%d")

if [ -f "archive/$DATE-$NAME.md" ]; then
    echo "Archive for that date/name already exists..."
    exit 1
fi

cat >"archive/$DATE-$NAME.md" <<ENDOFFILE
# Archive
ENDOFFILE
