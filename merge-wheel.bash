#!/bin/bash

set -ex


# Implementation:

# Get the name-version from the first wheel.
DIST_DIR=dist

rm -rf "$DIST_DIR"
# mkdir -p "$DIST_DIR/dist-pyo3"
# mkdir -p "$DIST_DIR/dist-bin"

# Build the wheel
# maturin build -F python --release --bindings pyo3 -o "$DIST_DIR/dist-pyo3" $@
# maturin build -F python --release --bindings bin  -o "$DIST_DIR/dist-bin" $@

ls -lah dist-pyo3

# Grab Info
file_name=$(basename $(/bin/ls dist-pyo3/*.whl))

if [[ "$OSTYPE" == "msys" ]]; then
    DELIM='\'
else
    DELIM='/'
fi

dist_info=$(7z l -ba dist-pyo3/*.whl | grep "\.dist-info[\\/|/]METADATA" | awk '{print $6}' | cut -d"$DELIM" -f1)
echo "The dist-info is $dist_info"

name_version=$(basename -s '.dist-info' $dist_info)

# Merge wheel
mkdir -p "$DIST_DIR/merged"
7z x -y "dist-pyo3/$file_name" -o"$DIST_DIR/merged"
7z x -y "dist-bin/*.whl"  -o"$DIST_DIR/merged"

# Merge record
7z e -y "dist-pyo3/$file_name" "*.dist-info/RECORD" -odist-pyo3
7z e -y "dist-bin/*.whl"  "*.dist-info/RECORD" -odist-bin
cat dist-pyo3/RECORD dist-bin/RECORD | sort | uniq > "$DIST_DIR/merged/$name_version.dist-info/RECORD"

# Create the wheel

cd "$DIST_DIR/merged"
7z a -tzip "../$file_name" .
cd ..
rm -rf "merged"
