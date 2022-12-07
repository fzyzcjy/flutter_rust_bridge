# Dart opaque type safety


## Ownership and GC

From the moment the opaque type is passed, Rust will own a persistent representation of the dart object (`Dart_PersistentHandle` or `JsValue`).
This means that while Rust owns `DartOpaque` the object will not be cleared by GC.
Also flutter_rust_bridge provides a thread-safe drop for `DartOpaque`: Rust delegates the drop to the Dart side using the Dart port.


## Dispose flutter_rust_bridge Api before all `DartOpaques` are cleaned.

If there is an attempt to delegate the drop to the Dart side after the drop port (Api.dispose()) has been closed. flutter_rust_bridge will issue a warning in the logs, the memory behind the object will leak.


## Example

### Case 1: loopBack. 

Rust `api.rs`:
```rust,noplayground
pub fn loop_back(opaque: DartOpaque) -> DartOpaque {
    opaque
}
```

Dart:
```dart

String f() => 'Test_String';

var fn = await api.loopBack(opaque: f) as String Function();

expect(fn(), 'Test_String');
```


### Case 2: drop.

Rust `api.rs`:
```rust,noplayground
pub fn sync_accept_dart_opaque(opaque: DartOpaque) -> SyncReturn<String> {
    drop(opaque);
    SyncReturn("test".to_owned())
}

pub fn async_accept_dart_opaque(opaque: DartOpaque) {
    drop(opaque);
}
```

Dart:
```dart
// the closure is safely removed on the Rust side (on another thread)
await api.asyncAcceptDartOpaque(opaque: () => 'Test_String');
// the closure is safely removed on the Rust side (on current thread)
api.syncAcceptDartOpaque(opaque: () => 'Test_String');
```


### Case 3: Unwrap.

Rust `api.rs`:
```rust,noplayground
/// [DartWrapObject] can be safely retrieved on a dart thread.
pub fn unwrap_dart_opaque(opaque: DartOpaque) -> SyncReturn<String> {
    let handle = opaque.try_unwrap().unwrap();
    SyncReturn("Test".to_owned())
}

/// [DartWrapObject] cannot be obtained 
/// on a thread other than the thread it was created on.
pub fn panic_unwrap_dart_opaque(opaque: DartOpaque) {
    let handle = opaque.try_unwrap().unwrap();
}
```

Dart:
```dart

// Rust gets (drop safely) wrap Dart_PersistentHandler (or JsValue).
api.unwrapDartOpaque(opaque: () => 'Test_String');

// We get an error because DartOpaque was passed to another thread.
await expectLater(() => api.panicUnwrapDartOpaque(opaque: () => 'Test_String'), throwsA(isA<FfiException>()));
```