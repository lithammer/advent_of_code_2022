#!/bin/sh

DEST="day$(printf '%02d' "$1")/src/input.txt"

curl -s --cookie "session=$(cat session)" "https://adventofcode.com/2022/day/$1/input" >"$DEST"

head -3 "$DEST"
echo '...'
tail -3 "$DEST"
