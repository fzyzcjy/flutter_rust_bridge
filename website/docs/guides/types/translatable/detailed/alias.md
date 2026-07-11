# Type alias

Type alias is also supported. For example:

```rust
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

## Generic type aliases

Generic type aliases are expanded at the use site, so you can use them in exported
signatures exactly like the underlying type. A common case is a project-wide
fallible `Result` shortcut:

```rust
pub enum AppError {...}

pub type AppResult<T> = Result<T, AppError>;

// Equivalent to `-> Result<MyData, AppError>`: on the Dart side this returns
// `MyData` on success and throws a Dart exception carrying `AppError` on failure.
pub fn load_data() -> AppResult<MyData> {...}
```

The substitution is applied recursively, so aliases built on top of other generic
aliases (for example `pub type Wrapper<T> = Option<T>`) work as well.

## Limitation

- The `ItemType` inside Generic is not supported yet, such as `SyncReturn<Id>`. The nested `ItemType` may also not be supported. This also applies to generic aliases nested inside such wrappers (e.g. `SyncReturn<AppResult<T>>`).
- Generic type aliases with lifetime parameters, const parameters, or a `where` clause are not yet supported.
- A generic alias literally named `Result` (e.g. `pub type Result<T> = std::result::Result<T, AppError>`) is not yet expanded. `Result` is reserved for the built-in fallible-return detection, and use a differently-named alias such as `AppResult<T>` currently if you want the concrete error type to flow through.
