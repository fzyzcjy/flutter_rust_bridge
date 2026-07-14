## Purpose

Verify that a Darwin Flutter app generated through flutter_rust_bridge initializes when CocoaPods is configured with static framework linkage.

This test is the manual reproduction for GitHub issue #3073. On an unfixed baseline, switching the app Podfile from `use_frameworks!` to `use_frameworks! :linkage => :static` reproduces the loader failure reported by users. The failure can appear as either a missing generated framework binary or, after falling back to `DynamicLibrary.process()`, a missing `frb_get_rust_content_hash` symbol.

## Scope

- Example app: `frb_example/flutter_via_integrate`
- Platforms: macOS app target, with the same CocoaPods linkage issue applying to iOS app targets
- Scenario: Flutter app target consumes the Rust pod through static CocoaPods linkage
- Related issue: #3073

## Prerequisites

- A clean flutter_rust_bridge checkout
- Submodules initialized
- The local Tart-based Apple development environment from `frb-dev-env`
- Xcode, CocoaPods, Flutter, Dart, and Rust available inside the Tart VM

Record these versions before running the test:

```sh
git rev-parse HEAD
git status --short
.claude/skills/frb-dev-env/frb_dev_env.py tart exec -- /bin/zsh -lc 'flutter --no-version-check --version'
.claude/skills/frb-dev-env/frb_dev_env.py tart exec -- /bin/zsh -lc 'dart --version'
.claude/skills/frb-dev-env/frb_dev_env.py tart exec -- /bin/zsh -lc 'rustc --version'
.claude/skills/frb-dev-env/frb_dev_env.py tart exec -- /bin/zsh -lc 'pod --version'
.claude/skills/frb-dev-env/frb_dev_env.py tart exec -- /bin/zsh -lc 'xcodebuild -version'
```

## Preparation

Start the Tart VM and sync the repository:

```sh
.claude/skills/frb-dev-env/frb_dev_env.py tart create
.claude/skills/frb-dev-env/frb_dev_env.py tart start --wait 300
.claude/skills/frb-dev-env/frb_dev_env.py tart upload
```

## Steps

First verify the example still passes with the default dynamic CocoaPods linkage:

```sh
.claude/skills/frb-dev-env/frb_dev_env.py tart exec --sync-code -- /bin/zsh -lc 'set -euo pipefail; cd frb_example/flutter_via_integrate; flutter --no-version-check clean; flutter --no-version-check pub get; flutter --no-version-check test integration_test/simple_test.dart --verbose --reporter=expanded --device-id macos'
```

Then rerun the same example after changing only the VM-local Podfile copy to static framework linkage:

```sh
.claude/skills/frb-dev-env/frb_dev_env.py tart exec --sync-code -- /bin/zsh -lc 'set -uo pipefail; cd frb_example/flutter_via_integrate; perl -0pi -e "s/use_frameworks!\n/use_frameworks! :linkage => :static\n/" macos/Podfile; flutter --no-version-check clean; flutter --no-version-check pub get; flutter --no-version-check test integration_test/simple_test.dart --verbose --reporter=expanded --device-id macos; rc=$?; if [ "$rc" -ne 0 ]; then echo "===== frb static linkage diagnostics ====="; find build/macos/Build/Products/Debug -name "librust_lib_flutter_via_integrate.a" -print; nm -gU build/macos/Build/Products/Debug/rust_lib_flutter_via_integrate/librust_lib_flutter_via_integrate.a | grep frb_get_rust_content_hash || true; nm -gU build/macos/Build/Products/Debug/flutter_via_integrate.app/Contents/MacOS/flutter_via_integrate.debug.dylib | grep frb_get_rust_content_hash || true; grep -R "OTHER_LDFLAGS\|force_load\|rust_lib_flutter_via_integrate" -n macos/Pods/Target\ Support\ Files/Pods-Runner macos/Pods/Target\ Support\ Files/rust_lib_flutter_via_integrate | head -n 80 || true; fi; exit "$rc"'
```

The `--sync-code` run operates on the VM-local repository copy. The host checkout should remain clean after the test:

```sh
git status --short
```

## Expected Result

For the fixed product behavior, both the default-linkage and static-linkage test commands exit with status 0 and the Flutter test reports success.

For an unfixed baseline, the default-linkage command should pass, while the static-linkage command should reproduce issue #3073 by failing during FRB initialization with one of these signatures:

```text
Failed to load dynamic library 'rust_lib_flutter_via_integrate.framework/rust_lib_flutter_via_integrate'
```

or:

```text
Failed to lookup symbol 'frb_get_rust_content_hash'
```

The symbol diagnostics should show `_frb_get_rust_content_hash` in the Rust static archive but missing from the final Flutter app debug dylib.

## Failure Criteria

- The default-linkage command fails. Treat this as an environment or unrelated example-app failure before using this test to diagnose static linkage.
- The static-linkage command fails with one of the expected signatures on an unfixed baseline. This is a successful reproduction of issue #3073, but still a product failure.
- The static-linkage command fails with either expected signature after applying the issue #3073 fix. Treat this as a regression.
- The host checkout is dirty after the test. Revert only the local manual-test Podfile edit before committing unrelated work.

## Results To Capture

- Full command log for both default-linkage and static-linkage runs
- Flutter, Dart, Rust, CocoaPods, and Xcode versions
- `git rev-parse HEAD`
- The static-linkage failure text
- `nm` output for `frb_get_rust_content_hash` in the Rust static archive and final app debug dylib
- Generated CocoaPods xcconfig linker flags for the Runner and Rust pod targets
