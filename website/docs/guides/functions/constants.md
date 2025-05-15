# Constants

If this feature is enabled,
Rust constants will be automatically translated as well.
Same as functions, it needs to be `pub` to be included.

In order to enable this feature, please specify `parse_const: true` in the `flutter_rust_bridge.yaml` configuration file.

## Example

```rust
pub const CONST_INT: i32 = 42;
```

It will provide the following getter automatically:

```dart
int get constIntTwinNormal => ...;
```
