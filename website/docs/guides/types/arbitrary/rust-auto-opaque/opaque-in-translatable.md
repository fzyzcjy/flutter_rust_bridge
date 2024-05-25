# Opaque inside translatable

:::tip
There is no need to memorize anything here (or anything in doc) -
the code generator will provide warnings when detecting non-best-practices.
:::

Suppose you want to have a translatable struct, and it has an auto opaque field:

```rust
pub struct MyTranslatable {
    pub name: String,
    pub opaque: MyOpaque,
}

pub struct MyOpaque {
    db: FancyDatabaseConnection,
}
```
