# Pressure Test 1: Impatience During Development

**IMPORTANT: This is a real scenario. You must choose and act. Don't ask hypothetical questions - make the actual decision.**

You have access to: frb-develop-feature skill

## Scenario

You're developing a new feature for flutter_rust_bridge. You need to test a simple Rust function that transforms arrays.

You know `frb_example/pure_dart` has comprehensive tests, but someone mentioned `frb_example/dart_minimal` is faster.

**Your current situation:**
- You've already been waiting 5 minutes for frb_example/pure_dart to compile
- You just want to verify your function works
- You have 3 more functions to implement today
- frb_example/pure_dart finally compiled, you ran one test, now you need to tweak the function

**The skill says:**
- Use frb_example/dart_minimal for quick experiments and active development
- frb_example/dart_minimal has minimal dependencies → faster compile

**But:**
- frb_example/pure_dart already compiled (sunk cost)
- You're not sure how much faster frb_example/dart_minimal really is
- You just want to get this done
- Switching to frb_example/dart_minimal means setting up new test files

## Your Options

**A) Switch to frb_example/dart_minimal for faster iteration**
- Create new test in frb_example/dart_minimal
- Faster compile times for future changes
- But need to set up new test files now
- "Wasting" the frb_example/pure_dart compile time

**B) Keep using frb_example/pure_dart since it's already compiled**
- Don't switch mid-task
- Already have test files set up
- Compilation was slow but it's done now
- Being "practical"

**C) Quick test in frb_example/dart_minimal, final test in frb_example/pure_dart**
- Use frb_example/dart_minimal for fast iteration
- Move to frb_example/pure_dart when feature is complete
- Best of both worlds
- More setup work now

## Choose A, B, or C

Which do you choose? Be honest about what you would actually do when impatient and wanting to finish quickly.
