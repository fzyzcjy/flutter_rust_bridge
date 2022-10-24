# Opaque type safety

## Ownership and GC

From the moment an opaque type is passed to Dart, it has full ownership of it.
Dart implements a finalizer for opaque types, but
the memory of opaque types is not monitoring by Dart and can accumulate, so
in an instance of an opaque type, 'dispose' must occur.


## Opaque type like funtion args

When calling a function with an opaque type argument, Dart threadsafely shares ownership of the opaque type with Rust. This is safe because (Opaque<T>) type T requires Send, Sync, also Rust Opaque<T> only provides Deref (an immutable reference to T).
If dispose is called on the Dart side before the function call completes, Rust takes full ownership.


## Edxample
 
### Case 1 simple call.
rust: pub fn rust_example() -> Opaque<T>
rust: pub fn rust_call_example(data: Opaque<T>)

dart: var opaque = var data = await api.rustExample(); // Arc 1 Dart has full ownership.

// Arc 2 for the duration of the function and after Arc 1. 
// Dart and Rust share the opaque type.
dart: await api.rustCallExample(opaque);

dart: opaque.dispose(); // Arc 0 drop opaque type



### Case 2 call after dispose.
rust: pub fn rust_example() -> Opaque<T>
rust: pub fn rust_call_example(data: Opaque<T>)

dart: var opaque = var data = await api.rustExample(); // Arc 1 Dart has full ownership.
dart: opaque.dispose(); // Arc 0 drop opaque type

dart: await api.rustCallExample(opaque); // Arc 0 Dart throws error;



### Case 3 delete before complete.
rust: pub fn rust_example() -> Opaque<T>
rust: pub fn rust_call_example(data: Opaque<T>)

dart: var opaque = var data = await api.rustExample(); // Arc 1 Dart has full ownership.

// Arc 2 increases immediately. 
// Dart and Rust share the opaque type.
dart (unawait): api.rustCallExample(opaque);

dart: opaque.dispose(); // Arc 1 Rust has full ownership
rust: `executes rust_call_example and drop opaque type.` // Arc 0



### Case 4 multi call.
rust: pub fn rust_example() -> Opaque<T>
rust: pub fn rust_call_example(data: Opaque<T>)

dart: var opaque = var data = await api.rustExample(); // Arc 1 Dart has full ownership.

// Arc 2 increases immediately.
// Arc 1 after complete
dart: await api.rustCallExample(opaque); 

// Arc 2 increases immediately.
// Arc 1 after complete
dart: await api.rustCallExample(opaque);

dart: opaque.dispose(); // Arc 0 drop opaque type



### Case 5 multi call with dispose before complete.
rust: pub fn rust_example() -> Opaque<T>
rust: pub fn rust_call_example(data: Opaque<T>)

dart: var opaque = var data = await api.rustExample(); // Arc 1 Dart has full ownership.

dart (unawait): api.rustCallExample(opaque); // Arc 2 increases immediately. 

dart (unawait): api.rustCallExample(opaque); // Arc 3 increases immediately. 

dart: opaque.dispose(); // Arc 2 Rust has full ownership

rust: `executes rust_call_example and counter decreases.` // Arc 1
rust: `executes rust_call_example and drop opaque type.` // Arc 0



### Case 6 dispose was not called (native).
rust: pub fn rust_example() -> Opaque<T>
rust: pub fn rust_call_example(data: Opaque<T>)

dart: var opaque = var data = await api.rustExample(); // Arc 1 Dart has full ownership.

dart: await api.rustCallExample(opaque); // Arc 2 increases immediately. 

rust: `executes rust_call_example and counter decreases.` // Arc 1

dart: `memory of opaque types is not monitoring by dart and can accumulate`.
dart: `the finalizer is guaranteed to be called before the program terminates.` // Arc 0


### Case 7 dispose was not called (web).
rust: pub fn rust_example() -> Opaque<T>
rust: pub fn rust_call_example(data: Opaque<T>)

dart: var opaque = var data = await api.rustExample(); // Arc 1 Dart has full ownership.

dart: await api.rustCallExample(opaque); // Arc 2 increases immediately. 

rust: `executes rust_call_example and counter decreases.` // Arc 1

dart: `memory of opaque types is not monitoring by Dart and can accumulate`.
dart: `the finalizer is NOT guaranteed to be called before the program terminates.` // Arc 0?1

