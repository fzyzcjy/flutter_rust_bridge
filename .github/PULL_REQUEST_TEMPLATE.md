## Changes

Fixes #the-issue-number-here

## Procedure and Checklist

In order to quickly iterate and avoid slowing down development pace by making CI pass, only the following simplified steps are needed, and I (fzyzcjy) will handle the rest of CI / moving the tests currently (will have more automation in the future).

- [ ] Implement the feature / fix the bug.
- [ ] Add tests in `frb_example/dart_minimal`.
- [ ] Make `dart_minimal`'s CI green.

Utility commands
- Run codegen: `cargo run --manifest-path /path/to/flutter_rust_bridge/frb_codegen/Cargo.toml -- generate`
- Run tests: `./frb_internal test-dart-native --package frb_example/dart_minimal`
