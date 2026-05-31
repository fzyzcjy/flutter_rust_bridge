---
name: frb-dev-env
description: "Use only if the user wants container-based development workflow."
---

# User FRB Environment

Use this skill before setting up, diagnosing, or running `flutter_rust_bridge` development commands in the user's environment.

## Principles

- FRB runs locally in Docker by default. Do not use remote workspaces for FRB unless the user explicitly asks for a one-off remote run.
- Use the current checkout/worktree that Codex or the user already selected. Do not create a new worktree unless the user explicitly asks.
- Run FRB commands from the repository root unless the command itself intentionally changes directories.
- Prefer repository tooling such as `./frb_internal` over ad hoc direct invocations.
- Do not manually edit generated files (`frb_generated.*`, `*.freezed.dart`, `*.g.dart`) as the final fix.

## First Checks

Before running tests, lint, code generation, or setup:

```bash
git rev-parse --show-toplevel
git status --short
git submodule status --recursive
```

If submodules are uninitialized, initialize them locally:

```bash
git submodule update --init --recursive
```

## Docker Container

Use Docker by default for the user's local FRB development. Each worktree should have one long-lived local container that is reused for tests, lint, code generation, and setup commands.

Use `frb_dev_env.py` next to this skill to inspect, create, start, and reuse the per-worktree container. The script derives the container name from the canonical worktree root only:

```text
frb-<worktree-root-sha256-prefix-12>
```

It also mounts the worktree at `/workspace` and labels the container with:

```text
tom.frb.repo=flutter_rust_bridge
tom.frb.worktree=<canonical worktree root>
```

Typical usage:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py info
.claude/skills/frb-dev-env/frb_dev_env.py create
.claude/skills/frb-dev-env/frb_dev_env.py exec -- bash -lc './frb_internal --help'
```

## Cleanup

Delete a worktree's Docker container when the worktree is no longer needed, or when local Docker resources are getting crowded:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py delete
```

The delete command validates the container labels before removing it. Use `--force` only when intentionally removing a mismatched container.

Use the project `frb-docker` skill for image, devcontainer, and Dockerfile details; this skill only defines the user's per-worktree container policy.

## Project Skills

After applying this personal environment policy, also read the relevant project-level `frb-*` skills for the actual task, such as code generation, tests, lint, Docker details, CI triage, or PR preparation.

This skill only captures the user's local environment policy and per-worktree Docker container policy.
