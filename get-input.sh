#!/bin/sh -e

[ -z "$1" ] && echo "Usage: ${0##*/} <day>" && exit 1

DAY=$(printf '%d' "$1")
PADDED_DAY=$(printf '%02d' "$DAY")
DEST="day$PADDED_DAY/src/input.txt"

curl -s --cookie "session=$(cat session)" "https://adventofcode.com/2022/day/$DAY/input" >"$DEST"

head -3 "$DEST"
echo '[...]'
tail -3 "$DEST"
