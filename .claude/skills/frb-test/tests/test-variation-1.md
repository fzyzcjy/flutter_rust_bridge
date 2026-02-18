# Test Scenario: Variation 1 - Web Test Request

**Type:** Variation scenario
**Skill under test:** frb-test

## Scenario Setup

You are an AI assistant helping with flutter_rust_bridge development. A developer needs to run web tests.

## Prompt

```
I need to verify my changes work on the web platform.
How do I run web tests for frb_dart locally?
```

## Expected Behavior (with skill)

The agent should:
1. Recognize this is a web test request
2. Recommend `./frb_internal test-dart-web --package frb_dart`
3. Note that web tests are different from native tests

## Baseline Documentation (what agent does WITHOUT skill)

**Result:** Agent found correct answer with good explanation.

**Commands suggested:**
1. Direct: `dart test -p chrome`
2. Internal tool: `./frb_internal test-dart-web --package frb_dart`

**Assessment:**
- ✅ Found both approaches
- ✅ Explained difference between frb_dart and other packages
- ✅ Understood web vs native testing distinction
- ✅ Mentioned Chrome requirement

## Evaluation Criteria

- Correct command: `./frb_internal test-dart-web --package frb_dart` ✅
- Understanding that web tests need a different command than native tests ✅
