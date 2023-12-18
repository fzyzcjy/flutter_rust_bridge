# Overview

There are basically two categories when it comes to types:

Firstly, Rust and Dart types, even if they are not encodable, not cloneable,
require a handle of native resources, etc, are supported.
Shortly speaking, they will become "opaque" handles in the other side (imagine as fancy pointers).
You can use them as function arguments, return values, call methods on them, etc.
This is the [arbitrary types](arbitrary).

Secondly, a lot of types (even structs, enums, ...) can be further translated into Dart native types.
For example, suppose you return a Rust struct of type `struct A { name: String, children: Vec<A> }`.
Then, the Dart side will receive a normal Dart object of type `class A { String name; List<A> children; }`,
with all values properly translated,
just like any other Dart object and classes we see everyday.
This is discussed in the [translatable types](translatable) section.
