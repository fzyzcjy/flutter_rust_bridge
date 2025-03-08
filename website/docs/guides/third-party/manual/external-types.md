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

## Traits in External Third-Party

Traits in External Third-party packages can be exposed / parsed through a `Proxy / Wrapper` design pattern as illustrated through the following example. 

### Example

#### External Third-party rust library

Assume there is an external crate/package called 'calc' that contains a trait by name `Calc` ( and an implementation `MyCalc` ) . 

How do we use this trait and implementation in our `flutter_rust_bridge` based package ? 

*<external_crate>/calc.rs* : 

```rust

pub trait Calc {
    fn add(&self, a: u32, b: u32) -> u32;

    fn overflow_add(&self, a: u32, b: u32) -> Result<u32, CalcError>;
}

pub struct MyCalc {}

impl Calc for MyCalc {
    fn add(&self, a: u32, b: u32) -> u32 {
        a + b
    }

    fn overflow_add(&self, a: u32, b: u32) -> Result<u32, CalcError> {
        Err(CalcError::OverflowError)
    }
}
```

#### flutter_rust_bridge based package

Suppose we want to use the above mentioned third-party package in our flutter/dart project, we first create a package separately based on `flutter_rust_bridge` . 

The scope of creating a package for `flutter_rust_bridge` is beyond the scope of this example. But assume that is created, we can try to use the above mentioned trait as below. 

##### Use Calc trait - Attempt 1 

*rust/src/api/calc.rs* 

```rust
use third_party::{Calc, MyCalc};

#[flutter_rust_bridge::frb(sync)]
pub fn new_calc() -> Box<dyn Calc> {
    Box::new(MyCalc {})
}
```

Then invoke `flutter_rust_bridge_codegen  generate` to generate the dart code from the rust libraries.

While the dart package does get generated, it is not really useful practically. 

```dart
Future<BoxCalc> newCalc() => RustLib.instance.api.crateApiParserNewCalc();

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Box < dyn Calc + Send + Sync >>>
abstract class BoxCalc implements RustOpaqueInterface {}
```

As you can see above, `BoxCalc` by itself is useless even if it is error-free since it does not have anything about the methods / functions in the trait. 

##### Use Calc trait - Attempt 2 

To address the above given issue, we use a `Proxy/Wrapper` pattern to address the same as below. 

* Create a new type for the `Box<Calc>`
* Create a new struct with the type as a member.
* Re-implement all the methods with a proxy to the actual trait implementation

*rust/src/api/calc.rs* 

```rust
#[frb(ignore)]
type ThirdPartyCalc = Box<dyn Calc + Send + Sync>;

#[frb]
pub struct CalcWrapper(ThirdPartyCalc);

impl CalcWrapper {
    pub fn add(&self, a: u32, b: u32) -> u32 {
        self.0.add(a, b)
    }

    pub fn overflow_add(&self, a: u32, b: u32) -> Result<u32, CalcError> {
        self.0.overflow_add(a, b)
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn new_calc() -> CalcWrapper {
    CalcWrapper(Box::new(MyCalc {}))
}
```

This should work now as you have an equivalent type in `dart` to use all those member methods / functions in the underlying Rust trait. 





