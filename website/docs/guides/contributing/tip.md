# Tips for development

## The `./frb_internal`

The `./frb_internal whatever-command` (or `./frb_internal.bat`) delegates to the `./tools/frb_internal` dart package.
It contains all scripts to work on flutter_rust_bridge development.
It as a similar role as [justfile](https://github.com/casey/just/blob/master/justfile), makefile, etc.
For example, `./frb_internal precommit --fast` (or `--slow`) runs code generator, formatter, etc for you.

## The `just codegen`

To run the `flutter_rust_bridge_codegen`, but using the local code (instead of a released version),
please replace it with `just codegen`.

## Reproduce CI failures

As is seen in [the CI workflow definitions](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/.github/workflows/ci.yaml),
most CI jobs are nothing but a `./frb_internal some-command`.
Therefore, to debug and fix CI failures, you can often run that command locally.

## Check the dumped data

With `dump_all: true` in the config (which is already done for `frb_example/pure_dart`),
many things are dumped into the `rust/target/frb_dump/` folder,
which is often helpful for debugging the codegen.
For example, you can check the effective configuration, the IR (intermediate representation),
the generated spec, the generated code, etc.

## Use `frb_example/dart_minimal` as testing bed

A lot of tests reside in `frb_example/pure_dart`, thus it is a bit big and slow to compile and examine.
Therefore, I (personally) often use `frb_example/dart_minimal` as a testing bed.

For example, I will ad-hoc add one or two functions to dart_minimal when wanting to examine outputs and behavior.
