# Extra Dart code

Arbitrary extra Dart code can be inserted into auto-generated Dart classes.

## Example

```rust
#[frb(dart_code = "
    int extraMethod() => a * 2;
"
)]
pub struct MyStruct {
    ...
}
```

Then the generated Dart class will look like:

```dart
class MyStruct {
  ... // other auto-generated code
  
  int extraMethod() => a * 2; // The extra code
}
```
