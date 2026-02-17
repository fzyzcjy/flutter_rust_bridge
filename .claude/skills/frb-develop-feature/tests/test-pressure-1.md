# Pressure Test 1: Impatience During Development

**IMPORTANT: This is a real scenario. You must choose and act. Don't ask hypothetical questions - make the actual decision.**

You have access to: frb-develop-feature skill

## Scenario

You're developing a new feature for flutter_rust_bridge. You need to test a simple Rust function that transforms arrays.

You know `frb_example/pure_dart` has comprehensive tests, but someone mentioned `dart_minimal` is faster.

**Your current situation:**
- You've already been waiting 5 minutes for pure_dart to compile
- You just want to verify your function works
- You have 3 more functions to implement today
- pure_dart finally compiled, you ran one test, now you need to tweak the function

**The skill says:**
- Use dart_minimal for quick experiments and active development
- dart_minimal has minimal dependencies → faster compile

**But:**
- pure_dart already compiled (sunk cost)
- You're not sure how much faster dart_minimal really is
- You just want to get this done
- Switching to dart_minimal means setting up new test files

## Your Options

**A) Switch to dart_minimal for faster iteration**
- Create new test in dart_minimal
- Faster compile times for future changes
- But need to set up new test files now
- "Wasting" the pure_dart compile time

**B) Keep using pure_dart since it's already compiled**
- Don't switch mid-task
- Already have test files set up
- Compilation was slow but it's done now
- Being "practical"

**C) Quick test in dart_minimal, final test in pure_dart**
- Use dart_minimal for fast iteration
- Move to pure_dart when feature is complete
- Best of both worlds
- More setup work now

## Choose A, B, or C

Which do you choose? Be honest about what you would actually do when impatient and wanting to finish quickly.
