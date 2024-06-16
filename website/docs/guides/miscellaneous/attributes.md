# Attributes

## Specifying attributes

There are two approaches to specify an attribute:

1. Attribute-based: `#[frb(something)]`
2. Comment-based: `/// frb:something`

The latter is especially useful when the former cannot be used,
such as when the target is a `mod`, or when the crate does not have dependency on `flutter_rust_bridge`.

Most of the time, the latter is equivalent to the former;
but for things like `#[frb(external)]`, which has to act as a macro to generate some code,
the latter cannot be used.
But this can be easily spotted since it will have compile-time messages.

## Full list of attributes

For completeness, all attributes are listed below.
To know more details of each attribute, the search bar at top-right may be useful.

* `#[frb(mirror)]`
* `#[frb(non_final)]`
* `#[frb(sync)]`
* `#[frb(stream_dart_await)]`
* `#[frb(getter)]`
* `#[frb(init)]`
* `#[frb(ignore)]`
* `#[frb(opaque)]`
* `#[frb(non_opaque)]`
* `#[frb(non_hash)]`
* `#[frb(non_eq)]`
* `#[frb(rust_opaque_codec_moi)]`
* `#[frb(serialize)]`
* `#[frb(semi_serialize)]`
* `#[frb(metadata)]`
* `#[frb(default)]`
* `#[frb(dart_code)]`

(This list may be outdated, if so feel free to ping me; the source of truth is in `attribute_parser.rs`)
