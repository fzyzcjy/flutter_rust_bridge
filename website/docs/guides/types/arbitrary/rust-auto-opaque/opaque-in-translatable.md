# Opaque inside translatable

:::tip
There is no need to memorize anything here (or anything in doc) -
the code generator will provide warnings when detecting non-best-practices.
:::

Suppose you want to have a translatable struct, and it has an auto opaque field:

```rust
pub struct A {
    pub name: String,
    pub b: B,
}

pub struct B {
    db: FancyDatabaseConnection,
}
```

Then, it is suggested (and flutter_rust_bridge will automatically hint you about that)
to add an `RustAutoOpaque<...>` wrapper on it:

```diff
-    pub b: B,
+    pub b: RustAutoOpaque<B>,
```

This is because, TODO
