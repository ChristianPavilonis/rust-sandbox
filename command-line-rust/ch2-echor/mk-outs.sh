#!/usr/bin/env bash

OUTDIR="tests/expected"
[[ ! -d "OUTDIR" ]] && mkdir -p "$OUTDIR"

echo "Hello there" > $OUTDIR/hello1.txt
echo "Hello"  "there" > $OUTDIR/hello2.txt
echo -n "Hello  there" > $OUTDIR/hello1n.txt
echo -n "Hello" "there" > $OUTDIR/hello2n.txt
