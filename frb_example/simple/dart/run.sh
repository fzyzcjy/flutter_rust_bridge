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

# some configurations ref: https://github.com/dart-lang/sdk/blob/master/runtime/tools/valgrind.py
# --log-file=valgrind.log # you can add this
valgrind --leak-check=full --trace-children=yes --ignore-ranges=0x000-0xFFF \
  --error-exitcode=1 --errors-for-leak-kinds=definite \
  --suppressions=dart.supp \
  ./main.exe "${CARGO_TARGET_DIR}/debug/libflutter_rust_bridge_example.so"