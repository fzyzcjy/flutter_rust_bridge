---
name: frb-tart-prepare
description: Use when preparing or explaining the FRB Tart base VM for iOS Simulator validation, including local OCI registry mirroring, Packer provisioning, and base VM hygiene.
---

# FRB Tart Base VM Preparation

Use this skill when preparing or explaining the reusable Tart base VM used by `frb-dev-env tart ...`.

## Source Of Truth

Read the checked-in guide first:

```text
.claude/skills/frb-tart-prepare/README.md
```

That README owns the detailed Tart source image, local OCI registry, Packer build, smoke test, promotion, cost, and base hygiene instructions. Keep this skill thin so the guide remains the audited source of truth for the Tart base VM workflow.

## Required Reminders

- `frb-tart-base` is immutable after promotion; do not boot it and manually install tools into it.
- Durable base image changes belong in `.claude/skills/frb-tart-prepare/packer/frb-tart-base.pkr.hcl` or its provision scripts.
- Use `frb-tart-base-candidate` for rebuilds, smoke test the candidate, then promote it to `frb-tart-base`.
- For daily per-worktree Tart usage, read the Tart section inside `.claude/skills/frb-dev-env/SKILL.md`.
