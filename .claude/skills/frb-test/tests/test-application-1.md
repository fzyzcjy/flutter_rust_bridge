# Test Scenario: Application 1 - Rust Codegen Change

**Type:** Application scenario
**Skill under test:** frb-test

## Prompt

```
I just modified some code in frb_codegen/src/library/api_dart/mod.rs to fix a bug in Dart code generation.
Which tests should I run locally to verify this change works correctly?
```

## Expected Behavior

- Correct command: `./frb_internal test-rust-package --package frb_codegen`
