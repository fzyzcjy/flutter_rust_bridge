# Type alias

Type alias is also supported. For example:

```rust,noplayground
enum MyEnum {...}
struct MyStruct {...}

// type aliases
pub type Id = u64;
pub type EnumAlias = MyEnum;
pub type StructAlias = MyStruct;

// can also use them in fields, etc
pub struct TestModel { pub id: Id, pub e: EnumAlias, pub s: StructAlias}

pub fn f(input: Id) -> TestModel {...}
```

## Limitation

The `ItemType` inside Generic is not supported yet, such as `SyncReturn<Id>`. The nested `ItemType` may also not be supported.

