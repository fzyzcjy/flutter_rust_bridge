# Baseline Test: Debug Code Generation

**Run WITHOUT the skill loaded to establish baseline behavior.**

## Scenario

A developer says: "The generated Dart code doesn't look right. I want to see what intermediate representations FRB is using during code generation. How can I inspect the internal state?"

## Expected Behavior (with skill)

Agent should:
1. ✅ Mention `dump_all: true` configuration
2. ✅ Show correct dump location: `rust/target/frb_dump/`
3. ✅ Provide example code for enabling dump

## Baseline Result (without skill)

**Found:**
- ✅ Found `dump_all: true` configuration
- ✅ Found correct dump location
- ⚠️ Didn't show explicit code example for lib.rs configuration

## After Skill Loaded

**Improvements:**
- ✅ Clear step-by-step instructions
- ✅ Explicit code example: `flutter_rust_bridge::frb_generated_boilerplate!(dump_all: true);`
- ✅ Quick Reference table with dump location
