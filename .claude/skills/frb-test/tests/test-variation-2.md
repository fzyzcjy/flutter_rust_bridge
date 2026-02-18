# Test Scenario: Variation 2 - Multiple Packages Changed

**Type:** Variation scenario
**Skill under test:** frb-test

## Prompt

```
I made changes to both frb_rust and frb_dart.
What tests should I run to verify everything works?
```

## Expected Behavior

- Both packages identified
- Commands:
  - `./frb_internal test-rust-package --package frb_rust`
  - `./frb_internal test-dart-native --package frb_dart`
