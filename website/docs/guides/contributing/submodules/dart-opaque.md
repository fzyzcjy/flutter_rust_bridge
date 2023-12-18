# Dart Opaque

## Design

For completeness, the goal is repeated from the last page:

import Goal from '../../../snippets/_opaque-design-goal.mdx';

:::tip goal
<Goal />
:::

## Safety concern

Questions similar to last section can be asked and answered,
so I do not repeat it here,
since the semantics of each component is mentioned below.

## Details of the components

### `(Generalized)AutoDropDartPersistentHandle`

```rust
struct AutoDropDartPersistentHandle(Dart_PersistentHandle);
impl Drop for AutoDropDartPersistentHandle { ... }
```

A `Dart_PersistentHandle` that delete the handle when `Drop`ped.

### `GuardedBox`, `ThreadBox`, `DartIsolateBox`

Take `ThreadBox` as an example. It is equivalent to:

```rust
struct ThreadBox<T> { inner: T, thread_id: ThreadId }
impl<T> ThreadBox<T> {
    fn any_method_that_uses_inner_value(&self) {
        if !self.is_on_creation_thread() { panic!(); }
        ...
    }
}
unsafe impl<T> Send for ThreadBox<T> {} // and Sync
```

Only allows manipulation at the thread which it is created.
It is a "black box" that nobody can open it when it is on another thread.

The inner value can never be (1) used or (2) dropped
on any thread except for the creation thread.

Therefore, even though it is `Send`/`Sync` among threads,
it is just a blackbox on all other threads, so we are safe.

### `DartOpaqueNonClone`

```rust
struct DartOpaqueNonClone {
    persistent_handle: ThreadBox<GeneralizedAutoDropDartPersistentHandle>,
    drop_port: SendableMessagePortHandle,
}
```

Only safe functions. Semantics about Send/Sync are already encapsulated in the ThreadBox.

### `DartOpaque`

```rust
struct DartOpaque {
    arc: Arc<DartOpaqueNonClone>,
}
```

Add `Arc` to make it cloneable.

## V1 documentations

:::info
This section was written for V1, so it may be slightly outdated for V2.
:::

### Ownership and GC

From the moment the opaque type is passed, Rust will own a persistent representation of the dart object (`Dart_PersistentHandle` or `JsValue`).
This means that while Rust owns `DartOpaque` the object will not be cleared by GC.
Also flutter_rust_bridge provides a thread-safe drop for `DartOpaque`: Rust delegates the drop to the Dart side using the Dart port.

### Example

#### Case 1: loopBack. 

Rust `api.rs`:
```rust
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


#### Case 2: drop.

Rust `api.rs`:
```rust
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


#### Case 3: Unwrap.

Rust `api.rs`:
```rust
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

### Dispose

If there is an attempt to delegate the drop to the Dart side after the drop port (RustApi.dispose()) has been closed,
flutter_rust_bridge will issue a warning in the logs, the memory behind the object will leak.

However, this should not happen, because RustApi itself usually live for the whole application lifetime,
and there is no need to dispose it.

