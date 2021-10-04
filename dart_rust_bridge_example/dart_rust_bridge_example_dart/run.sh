#!/bin/bash

set -eux

if [[ -z "${CARGO_TARGET_DIR}" ]]; then
  echo 'Please set environment variable CARGO_TARGET_DIR'
  exit 1
fi

(cd ../dart_rust_bridge_example_rust && cargo build)

# dart pub get

# need to be AOT, since prod environment is AOT, and JIT+valgrind will have strange problems
dart compile exe lib/main.dart -o main.exe

# some configurations ref: https://github.com/dart-lang/sdk/blob/master/runtime/tools/valgrind.py
echo 'Please look at valgrind.log to see valgrind outputs!'
valgrind --leak-check=full --trace-children=yes --ignore-ranges=0x000-0xFFF --log-file=valgrind.log ./main.exe "${CARGO_TARGET_DIR}/debug/libdart_rust_bridge_example.so" --observe