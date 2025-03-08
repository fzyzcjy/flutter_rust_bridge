# External types

:::info
Third-party packages can be converted automatically; only use the feature in this page when the automation is unwanted.
:::

This page shows how to use translatable types defined in third party crates in an manual approach - the "mirroring" feature.

This boilerplate is only needed for translatable types.
For [arbitrary types](../../types/arbitrary), everything is automatic.

In short, you need to define the type again mirroring the external type that you want
to use. That definition is only used at code-generation time to tell `flutter_rust_bridge` type information.

## Remarks

No need to worry whether this breaks the DRY principle, or what happens when you accidentally write down a wrong field.
This is because compile errors will happen if your mirrored type is not exactly same as the original type.

When multiple structs have the same fields, you can mirror them *once* using grammar
like `#[frb(mirror(FirstStruct, SecondStruct, ThirdStruct))]`.

When it comes to types in other crates,
[serde](https://crates.io/crates/serde) also has a similar developer-facing API,
requiring users to write down the details of the remote struct:
https://serde.rs/remote-derive.html.

## Example

```rust
// Mirroring example:
// The goal of mirroring is to use external objects without needing to convert them with an intermediate type
// In this case, the struct ApplicationSettings is defined in another crate (called external-lib)

// To use an external type with mirroring, it MUST be imported publicly (aka. re-export)
pub use external_lib::{ApplicationEnv, ApplicationMode, ApplicationSettings};

// To mirror an external struct, you need to define a placeholder type with the same definition
#[frb(mirror(ApplicationSettings))]
pub struct _ApplicationSettings {
    pub name: String,
    pub version: String,
    pub mode: ApplicationMode,
    pub env: Box<ApplicationEnv>,
}

// It works with basic enums too
// Enums with struct variants are not yet supported
#[frb(mirror(ApplicationMode))]
pub enum _ApplicationMode {
    Standalone,
    Embedded,
}

#[frb(mirror(ApplicationEnv))]
pub struct _ApplicationEnv {
    pub vars: Vec<String>,
}

// This function can directly return an object of the external type ApplicationSettings because it has a mirror
pub fn get_app_settings() -> ApplicationSettings {
    external_lib::get_app_settings()
}

// Similarly, receiving an object from Dart works. Please note that the mirror definition must match entirely and the original struct must have all its fields public.
pub fn is_app_embedded(app_settings: ApplicationSettings) -> bool {
    // println!("env: {}", app_settings.env.vars[0]);
    match app_settings.mode {
        ApplicationMode::Standalone => false,
        ApplicationMode::Embedded => true,
    }
}
```

Another example using one struct to mirror multiple structs:

```rust
// *no* need to do these
#[frb(mirror(MessageId))]
pub struct MId(pub [u8; 32]);

#[frb(mirror(BlobId))]
pub struct BId(pub [u8; 32]);

#[frb(mirror(FeedId))]
pub struct FId(pub [u8; 32]);

// simply do this is sufficient
#[frb(mirror(MessageId, BlobId, FeedId))]
pub struct Id(pub [u8; 32]);
```

## Traits

Before flutter_rust_bridge supports more advanced parsing,
traits in external third-party packages can be utilized
through a proxy/wrapper/newtype design pattern as illustrated through the following example. 

### Example

In this example,
assume there is an external crate/package called 'calc' that contains a trait by name `Calc` (and an implementation `CalcImpl`),
and we demonstrate how to use this trait and implementation in our `flutter_rust_bridge` based package.

#### External crate

*<external_crate>/calc.rs* : 

```rust
pub trait Calc {
    fn add(&self, a: u32, b: u32) -> u32;
}

pub struct CalcImpl {}

impl Calc for CalcImpl {
    fn add(&self, a: u32, b: u32) -> u32 {
        a + b
    }
}
```

#### Our crate

##### Scenario 1

*rust/src/api/calc.rs* 

```rust
pub fn new_calc() -> Box<dyn Calc> {
    Box::new(CalcImpl {})
}
```

Then we get an opaque object about `Calc`, which can be passed back to Rust functions.

##### Scenario 2

If we want to have the methods on the `Calc` trait exposed as methods in Dart,
one approach is to use Rust's commonly-seen proxy/wrapper/newtype design pattern as follows.

*rust/src/api/calc.rs* 

```rust
// So-called "newtype" pattern
pub struct CalcWrapper(Box<dyn Calc + Send + Sync>);

impl CalcWrapper {
    pub fn new() -> Self {
        Self(Box::new(CalcImpl {})) // or whatever else...
    }
    
    pub fn add(&self, a: u32, b: u32) -> u32 {
        self.0.add(a, b)
    }
}
```

This should work now as you have an equivalent type in `dart` to use all those member methods / functions in the underlying Rust trait. 
