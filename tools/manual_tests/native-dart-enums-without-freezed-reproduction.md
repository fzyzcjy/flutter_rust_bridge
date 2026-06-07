# Native Dart Enums Without Freezed Reproduction

## Purpose

Verify the current generated Dart API shape for Rust enums with associated data. This manual test captures that complex Rust enums are generated as Freezed-backed Dart sealed classes, including a `.freezed.dart` part, rather than native Dart 3 sealed class subclasses.

## Source

- Context: GitHub discussion #3117, "Native Dart 3 Sealed Classes Without Freezed for Rust Enums in FRB v2"
- Related docs or skills: `frb-issue-to-green-pr`, `frb-manual-test`

## When To Run

Run this when validating discussion #3117 or when changing Dart API generation for Rust enums with associated data.

## Preconditions

- Repository: `fzyzcjy/flutter_rust_bridge`
- Required checkout state: clean checkout with submodules initialized and generated example files present
- Required credentials or account state: not required
- Required device or simulator state: not required

## Environment

- OS: not constrained
- Flutter: not required for inspection-only reproduction
- Dart: not required for inspection-only reproduction
- Rust: not required for inspection-only reproduction
- Device or simulator: not required
- Browser or external service: not required

## Preparation

```bash
git submodule update --init --recursive
git status --short
```

## Test Data

- Input files, API examples, account fixtures, or generated assets: `frb_example/pure_dart/lib/src/rust/api/enumeration.dart` and `frb_example/pure_dart/lib/src/rust/api/enumeration.freezed.dart`
- Reset procedure before each run: start from a clean checkout

## Steps

1. Confirm the generated Dart API file imports Freezed and declares the Freezed part file.

   ```bash
   rg -n "freezed_annotation|part 'enumeration.freezed.dart'" frb_example/pure_dart/lib/src/rust/api/enumeration.dart
   ```

2. Confirm a complex enum is emitted as a Freezed-backed sealed class.

   ```bash
   rg -n "@freezed|sealed class EnumWithItemMixedTwinNormal with \\_\\$EnumWithItemMixedTwinNormal" frb_example/pure_dart/lib/src/rust/api/enumeration.dart
   ```

3. Confirm the Freezed-generated implementation file exists and contains generated implementation classes for the enum variants.

   ```bash
   rg -n "class _\\$EnumWithItemMixedTwinNormal_BImpl|abstract class EnumWithItemMixedTwinNormal_B" frb_example/pure_dart/lib/src/rust/api/enumeration.freezed.dart
   ```

## Expected Result

Each command exits successfully and prints matching lines. The matches show that the current behavior requires Freezed for complex enum Dart API generation.

```text
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'enumeration.freezed.dart';
@freezed
sealed class EnumWithItemMixedTwinNormal with _$EnumWithItemMixedTwinNormal
class _$EnumWithItemMixedTwinNormal_BImpl
```

## Failure Criteria

The test fails if any of the following happens:

- The commands cannot run from the repository root.
- `enumeration.dart` no longer contains Freezed import, part, or `@freezed` complex enum class output.
- `enumeration.freezed.dart` no longer contains generated implementation classes for the complex enum variants.

## Results To Capture

- Full terminal log from the commands.
- Current branch and commit.
- Final `git status --short` output.

## Troubleshooting

- If the generated files are absent or stale, run the repository generation workflow for `frb_example/pure_dart` and record the exact command and failure if generation is blocked.
- If `rg` is unavailable, use an equivalent text search command and record the replacement.

## Cleanup

```bash
git status --short
```

No device, service, or account cleanup is required.

## Future Automation

This scenario should become an automated regression once native Dart 3 sealed enum generation is implemented, using generated `pure_dart` and `pure_dart_pde` coverage instead of this inspection-only reproduction.
