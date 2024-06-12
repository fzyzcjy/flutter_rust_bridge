# Custom encoder/decoders

We can customize how a type is encoded and decoded.

## Example

Suppose we have a `FancyRustType` and corresponding type `FancyDartType`
(they can be the same name, here we use different names just to make it clearer).
If we want to encode it using a `String` (can use arbitrary complex types as long as flutter_rust_bridge supports),
then we can write code like below:

```rust
#[frb(rust2dart(dart_type = "FancyDartType", dart_code = "FancyDartType.letsParseIt({})"))]
pub fn encode_fancy_type(raw: FancyRustType) -> String { ... }

#[frb(dart2rust(dart_type = "FancyDartType", dart_code = "{}.letsEncodeIt()"))]
pub fn decode_fancy_type(raw: String) -> FancyRustType { ... }
```

The function names above are arbitrarily chosen.
Then, whenever we are using `FancyType`, such as:

```rust
pub fn f(a: FancyRustType) { ... }
```

It will be automatically converted to:

```dart
void f(FancyDartType a) { ... }
```

And under the hood, the type will be encoded/decoded via the custom functions.

## Remarks

* If the Dart types need some `import`s to work, the `dart_preamble` config key in `flutter_rust_bridge.yaml` can be utilized to import things.
* If the encoding/decoding process returns a `Result`, currently you can `unwrap()` it to convert it to a panic.
