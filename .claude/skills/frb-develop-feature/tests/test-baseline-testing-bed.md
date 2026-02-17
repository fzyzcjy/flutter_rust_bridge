# Baseline Test: Testing Bed Selection

**Run WITHOUT the skill loaded to establish baseline behavior.**

## Scenario

A developer says: "I need to test a new function I'm adding to the Rust side. The function handles array transformation. Where should I put the test and how do I run it?"

The codebase has:
- `frb_example/pure_dart` - comprehensive test suite
- `frb_example/dart_minimal` - minimal example

## Expected Behavior (with skill)

Agent should:
1. ✅ Mention both `dart_minimal` and `pure_dart` as options
2. ✅ Explain WHEN to use each (compile speed difference)
3. ✅ Recommend `dart_minimal` for quick experiments during development
4. ✅ Show both fast and full generation commands
5. ✅ Explain twin naming convention with WHY (~6 auto-generated tests)

## Baseline Result (without skill)

**Gaps identified:**
- ⚠️ Didn't emphasize compile speed difference strongly
- ⚠️ Didn't explain WHY twin naming generates multiple tests
- ⚠️ Used `./frb_internal precommit-generate` without mentioning faster alternative

## After Skill Loaded

**Improvements:**
- ✅ Clear guidance on when to use each testing bed
- ✅ Quick Reference table for easy scanning
- ✅ Explicit WHY for twin naming (auto-generates ~6 test variants)
- ✅ Both fast and full generation commands shown
