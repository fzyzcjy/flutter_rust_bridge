---
name: frb-feature-flag
description: "Use when adding or testing flutter_rust_bridge feature flags, especially flags that require coverage for both enabled and disabled states or should have package-level config plus item-level #[frb(...)] overrides."
---

# FRB Feature Flags

Use this skill when adding a feature flag, changing a feature flag, or writing coverage for a feature flag.

## API Surface

For feature flags requiring coverage for both enabled and disabled states:

- Expose the flag in `flutter_rust_bridge.yaml`.
- Expose the flag in the command-line interface.
- Prefer adding an item-level `#[frb(...)]` override when the behavior can reasonably be scoped to a Rust item.

## Test Coverage

In `frb_example/pure_dart`, prefer tests that use the item-level `#[frb(...)]` override so enabled and disabled cases are both exercised in the same package.

Do not rely only on package-level config differences when both states can be covered clearly with item-level `#[frb(...)]` overrides.
