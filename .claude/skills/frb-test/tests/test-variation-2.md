# Test Scenario: Variation 2 - Multiple Packages Changed

**Type:** Variation scenario
**Skill under test:** frb-test

## Scenario Setup

You are an AI assistant helping with flutter_rust_bridge development. A developer has made changes across multiple packages.

## Prompt

```
I made changes to both frb_rust and frb_dart.
What tests should I run to verify everything works?
```

## Expected Behavior (with skill)

The agent should:
1. Recognize multiple packages need testing
2. Recommend both:
   - `./frb_internal test-rust-package --package frb_rust`
   - `./frb_internal test-dart-native --package frb_dart`
3. Mention that tests can be run in parallel

## Baseline Documentation (what agent does WITHOUT skill)

**Result:** Agent found correct answer with integration test suggestions.

**Commands suggested:**
1. Rust: `./frb_internal test-rust-package --package frb_rust`
2. Dart: `./frb_internal test-dart-native --package frb_dart`
3. Integration: `./frb_internal test-dart-native --package frb_example/pure_dart`
4. Web: `./frb_internal test-dart-web --package frb_dart`

**Assessment:**
- ✅ Both packages identified
- ✅ Correct commands for both
- ✅ Suggested integration tests
- ✅ Mentioned web testing option

## Evaluation Criteria

- Both packages identified: frb_rust and frb_dart ✅
- Correct commands for both:
  - Rust: `./frb_internal test-rust-package --package frb_rust` ✅
  - Dart: `./frb_internal test-dart-native --package frb_dart` ✅
