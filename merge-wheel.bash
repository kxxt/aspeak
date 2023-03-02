#!/bin/bash

set -e


# Implementation:

# Get the name-version from the first wheel.
TMPDIR=.merge-tmp

rm -rf "$TMPDIR"
mkdir -p "$TMPDIR/tmp1"
mkdir -p "$TMPDIR/tmp2"

# Build the wheel
maturin build -F python --release --bindings pyo3 -o "$TMPDIR/tmp1" $@
maturin build -F python --release --bindings bin  -o "$TMPDIR/tmp2" $@

# Grab Info
file_name=$(basename $(/bin/ls "$TMPDIR/tmp1"/*.whl))
dist_info=$(unzip -qql "$TMPDIR/tmp1/*.whl" | grep "\.dist-info/METADATA" | awk '{print $4}' | cut -d/ -f1)
name_version=$(basename -s '.dist-info' $dist_info)

# Merge wheel
mkdir -p "$TMPDIR/merged"
unzip -qo "$TMPDIR/tmp1/$file_name" -d "$TMPDIR/merged"
unzip -qo "$TMPDIR/tmp2/$file_name" -d "$TMPDIR/merged"

# Merge record
unzip -qjo "$TMPDIR/tmp1/$file_name" "*.dist-info/RECORD" -d "$TMPDIR/tmp1"
unzip -qjo "$TMPDIR/tmp1/$file_name" "*.dist-info/RECORD" -d "$TMPDIR/tmp2"
cat "$TMPDIR/tmp1/RECORD" "$TMPDIR/tmp2/RECORD" | sort | uniq > "$TMPDIR/merged/$name_version.dist-info/RECORD"

# Create the wheel

cd "$TMPDIR/merged"
zip -qr "../../$file_name" *
cd ../..
rm -rf "$TMPDIR"
