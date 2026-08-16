#!/usr/bin/env bash

set -euo pipefail

if [ "$#" -lt 2 ] || [ "$#" -gt 3 ]; then
  echo "Usage: $0 <cargo-manifest-dir> <output-dir> [debug|release]" >&2
  exit 64
fi

BASEDIR=$(cd "$(dirname "$0")" && pwd -P)
MANIFEST_DIR=$(cd "$1" && pwd -P)
OUTPUT_DIR=$(mkdir -p "$2" && cd "$2" && pwd -P)
CONFIGURATION=${3:-release}

case "$CONFIGURATION" in
  debug|release) ;;
  *)
    echo "Configuration must be debug or release: $CONFIGURATION" >&2
    exit 64
    ;;
esac

export CARGOKIT_CONFIGURATION="$CONFIGURATION"
export CARGOKIT_MANIFEST_DIR="$MANIFEST_DIR"
export CARGOKIT_OUTPUT_DIR="$OUTPUT_DIR"
export CARGOKIT_TARGET_TEMP_DIR="$OUTPUT_DIR/.cargokit-build"
export CARGOKIT_TOOL_TEMP_DIR="$CARGOKIT_TARGET_TEMP_DIR/build_tool"
export CARGOKIT_ROOT_PROJECT_DIR="$(pwd -P)"

exec "$BASEDIR/run_build_tool.sh" build-spm
