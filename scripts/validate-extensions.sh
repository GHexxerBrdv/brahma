#!/usr/bin/env bash
set -euo pipefail
for file in extensions/*/extension.yaml; do
  test -f "$file"
  echo "validated: $file"
done
