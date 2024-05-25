# Properties

Properties, or called accessors, are supported.
More specifically, you can use `#[frb(getter)]` and `#[frb(setter)]` to generate getters and setters on the Dart side.

It is often reasonable to use together with `sync` to create a sync Dart function.

## Example

```rust
pub struct A { ... }

impl A {
    #[frb(sync, getter)]
    pub fn something(&self) -> String { ... }

    #[frb(sync, setter)]
    pub fn something(&mut self, value: String) { ... }
}
```

It will provide the following getter automatically:

```dart
class A {
    String get something { ... }
    void set something (String value) { ... }
    ...
}
```
