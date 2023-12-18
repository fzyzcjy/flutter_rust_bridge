# Rust Opaque

## Design

We try our best to achieve the following:

import Goal from '../../../snippets/_opaque-design-goal.mdx';

:::tip goal
<Goal />
:::

## Safety concern

When looking at the components below, one critical question is: Is the implementation safe and sound?
With the following three components, in my humble opinion, the question is roughly splitted into:

1. `Droppable`: Does it ensure the "release the resource once and exactly once"?
2. `RustArc`: Does it ensure the semantics of standard `std::sync::Arc`?
E.g. after `intoRaw` is called, are we guarantee the pointer is "forgotten"?
3. `RustOpaque`: Together with the Rust side, is the `Arc::into_raw` and `Arc::from_raw` paired? (One `into_raw` for one `from_raw`)

Since each component is quite tiny, it is not very hard to check it.
Feel free to create an issue if you find any problems!

## Transferring between Rust and Dart

With the three-components abstractions below, this can be explained within a sentence:
To transfer a `RustOpaque` (indeed an `Arc`),
do `std::sync::Arc::into_raw` on one side,
and `std::sync::Arc::from_raw` on the other side.
(Replace `Arc` with `RustArc` when proper).

## Details of the components

(The text below are mainly copied from the code comments.)

### `Droppable`

```dart
class Droppable {
  PlatformPointer? _ptr;
  void dispose() { ... }
}
```

Encapsulates the `internalResource` release logic.

In Rust, it is simple to release some resource: Just implement `Drop` trait.
However, there are two possible chances to release resource in Dart:
1. When the object is garbage collected, the Dart finalizer will call a callback you choose.
2. When the user explicitly calls `dispose()` function, you can do releasing job.

But we want to release the `internalResource` *once and exactly once*.
That's what this class does.

### `RustArc`

```dart
class RustArc extends Droppable {
  RustArc clone() { ... }
  RustArc.fromRaw({int ptr}) { ... }
  PlatformPointer intoRaw() { ... }
}
```

The Rust `std::sync::Arc` on the Dart side.

It uses `Droppable` to ensure the Arc's destructor is correctly called exactly once.

### `RustOpaque`

```dart
abstract class RustOpaque {
  final RustArc _arc;
  RustOpaque.decode(raw) { ... }
  int encode() { ... }
}
```

Finally, the object we are interested in.

It uses `RustArc` to hold the actual data.

## V1 documentations

:::info
This section was written for V1, so it may be slightly outdated for V2.
:::

### Restrictions

A `RustOpaque type` can be created from any Rust structure. 
The `flutter_rust_bridge` async dart api requires the Rust type to be `Send` and `Sync`, due to the possible sharing of `RustOpaque type` by multiple `flutter_rust_bridge` executor threads.

### Ownership and GC

From the moment an opaque type is passed to Dart, it has full ownership of it.
Dart implements a finalizer for opaque types, but the memory usage of opaque types is not monitored by Dart and can accumulate, so in order to prevent memory leaks, opaque pointers must be `dispose`d.

### Rust opaque type like function args

When calling a function with an opaque type argument, the Dart thread safely shares ownership of the opaque type with Rust. This is safe because `RustOpaque<T>` requires that T be `Send` and `Sync`, furthermore Rust's `RustOpaque<T>` hand out immutable references through `Deref` or get an internal property if only Rust owns the opaque type. If dispose is called on the Dart side before the function call completes, Rust takes full ownership.

### Example

#### Case 1: Simple call. 

Rust `api.rs`:
```rust
pub use crate::data::HideData; // `pub` for bridge_generated.rs

pub fn create_opaque() -> RustOpaque<HideData> {
    // [`HideData`] has private fields.
    RustOpaque::new(HideData::new())
}

pub fn run_opaque(opaque: RustOpaque<HideData>) -> String {
    // RustOpaque impl Deref trait.
    opaque.hide_data()
}
```

Dart: (test:'Simple call' frb_example/pure_dart/dart/lib/main.dart)
```dart
// (Arc counter = 1) Dart has full ownership.
var opaque = await api.createOpaque();

// (Arc counter = 2) for the duration of the function 
// and after (Arc counter = 1).
// 
// Dart and Rust share the opaque type.
String hideData = await api.runOpaque(opaque);

// (Arc counter = 0) opaque type is dropped (deallocated).
opaque.dispose();
```



#### Case 2: Call after dispose.

Rust `api.rs`:
```rust
pub use crate::data::HideData; // `pub` for bridge_generated.rs

pub fn create_opaque() -> RustOpaque<HideData> {
    // [`HideData`] has private fields.
    RustOpaque::new(HideData::new())
}

pub fn run_opaque(opaque: RustOpaque<HideData>) -> String {
    // RustOpaque impl Deref trait.
    opaque.hide_data()
}
```

Dart: (test:'Call after dispose' frb_example/pure_dart/dart/lib/main.dart)
```dart
// (Arc counter = 1) Dart has full ownership.
var opaque = await api.createOpaque();

// (Arc counter = 0) opaque type dropped (deallocated)
opaque.dispose();

// (Arc counter = 0) Dart throws StateError('Use after dispose.')
try {
    await api.runOpaque(opaque: opaque);
} on StateError catch (e) {
    expect(e.toString(), 'Bad state: Use after dispose.');
}
```


#### Case 3: Dispose before complete.

Rust `api.rs`:
```rust
pub use crate::data::HideData; // `pub` for bridge_generated.rs

pub fn create_opaque() -> RustOpaque<HideData> {
    // [`HideData`] has private fields.
    RustOpaque::new(HideData::new())
}

pub fn run_opaque(opaque: RustOpaque<HideData>) -> String {
    // RustOpaque impl Deref trait.
    opaque.hide_data()
}

pub fn run_opaque_with_delay(opaque: RustOpaque<HideData>) -> String {
    sleep(Duration::from_millis(1000));
    opaque.hide_data()
}
```

Dart:
```dart
// (Arc counter = 1) Dart has full ownership.
var opaque = await api.createOpaque();

// (Arc counter = 2) increases immediately. 
// Dart and Rust share the opaque type.
// Safely because opaque type has `Send` `Sync` Rust trait.
var unawait_task = api.runOpaqueWithDelay(opaque: opaque);

// (Arc counter = 1) Rust has full ownership.
// Dart stops owning the opaque type. 
// Trying to use an opaque type will throw StateError('Use after dispose.')
opaque.dispose();

// Successfully completed.
//
// Rust:
// `executes run_opaque_with_delay.`
// after complete (Arc counter = 0) 
// opaque type is dropped (deallocated)
await unawait_task;
```


#### Case 4: Multi call.

Rust `api.rs`:
```rust
pub use crate::data::HideData; // `pub` for bridge_generated.rs

pub fn create_opaque() -> RustOpaque<HideData> {
    // [`HideData`] has private fields.
    RustOpaque::new(HideData::new())
}

pub fn run_opaque(opaque: RustOpaque<HideData>) -> String {
    // RustOpaque impl Deref trait.
    opaque.hide_data()
}
```

Dart: (test:'Double Call' frb_example/pure_dart/dart/lib/main.dart)
```dart

// (Arc counter = 1) Dart has full ownership.
var opaque = await api.createOpaque();

// (Arc counter = 2) increases immediately.
// (Arc counter = 1) after complete
String hideData1 = await api.runOpaque(opaque: opaque);

// (Arc counter = 2) increases immediately.
// (Arc counter = 1) after complete
String hideData2 = await api.runOpaque(opaque: opaque);

// (Arc counter = 0) opaque type is dropped (deallocated)
opaque.dispose();
```



#### Case 5: Double call with dispose before complete.

Rust `api.rs`:
```rust
pub use crate::data::HideData; // `pub` for bridge_generated.rs

pub fn create_opaque() -> RustOpaque<HideData> {
    // [`HideData`] has private fields.
    RustOpaque::new(HideData::new())
}

pub fn run_opaque(opaque: RustOpaque<HideData>) -> String {
    // RustOpaque impl Deref trait.
    opaque.hide_data()
}
```

Dart:
```dart

// (Arc counter = 1) Dart has full ownership.
var opaque = await api.createOpaque();

// (Arc counter = 2) increases immediately. 
var unawait_task1 = api.runOpaque(opaque); *1

// (Arc counter = 3) increases immediately. 
var unawait_task2 = api.runOpaque(opaque); *2

// (Arc counter = 2) Rust has full ownership
opaque.dispose();

// (*1 is complete) (Arc counter = 1)
//
// Rust:
//
//`executes rust_call_example and counter decreases.`

// (*2 is complete) (Arc counter = 0) 
// opaque type is dropped (deallocated)
//
// Rust:
//
//`executes rust_call_example and drop opaque type.`
```


#### Case 6: Dispose was not called (native).

Rust `api.rs`:
```rust
pub use crate::data::HideData; // `pub` for bridge_generated.rs

pub fn create_opaque() -> RustOpaque<HideData> {
    // [`HideData`] has private fields.
    RustOpaque::new(HideData::new())
}

pub fn run_opaque(opaque: RustOpaque<HideData>) -> String {
    // RustOpaque impl Deref trait.
    opaque.hide_data()
}
```

Dart:
```dart

// (Arc counter = 1) Dart has full ownership.
var opaque = await api.createOpaque();

// (Arc counter = 2) increases immediately. 
String hideData = await api.runOpaque(opaque);

// (Arc counter = 1)
//
// Rust:
//
// `executes rust_call_example and counter decreases.`

// memory of opaque types is not monitoring by dart and can accumulate.
// (Arc counter = 0) 
// opaque type is dropped (deallocated)
// 
// Dart:
//
// `the finalizer is guaranteed to be called before the program terminates.`
```

#### Case 7: Dispose was not called (web).

Rust `api.rs`:
```rust
pub use crate::data::HideData; // `pub` for bridge_generated.rs

pub fn create_opaque() -> RustOpaque<HideData> {
    // [`HideData`] has private fields.
    RustOpaque::new(HideData::new())
}

pub fn run_opaque(opaque: RustOpaque<HideData>) -> String {
    // RustOpaque impl Deref trait.
    opaque.hide_data()
}
```

Dart:
```dart

// (Arc counter = 1) Dart has full ownership.
var opaque = await api.createOpaque(); 

// (Arc counter = 2) increases immediately. 
String hideData = await api.rustOpaque(opaque);

// (Arc counter = 1)
//
// Rust:
//
//`executes rust_call_example and counter decreases.`

// memory of opaque types is not monitoring by Dart and can accumulate.
// (Arc count can be 0 or 1) don't count on automatic clearing.
//
// Dart:
//
//`the finalizer is NOT guaranteed to be called before the program terminates.`
```


#### Case 8: Unwrap.

Rust `api.rs`:
```rust
pub use crate::data::HideData; // `pub` for bridge_generated.rs

pub fn unwrap_rust_opaque(opaque: Opaque<HideData>) -> Result<String> {
    let res: Result<HideData, Opaque<HideData>> = opaque.try_unwrap();
    let data: HideData = res.map_err(|_| anyhow::anyhow!("opaque type is shared"))?;
    Ok(data.hide_data())
}

```

Dart:
```dart

// (Arc counter = 1) Dart has full ownership.
var opaque = await api.createOpaque();

// When passed as an argument, dart will relinquish ownership.
opaque.move = true;

// (Arc counter = 1) Rust has full ownership.
// On the Rust side, the Arc unwrap safely 
// as the Rust has full ownership of the opaque type. 
// Memory is cleared in the usual way Rust.
await api.unwrapRustOpaque(opaque: data);
```


