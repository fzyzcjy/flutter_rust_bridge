# Dart-only base
Create project from scratch or template?

Either:
```rust
/// Enforce the binding for this library (to prevent tree-shaking)
#[no_mangle]
pub extern "C" fn enforce_binding() {}
```
Or use the C generation FRB has? Try out C generation in mimir and write this based on that.
