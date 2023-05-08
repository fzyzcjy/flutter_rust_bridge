// Mirroring example:
// The goal of mirroring is to use external objects without needing to convert them with an intermediate type
// In this case, the struct ApplicationSettings is defined in another crate (called external-lib)
//
// These all structs are used in both the `api.rs` and `api2.rs` files. 
// They are added to test how the `Common Definitions` feature works when `dart_decl_output_path` is set.


// To use an external type with mirroring, it MUST be imported publicly (aka. re-export)
pub use external_lib::{
    ApplicationEnv, ApplicationEnvVar, ApplicationMessage, ApplicationMode, ApplicationSettings,
    ListOfNestedRawStringMirrored, NestedRawStringMirrored, Numbers, RawStringEnumMirrored,
    RawStringMirrored, Sequences,
};
use flutter_rust_bridge::frb;

// To mirror an external struct, you need to define a placeholder type with the same definition
#[frb(mirror(ApplicationSettings))]
pub struct _ApplicationSettings {
    pub name: String,
    pub version: String,
    pub mode: ApplicationMode,
    pub env: Box<ApplicationEnv>,
    pub env_optional: Option<ApplicationEnv>,
}

#[frb(mirror(ApplicationMode))]
pub enum _ApplicationMode {
    Standalone,
    Embedded,
}

#[frb(mirror(ApplicationEnvVar))]
pub struct _ApplicationEnvVar(pub String, pub bool);

#[frb(mirror(ApplicationEnv))]
pub struct _ApplicationEnv {
    pub vars: Vec<ApplicationEnvVar>,
}

// This function can directly return an object of the external type ApplicationSettings because it has a mirror
pub fn get_app_settings_to_api2() -> ApplicationSettings {
    external_lib::get_app_settings()
}

// This function can return a Result, that includes an object of the external type ApplicationSettings because it has a mirror
pub fn get_fallible_app_settings_to_api2() -> anyhow::Result<ApplicationSettings> {
    Ok(external_lib::get_app_settings())
}

// Similarly, receiving an object from Dart works. Please note that the mirror definition must match entirely and the original struct must have all its fields public.
pub fn is_app_embedded_in_api2(app_settings: ApplicationSettings) -> bool {
    // info!("env: {:?}", app_settings.env.vars);
    matches!(app_settings.mode, ApplicationMode::Embedded)
}

#[frb(mirror(ApplicationMessage))]
pub enum _ApplicationMessage {
    DisplayMessage(String),
    RenderPixel { x: i32, y: i32 },
    Exit,
}
