# Properties

Getters are currently supported.
If you need setters, feel free to create an issue.

It is often reasonable to use together with `sync` to create a sync Dart function.

## Example

```rust
pub struct A { ... }

impl A {
    #[frb(sync, getter)]
    pub fn something(&self) -> String { ... }
}
```

It will provide the following getter automatically:

```dart
class A {
    String get something { ... }
    ...
}
```