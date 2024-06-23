# Tricks

This page discusses some tricks that may be helpful when working with external crates.

## Wrappers

Sometimes, such as when needing to modify the third party struct definition itself,
the [wrappers](../manual/wrappers) approach can be useful.

## Injecting Dart code

The [`#[frb(dart_code)]`](../../misc-features/dart-code) feature can be utilized when we want to inject anything into the generated Dart code.
For example, we can add a method to an auto-generated Dart class.

## Using Rust macros

When we are going to write down code that contains a lot of duplication,
Rust macros can often be helpful.

## Ignore things

There can be things that are hard to automatically translate (e.g. complex lifetime specifiers, complex generics, ...),
and I suggest to put `#[frb(ignore)]` on them firstly to quickly get a working translation of the third party package.
After that, we can remove the `ignore`s and handle them (e.g. by [overriding methods](override-methods)).

## Using proxies

The [proxy](../../misc-features/proxy) feature can be utilized when we need to return a reference type,
especially when returning something like the reference to a struct field.
Please refer to that page for more details.
