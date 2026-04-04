# Misc Tips

## AI Assistant Skills

For common development workflows, see the skills in `.claude/skills/`.

## Tips for development

### Docker and devcontainer

Optionally, you can use Docker and/or devcontainer to quickly get a standardized development environment. The related code is in `.devcontainer`.

The published development image is `fzyzcjy/flutter_rust_bridge_dev`. It packages the Flutter + Rust toolchain used for flutter_rust_bridge development, including the nightly toolchain, `wasm-pack`, and Chromium for web testing.

Prefer the full version tag when you want reproducible environments:

```shell
docker run --rm -it fzyzcjy/flutter_rust_bridge_dev:flutter-3.41.2-rust-1.93.1-nightly-2025-02-01 bash
```

Those version numbers are derived from the `ARG` values in `.devcontainer/Dockerfile`, which is the single source of truth. `latest` only means the current recommended image and is not intended for long-term reproducibility.

The default devcontainer configuration still builds locally from `.devcontainer/Dockerfile`. If you want to use the prebuilt image manually, use the full version tag above or regenerate it from the current Dockerfile args after version bumps.

### The `./frb_internal`

The `./frb_internal whatever-command` (or `./frb_internal.bat`) delegates to the `./tools/frb_internal` dart package.
It contains all scripts to work on flutter_rust_bridge development.
It as a similar role as [justfile](https://github.com/casey/just/blob/master/justfile), makefile, etc.
For example, `./frb_internal precommit --mode fast` (or `--mode slow`) runs code generator, formatter, etc for you.

### CI autofix for generated changes

If a pull request fails because generated or normalized files were not committed, the `Precommit Autofix` workflow can prepare a patch artifact for you in the standardized dev image.

Open the workflow run, download the `precommit-autofix-diff` artifact, and apply it locally:

```shell
gh run download <run-id> -n precommit-autofix-diff
git apply precommit-autofix.diff
git commit -am "Apply precommit autofix"
git push
```

The workflow summary prints the exact command sequence for that run. The workflow never pushes commits on your behalf; it only prepares a patch for review and application.

### The `just codegen`

To run the `flutter_rust_bridge_codegen`, but using the local code (instead of a released version),
please replace it with `just codegen`.

## Draw a flamegraph for performance

If you are not working on improving performance, please ignore this subsection.

I have made some small scripts for me to get flamegraph on MacOS.
The scripts contain absolute paths, extra pieces for MacOS to work, etc.
So you can look at the source code and modify to suit your needs.

To execute it:

```shell
./frb_internal bench-flamegraph-compile
./frb_internal bench-flamegraph-run --filter 'VoidFunction.*FrbCstSse.*false' --loop-count 10000000
```

## Debug in Safari (or other browsers)

Given [this Flutter issue](https://github.com/flutter/flutter/issues/55323),
it seems that we cannot use things similar to `flutter run -d chrome` for Safari.
Here is a brief command to achieve similar results.

Note that we need `--profile` to get stack traces.

```shell
(cd frb_example/flutter_via_create && just codegen build-web && flutter build web --profile)
(cd frb_utils && dart run flutter_rust_bridge_utils serve-web --web-root ../frb_example/flutter_via_create/build/web)
```

## Print missing stack traces

This script quickly print out every uncaught exception with stack traces.
For example, when working on Safari, the stack traces are missing without this.

```dart
Future<void> main() async {
  FlutterError.onError = (details) {
    FlutterError.presentError(details);
    print('FlutterError.onError $details');
  };
  PlatformDispatcher.instance.onError = (error, stack) {
    print('PlatformDispatcher.instance.onError $error $stack');
    return true;
  };
  await runZonedGuarded(
    () async {
      YOUR_ORIGINAL_CODE_HERE;
    },
    (error, stackTrace) => print('runZonedGuarded error $error $stackTrace'),
  );
}
```
