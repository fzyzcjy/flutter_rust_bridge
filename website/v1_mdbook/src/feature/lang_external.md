# External types

## Types in other files within the same crate

Imported symbols can be used normally. For example, with `use crate::data::{MyEnum, MyStruct};`, you can use `MyEnum` or `MyStruct` in your code normally.

### Example

```rust,noplayground
use crate::data::{MyEnum, MyStruct};

pub fn use_imported_things(my_struct: MyStruct, my_enum: MyEnum) { ... }
```

Becomes:

```Dart
// Well it just behaves normally as you expect
Future<void> useImportedThings({required MyStruct myStruct, required MyEnum myEnum});
```

Remark: If you are curious about `Future`, have a look at [this](async_dart.md).

## Types in other crates

The feature is called "mirroring". In short, you need to define the type again mirroring the external type that you want to use. That definition is only used at code-generation time to tell `flutter_rust_bridge` type information. To see exact grammar, have a look at the example below.

No need to worry whether this breaks the DRY principle, or what happens when you accidentally write down a wrong field. This is because *compile errors will happen* if your mirrored type is not exactly same as the original type.

More information: [#352](https://github.com/fzyzcjy/flutter_rust_bridge/pull/352)

When multiple structs have the same fields, you can mirror them *once* using grammar like `#[frb(mirror(FirstStruct, SecondStruct, ThirdStruct))]`. ([#619](https://github.com/fzyzcjy/flutter_rust_bridge/pull/619))

### Example

```rust,noplayground
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


```rust,noplayground
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
