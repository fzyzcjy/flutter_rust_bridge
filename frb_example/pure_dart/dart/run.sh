#!/bin/bash

set -eux

if [[ -z "${CARGO_TARGET_DIR}" ]]; then
  echo 'Please set environment variable CARGO_TARGET_DIR'
  exit 1
fi

(cd ../rust && cargo build --verbose)

# dart pub get

# need to be AOT, since prod environment is AOT, and JIT+valgrind will have strange problems
dart compile exe lib/main.dart -o main.exe

PYTHONUNBUFFERED=1 ./valgrind_util.py ./main.exe "${CARGO_TARGET_DIR}/debug/libflutter_rust_bridge_example.so" --chain-stack-traces
