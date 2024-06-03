# Tips and procedures

## How to add a test

:::info
This package has some scripts to automatically create more tests based on the test you write,
so you write one test and get (usually) six tests ;)

That's why adding a test seems slightly more complex than usual - but surely quicker than manually writing 6 tests.
:::

1. In `./frb_example/pure_dart`,
add your test in (for example) `rust/src/api/whatever.rs` and `test/api/whatever_test.dart`.
2. Remember to mimic existing tests and add `_twin_normal`/`TwinNormal`/etc as appropriate (this allows more "twin" tests to be generated).
3. Run `./frb_internal precommit --mode slow`, or if that's too slow,
run `./frb_internal generate-internal-frb-example-pure-dart && ./frb_internal generate-run-frb-codegen-command-generate --package frb_example/pure_dart`

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

### Use `frb_example/dart_minimal` as testing bed

A lot of tests reside in `frb_example/pure_dart`, thus it is a bit big and slow to compile and examine.
Therefore, I (personally) often use `frb_example/dart_minimal` as a testing bed.

For example, I will ad-hoc add one or two functions to dart_minimal when wanting to examine outputs and behavior.

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
