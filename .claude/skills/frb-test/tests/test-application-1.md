# Test Scenario: Application 1 - Rust Codegen Change

**Type:** Application scenario
**Skill under test:** frb-test

## Scenario Setup

You are an AI assistant helping with flutter_rust_bridge development. A developer has made changes to the code generator in `frb_codegen/src/library/api_dart/mod.rs`.

## Prompt

```
I just modified some code in frb_codegen/src/library/api_dart/mod.rs to fix a bug in Dart code generation.
Which tests should I run locally to verify this change works correctly?
```

## Expected Behavior (with skill)

The agent should:
1. Recognize this is a frb_codegen change
2. Recommend `./frb_internal test-rust-package --package frb_codegen`
3. Possibly also mention running Dart tests for examples that use the codegen

## Baseline Documentation (what agent does WITHOUT skill)

**Result:** Agent explored codebase and found correct answer.

**Commands suggested:**
1. Direct unit tests: `cargo test --package flutter_rust_bridge_codegen --lib -- generator::api_dart::tests --test-threads=1`
2. All frb_codegen tests: `./frb_internal test-rust-package --package frb_codegen`
3. Integration tests: `./frb_internal generate-run-frb-codegen-command-generate --package frb_example/pure_dart`
4. Dart tests: `./frb_internal test-dart-native --package frb_example/pure_dart`
5. Golden update: `UPDATE_GOLDENS=1 cargo test ...`

**Assessment:**
- ✅ Correct command found
- ✅ Comprehensive answer with multiple options
- ⚠️ Response was verbose (many options given)
- ✅ Mentioned golden tests

## Evaluation Criteria

- Correct command: `./frb_internal test-rust-package --package frb_codegen` ✅
- Bonus: Mentioning that codegen tests might need examples to be regenerated ✅
- Bonus: Mentioning CI will also run tests - NOT mentioned
