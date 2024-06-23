# Preambles

We can put arbitrary extra Dart or Rust code in the generated files. This feature is called Dart/Rust preamble.

## Typical scenarios

This can be useful, for example:

* Import some extra things in the generate code. (e.g. `rust_preamble: "use some::Thing;"`)
* Suppress some rules of the linter. (e.g. `dart_preamble: "// ignore_for_file: this_is_a_lint_rule"`)

## Tricks

* In YAML, it is allowed to write down multi-line string literal (see example below)

## Example

Suppose we have this in `flutter_rust_bridge.yaml`:

```yaml
dart_preamble: |
  // ignore_for_file: this_is_a_lint_rule
  whatever_thing_here;
rust_preamble: |
  use some::Thing;
  also_whatever_thing_here;
```

Then the generated files will have such code pasted as-is on the top.
