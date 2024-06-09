# Tricks

This page discusses some tricks that may be helpful when working with external crates.

## Injecting Dart code

The [`#[frb(dart_code)]`](../miscellaneous/dart-code) feature can be utilized when we want to inject anything into the generated Dart code.
For example, we can add a method to an auto-generated Dart class.

## Using Rust macros

When we are going to write down code that contains a lot of duplication,
Rust macros can often be helpful.
