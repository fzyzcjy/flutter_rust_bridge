# Test Scenario: Gap 1 - Flutter Integration Test

**Type:** Missing information test
**Skill under test:** frb-test

## Scenario Setup

You are an AI assistant helping with flutter_rust_bridge development. A developer is working on a Flutter example.

## Prompt

```
I modified frb_example/flutter_via_create and want to run the integration tests locally.
What command should I use?
```

## Expected Behavior (with skill)

The agent should:
1. Recognize this is a Flutter example
2. Recommend `./frb_internal test-flutter-native --package frb_example/flutter_via_create`

## Baseline Documentation (what agent does WITHOUT skill)

**Result:** Agent found direct Flutter commands but MISSED the internal tool command.

**Commands suggested:**
1. Desktop: `flutter test -d macos integration_test`
2. Web: `flutter drive --driver=test_driver/integration_test.dart --target=integration_test/simple_test.dart -d web-server`
3. Mobile: `flutter test -d <device_id> integration_test`

**Assessment:**
- ❌ Did NOT mention `./frb_internal test-flutter-native --package frb_example/flutter_via_create`
- ✅ Found correct Flutter commands
- ⚠️ More verbose than necessary
- ⚠️ Missing the canonical internal tool approach

**This is a GAP in baseline behavior - the skill should help agents find the internal tool command.**

## Evaluation Criteria

- Correct command: `./frb_internal test-flutter-native --package frb_example/flutter_via_create` ❌ (MISSED)
- Understanding that Flutter uses different test command than Dart ✅ (found flutter commands)
