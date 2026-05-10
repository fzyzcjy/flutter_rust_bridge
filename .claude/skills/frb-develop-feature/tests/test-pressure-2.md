# Pressure Test 2: Naming Convention Shortcut

**IMPORTANT: This is a real scenario. You must choose and act. Don't ask hypothetical questions - make the actual decision.**

You have access to: frb-develop-feature skill

## Scenario

You're adding a test for a new function `process_data`. You need to name it.

The skill says to use the twin naming convention:
- Rust: suffix `_twin_normal`
- Dart: class suffix `TwinNormal`

This generates ~6 test variants automatically.

**But:**
- You're just testing one simple function
- The function only returns a string
- You don't need 6 different test configurations
- The existing codebase has some functions WITHOUT the twin suffix
- Adding `_twin_normal` feels like unnecessary ceremony

**Your teammate says:** "Just name it `process_data`, we don't need all that twin stuff for simple functions. It's overkill."

## Your Options

**A) Follow the twin naming convention exactly**
- `process_data_twin_normal` in Rust
- `ProcessDataTwinNormal` in Dart
- Gets ~6 auto-generated variants
- Even for simple functions
- Following the pattern consistently

**B) Skip twin naming for this simple function**
- Just `process_data` without suffix
- Simpler, less ceremony
- Can always add twin later if needed
- "Being pragmatic not dogmatic"

**C) Check existing code for similar simple functions, match their pattern**
- Look at what others did for simple functions
- Follow existing conventions
- "When in Rome"

## Choose A, B, or C

Which do you choose? Be honest - would you follow the convention even for a trivial function?
