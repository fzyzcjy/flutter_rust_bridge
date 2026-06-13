---
name: frb-publish-release
description: "Publish a flutter_rust_bridge release end to end: preflight checks, changelog preparation, frb_internal release publishing, released-version polling, CI babysitting, and post-release CI verification."
---

# FRB Publish Release

Use this skill when preparing, publishing, or babysitting a `flutter_rust_bridge` release.

## Workflow

### 1. Preflight

- Work from the repository root on the intended release branch, normally fresh `master`.
- Check `git status --short --branch` and do not start publishing from a dirty tree.
- Confirm the target version in `CHANGELOG.md`, root `Cargo.toml`, and `frb_dart/pubspec.yaml`.
- Compute the release versions the same way `./frb_internal release` does: the top `CHANGELOG.md` version is the new version and the next release section is the old version.
- Verify both old and new versions are legal before running any mutating release command. The only allowed shapes are stable SemVer `MAJOR.MINOR.PATCH` such as `2.0.0`, or beta SemVer `MAJOR.MINOR.PATCH-beta.N` such as `2.0.0-beta.1`. Use exactly `^\d+\.\d+\.\d+(-beta\.\d+)?$`.
- Reject versions with any other prerelease label, build metadata, missing numeric components, leading `v`, or loose text. Stop if either old or new version fails this check.
- Confirm the new version is different from the old version.
- Run the publish container credential preflight before starting irreversible publish steps:

  ```bash
  .claude/skills/frb-dev-env/frb_dev_env.py docker-run-rm --with-publish-credentials -- true
  ```

  Stop if the preflight fails. It checks GitHub CLI auth, Cargo credentials, and Dart pub credentials inside the same temporary credential layout used by release publishing.
- Confirm normal CI is green for the release commit before publishing. This is the default hard gate.

  A narrow exception is allowed only when all of these are true:

  - A recent earlier commit on the same release branch has fully green normal CI.
  - Every commit between that green commit and the intended release commit is unrelated to the publishable packages, version sources, generated release artifacts, and `frb_internal` release logic.
  - The mechanical gate script below exits successfully and its output is recorded in the release notes or journal.
  - The agent explicitly states that it is using the exception before publishing.

  Run the mechanical gate from the repository root:

  ```bash
  uv run --script .claude/skills/frb-publish-release/release_ci_gate.py \
    --base-green-ref <LAST_GREEN_SHA> \
    --release-ref HEAD
  ```

  Stop and wait for normal CI if the script reports any `BLOCK` path. The script is intentionally conservative: release-surface paths such as `Cargo.toml`, `frb_dart/pubspec.yaml`, `frb_codegen/**`, `frb_macros/**`, `frb_rust/**`, `frb_dart/**`, `frb_utils/**`, `frb_example/**`, `tools/frb_internal/**`, `pubspec.yaml`, `melos.yaml`, and lockfiles are not in the allowlist. `CHANGELOG.md` is allowed because adding the target release section is a normal release-preparation step and is reviewed directly before publishing. This exception is for changelog-only release preparation, docs, agent tooling, devcontainer, selected CI configuration, and other explicitly non-release paths only.

### 2. Write Changelog

- Use `frb-write-changelog` to create or refresh the target release section in `CHANGELOG.md`.
- Review the release section manually before publishing. The top `CHANGELOG.md` version is the source used by `frb_internal release`.
- If changelog or version files changed, commit that release preparation before publishing.

### 3. Publish

Use the repository release command through a temporary Docker container with publish credentials as the normal publishing path:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py docker-run-rm --with-publish-credentials -- ./frb_internal release
```

Do not split the normal release into separate `release-update-*` or publish commands. The one-shot command is the source of truth for release sequencing: it computes old/new versions from `CHANGELOG.md`, updates checked-in versions and generated version text, updates Scoop metadata, commits and pushes the version bump, creates a draft GitHub release, then runs the registry publisher:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py docker-run-rm --with-publish-credentials -- ./frb_internal release-publish-all
```

`release-publish-all` publishes these packages:

- `frb_codegen` -> crates.io package `flutter_rust_bridge_codegen`
- `frb_macros` -> crates.io package `flutter_rust_bridge_macros`
- `frb_rust` -> crates.io package `flutter_rust_bridge`
- `frb_dart` -> pub.dev package `flutter_rust_bridge`

Only use a split subcommand as a recovery path after confirming which one-shot step already completed. For example, if the version bump and GitHub draft already exist and only registry publication is needed, use `.claude/skills/frb-dev-env/frb_dev_env.py docker-run-rm --with-publish-credentials -- ./frb_internal release-publish-all` directly.

### 4. Check Released Versions

Poll registry state with:

```bash
./frb_internal get-released-version
```

To check a target version from a checkout whose manifests have not been bumped, pass it explicitly:

```bash
./frb_internal get-released-version --version <VERSION>
```

The command prints JSON:

```json
{
  "allReleased": true,
  "packages": [
    {
      "registry": "crates.io",
      "name": "flutter_rust_bridge_codegen",
      "manifestVersion": "2.12.0",
      "releasedVersion": "2.12.0",
      "isReleased": true
    }
  ]
}
```

Wait until every package has `isReleased: true` and `allReleased: true`. If one registry lags, keep polling with bounded waits and record the mismatched package instead of assuming the publish failed. Do not add the target version option to the mutating `release-*` subcommands; they derive their version from `CHANGELOG.md` and checked-in manifests.

For beta releases, `get-released-version --version <VERSION>` must verify the pub.dev `versions` list, not only pub.dev's `latest` field, because pub.dev keeps `latest` on the latest stable release when a prerelease is uploaded.

### 5. Babysit CI And Post-Release CI

- Keep watching the release commit's normal CI until it is green.
- After `./frb_internal get-released-version` reports `allReleased: true`, trigger `.github/workflows/post_release.yaml` for the release commit or `master`.
- Babysit post-release CI until it is green. Use `gh-actions-live-logs` when reading GitHub Actions logs.
- If post-release fails, classify the failure by release channel (`stable` or `unstable`) and install mode (`cargo-install`, `cargo-binstall`, `scoop`, or `homebrew`) before changing code or rerunning.

## Related Skills

- `frb-write-changelog` for the release section.
- `frb-fix-ci` or `frb-fix-main-ci` for CI failures.
- `gh-actions-live-logs` for GitHub Actions logs.
