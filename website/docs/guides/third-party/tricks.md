# Tricks

This page discusses some tricks that may be helpful when working with external crates.

## Injecting Dart code

The [`#[frb(dart_code)]`](../miscellaneous/dart-code) feature can be utilized when we want to inject anything into the generated Dart code.
For example, we can add a method to an auto-generated Dart class.

## Using Rust macros

When we are going to write down code that *repeat itself*,
Rust macros can often be helpful.

### Example

For example, instead of writing the following code
(this example is somehow artificial and only serve to show how macro reduces repetition):

```rust
impl One {
    #[frb(sync)]
    pub fn connect() {}
}

impl Two {
    #[frb(sync)]
    pub fn connect() {}
}

impl Three {
    #[frb(sync)]
    pub fn connect() {}
}

// ... repeat ...

impl Twenty {
    #[frb(sync)]
    pub fn connect() {}
}
```

We can do the following instead:

```rust
macro_rules! my_macro { ($($name:ident),+) => { $(
    impl $name {
        #[frb(sync)]
        pub fn connect() {}
    }
)+ }; }
my_macro!(One, Two, Three, ..., Twenty);
```
