#!/bin/bash

cp README.md.in README.md
version=$(taplo get -f Cargo.toml -s  package.version)

sed -i "s/@@ASPEAK_VERSION@@/$version/g" README.md
rg --replace "$(cat src/cli/aspeak.toml)" --passthru --no-line-number \
   --multiline --multiline-dotall '@@PROFILE_TEMPLATE@@' README.md > README.md.new
mv README.md.new README.md

