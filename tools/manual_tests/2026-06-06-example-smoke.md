# Basic Repository Smoke Test

## Purpose

Verify that a flutter_rust_bridge checkout is usable at the most basic level: the repository is clean, submodules are initialized, and the internal development entrypoint can start and print help.

## Source

- Context: Example manual test for the `frb-manual-test` workflow.
- Related docs or skills: `.claude/skills/frb-manual-test/SKILL.md`, `.claude/skills/frb-dev-env/SKILL.md`

## When To Run

Run this as a lightweight smoke test after cloning a new checkout, switching to a branch for manual work, changing development scripts, or preparing a machine before heavier FRB validation.

## Preconditions

- Repository: `fzyzcjy/flutter_rust_bridge`
- Required branch state: clean checkout with submodules initialized.
- Required credentials or account state: none.
- Required device or simulator state: none.

## Environment

- OS: macOS, Linux, or Windows with a shell capable of running repository scripts.
- Flutter: record `flutter --version` if Flutter is installed or required by later manual work.
- Dart: record `dart --version` if Dart is installed or required by later manual work.
- Rust: record `rustc --version` if Rust is installed or required by later manual work.
- Device or simulator: none.
- Browser or external service: none.

## Preparation

```bash
git submodule update --init --recursive
```

Run preparation commands from the repository root of the checkout being tested.

## Test Data

- Input files, API examples, account fixtures, or generated assets: none.
- Reset procedure before each run: return to a clean checkout or record any intentional local changes.

## Steps

1. Confirm the checkout has no unexpected local changes.

   ```bash
   git status --short
   ```

2. Confirm submodules are initialized.

   ```bash
   git submodule status --recursive
   ```

3. Confirm the internal FRB development entrypoint starts.

   ```bash
   ./frb_internal --help
   ```

## Expected Result

The smoke test passes when:

- `git status --short` prints no output, or only intentional local changes recorded by the executor.
- `git submodule status --recursive` prints initialized submodule entries and no entry starts with `-`.
- `./frb_internal --help` exits successfully and prints usage or command help text.

## Failure Criteria

The test fails if any of the following happens:

- The checkout has unexpected local changes.
- Any submodule entry is uninitialized.
- `./frb_internal --help` exits non-zero.
- `./frb_internal --help` does not print recognizable usage or command help text.

## Results To Capture

- Full terminal log for all commands.
- OS and shell used for the run.
- Any unexpected local changes or submodule lines.

## Troubleshooting

- If submodules are uninitialized, rerun `git submodule update --init --recursive` and record the output.
- If `./frb_internal --help` fails because toolchains are missing, record the missing command or error text and the local setup state.
- If the checkout is intentionally dirty, record the exact `git status --short` output and why it is expected.

## Cleanup

```bash
git status --short
```

Remove temporary files created during the run. This smoke test should not start long-lived services, modify devices, or change external account state.

## Future Automation

This smoke test can become automated as a lightweight checkout health check if the project wants a fast preflight separate from the full CI matrix.
