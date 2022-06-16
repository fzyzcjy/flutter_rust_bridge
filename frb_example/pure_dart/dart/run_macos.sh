#!/bin/zsh
#
# This script will build the rust dynamic library, make an executable
# from the dart code and give the library as an argument to the
# dart executable. The dart executable will load the dylib at runtime.
#
# It is intended to serve macOS developers working on the pure_dart example
# to run it locally.
#
# start by making this script executable with `chmod +x run_macos.sh`
#
# Make sure to run from `flutter_rust_bridge/frb_example/pure_dart/dart`
# and run `dart pub get` before running this script

export CARGO_TARGET_DIR;
CARGO_TARGET_DIR="$(pwd)/../rust/target"

(cd ../rust && cargo build)

dart compile exe lib/main.dart -o tests
./tests "${CARGO_TARGET_DIR}/debug/libflutter_rust_bridge_example.dylib"