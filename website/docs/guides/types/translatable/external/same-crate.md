# Same crate

For types in other files within the same crate, imported symbols can be used normally.
For example, with `use crate::data::{MyEnum, MyStruct};`, you can use `MyEnum` or `MyStruct` in your code normally.

## Example

```rust
use crate::data::{MyEnum, MyStruct};

pub fn use_imported_things(my_struct: MyStruct, my_enum: MyEnum) { ... }
```

Becomes:

```Dart
// Well it just behaves normally as you expect
Future<void> useImportedThings({required MyStruct myStruct, required MyEnum myEnum});
```

Remark: If you are curious about `Future`, have a look at [this](../../../concurrency/async-dart).

