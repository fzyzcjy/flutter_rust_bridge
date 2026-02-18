# Test Scenario: Application 2 - Example API Change

**Type:** Application scenario
**Skill under test:** frb-test

## Scenario Setup

You are an AI assistant helping with flutter_rust_bridge development. A developer has added a new function to the pure_dart example.

## Prompt

```
I added a new function `hello_twin_normal` to frb_example/pure_dart/rust/src/api/misc.rs
and wrote a test in frb_example/pure_dart/test/api/misc_test.dart.
The CI is failing. How can I reproduce and debug this locally?
```

## Expected Behavior (with skill)

The agent should:
1. Recognize this is a Dart example change
2. Recommend `./frb_internal test-dart-native --package frb_example/pure_dart`
3. Explain that local testing helps debug CI failures
4. Possibly mention checking if codegen was run

## Baseline Documentation (what agent does WITHOUT skill)

**Result:** Agent found correct answer with comprehensive explanation.

**Commands suggested:**
1. `./frb_internal test-dart-native --package frb_example/pure_dart`
2. Alternative direct: `dart --enable-experiment=native-assets test`
3. Codegen first: `./frb_internal generate-run-frb-codegen-command-generate --package frb_example/pure_dart`

**Assessment:**
- ✅ Correct command found
- ✅ Correct package path (frb_example/pure_dart)
- ✅ Explained codegen prerequisite
- ✅ Explained twin naming convention
- ⚠️ Response was verbose

## Evaluation Criteria

- Correct command: `./frb_internal test-dart-native --package frb_example/pure_dart` ✅
- Correct package path: frb_example/pure_dart ✅
- Mention of codegen requirements ✅
