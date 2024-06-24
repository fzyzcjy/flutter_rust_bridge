# Full list of attributes

The following are by alphabetical order instead of importance.
For example, seldomly used feature may appear near the top.

* `#[frb(dart2rust(..))]`: Custom encoders/decoders.
* `#[frb(dart_code = ..)]`: Inject extra Dart code.
* `#[frb(default = ..)]`: Set default parameters.
* `#[frb(external)]`: Mark external methods.
* `#[frb(getter)]`: Mark function as Dart getter.
* `#[frb(ignore)]`: Ignore the object annotated.
* `#[frb(init)]`: Mark function to be executed at startup.
* `#[frb(mirror)]`: Manually mirror external types (can use auto mode instead).
* `#[frb(name)]`: Rename the object.
* `#[frb(non_eq)]`: Disable generating `equals`.
* `#[frb(non_hash)]`: Disable generating `hashCode`.
* `#[frb(non_opaque)]`: Mark object as non-opaque.
* `#[frb(opaque)]`: Mark object as opaque.
* `#[frb(positional)]`: Generate positional instead of keyword arguments.
* `#[frb(proxy)]`: Enable proxy feature.
* `#[frb(rust2dart)]`: Custom encoders/decoders.
* `#[frb(setter)]`: Mark function as Dart setter.
* `#[frb(serialize)]`: Use SSE codec.
* `#[frb(stream_dart_await)]`: Await stream execution before returning.
* `#[frb(sync)]`: Generate synchronous function in Dart.
* `#[frb(type_64bit_int)]`: Change how 64-bit integers are translated.

For a up-to-date full list of supported attributes, please refer to the `FrbAttribute`
of [this file](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_codegen/src/library/codegen/parser/mir/parser/attribute.rs)
(except that it is pascal case instead of snake case).
To know more details of each attribute, the search bar at top-right may be useful.
