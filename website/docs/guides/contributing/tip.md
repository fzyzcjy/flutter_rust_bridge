# Misc Tips

## AI Assistant Skills

For common development workflows, see the skills in `.claude/skills/`.

## How to solve failed CI

* **When it is merely flaky**: Sometimes it is just flaky, not real bug. Thus, try to re-run the CI jobs
(e.g. by pushing a commit that does no real work in order to trigger the CI) and see.
* **Quickly solve git-diff errors**: If the CI is unhappy and provide you git diffs,
you can either run code generator locally, or directly *copy-and-paste* and `git apply` those diffs to fix your code.
* **How to reproduce locally**: Operations are usually reproducible using corresponding `./frb_internal something` commands shown in CI.
* **Automatic fix commands**: For many `./frb_internal` commands, there is a `--fix` option which tries to automatically fix everything.

## Tips for development

### The `./frb_internal`

The `./frb_internal whatever-command` (or `./frb_internal.bat`) delegates to the `./tools/frb_internal` dart package.
It contains all scripts to work on flutter_rust_bridge development.
It as a similar role as [justfile](https://github.com/casey/just/blob/master/justfile), makefile, etc.
For example, `./frb_internal precommit --mode fast` (or `--mode slow`) runs code generator, formatter, etc for you.

### The `just codegen`

To run the `flutter_rust_bridge_codegen`, but using the local code (instead of a released version),
please replace it with `just codegen`.

### Check the dumped data

With `dump_all: true` in the config (which is already done for `frb_example/pure_dart`),
many things are dumped into the `rust/target/frb_dump/` folder,
which is often helpful for debugging the codegen.
For example, you can check the effective configuration, the IR (intermediate representation),
the generated spec, the generated code, etc.

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
