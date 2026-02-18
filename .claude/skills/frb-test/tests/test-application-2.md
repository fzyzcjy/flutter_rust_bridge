# Test Scenario: Application 2 - Example API Change

**Type:** Application scenario
**Skill under test:** frb-test

## Prompt

```
I added a new function `hello_twin_normal` to frb_example/pure_dart/rust/src/api/misc.rs
and wrote a test in frb_example/pure_dart/test/api/misc_test.dart.
The CI is failing. How can I reproduce and debug this locally?
```

## Expected Behavior

- Correct command: `./frb_internal test-dart-native --package frb_example/pure_dart`
