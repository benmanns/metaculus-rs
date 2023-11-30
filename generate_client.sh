#!/usr/bin/env bash

set -euo pipefail

./process_spec.rb
# Clean up generated YAML to make it closer to the original
sed -E \
  -e '1d' \
  -e 's/"\$ref": "([^"'"'"']+)"/$ref: '"'"'\1'"'"'/g' \
  -e 's/^  "(\/[^"]+)":$/  \1:/g' \
  -e 's/\bpattern: "\^\.\+\$"$/pattern: ^.+$/g' \
  -i "Metaculus API (1.0) Modified.yaml"

openapi-generator-cli generate --config openapi-generator-cli.yaml
# enablePostProcessFile is broken, so post-process here:
cargo fmt --all
cargo clippy --all-features --fix --allow-dirty --allow-staged -- -W clippy::pedantic
# Reformat after fix, as clippy may have changed formatting
cargo fmt --all
