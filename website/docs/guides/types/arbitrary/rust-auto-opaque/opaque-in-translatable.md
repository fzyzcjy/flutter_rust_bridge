# Opaque inside translatable

:::tip
There is no need to memorize anything here (or anything in doc) -
the code generator will provide warnings when detecting non-best-practices.
:::

Suppose you want to have a translatable struct, and it has an (auto) opaque field:

```rust
pub struct A { pub b: B }

// Suppose it is opaque
pub struct B { ... }
```

If you want to use the same object of type `A` multiple times,
then it is suggested (and flutter_rust_bridge will automatically hint you about that)
to add an `RustAutoOpaque<...>` wrapper on it:

```diff
-    pub b: B,
+    pub b: RustAutoOpaque<B>,
```

(Optional) explanations:
This is because, shortly speaking,
the original version needs to have an owned `B`, and thus the `B` object cannot be used later.
On the other hand, the updated version, which uses `RustAutoOpaque` thus `Arc`, will only require shared ownership.

(Optional) In the future, we may utilize [this feature](../../../misc-features/proxy),
then the scenario above can be automatically handled without any change to `RustAutoOpaque`.
