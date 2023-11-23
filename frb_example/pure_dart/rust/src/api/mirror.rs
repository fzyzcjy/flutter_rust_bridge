// Mirroring example:
// The goal of mirroring is to use external objects without needing to convert them with an intermediate type
// In this case, the struct ApplicationSettings is defined in another crate (called external-lib)

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
pub fn get_app_settings() -> ApplicationSettings {
    external_lib::get_app_settings()
}

// This function can return a Result, that includes an object of the external type ApplicationSettings because it has a mirror
pub fn get_fallible_app_settings() -> anyhow::Result<ApplicationSettings> {
    Ok(external_lib::get_app_settings())
}

// Similarly, receiving an object from Dart works. Please note that the mirror definition must match entirely and the original struct must have all its fields public.
pub fn is_app_embedded(app_settings: ApplicationSettings) -> bool {
    // info!("env: {:?}", app_settings.env.vars);
    matches!(app_settings.mode, ApplicationMode::Embedded)
}

// use a stream of a mirrored type
pub fn app_settings_stream(sink: StreamSink<ApplicationSettings>) {
    let app_settings = external_lib::get_app_settings();
    sink.add(app_settings);
    sink.close();
}

// use a stream of a vec of mirrored type
pub fn app_settings_vec_stream(sink: StreamSink<Vec<ApplicationSettings>>) {
    let app_settings = vec![
        external_lib::get_app_settings(),
        external_lib::get_app_settings(),
    ];
    sink.add(app_settings);
    sink.close();
}

pub struct MirrorStruct {
    pub a: ApplicationSettings,
    pub b: MyStruct,
    pub c: Vec<MyEnum>,
    pub d: Vec<ApplicationSettings>,
}

// use a Struct consisting of mirror types as argument to a Stream
pub fn mirror_struct_stream(sink: StreamSink<MirrorStruct>) {
    let val = MirrorStruct {
        a: external_lib::get_app_settings(),
        b: MyStruct { content: true },
        c: vec![MyEnum::True, MyEnum::False],
        d: vec![
            external_lib::get_app_settings(),
            external_lib::get_app_settings(),
        ],
    };
    sink.add(val);
    sink.close();
}

// usa a tuple of Mirror types for a StreamSink
pub fn mirror_tuple_stream(sink: StreamSink<(ApplicationSettings, RawStringEnumMirrored)>) {
    let tuple = (
        external_lib::get_app_settings(),
        RawStringEnumMirrored::Raw(RawStringMirrored {
            value: String::from("test"),
        }),
    );
    sink.add(tuple);
    sink.close();
}

#[frb(mirror(ApplicationMessage))]
pub enum _ApplicationMessage {
    DisplayMessage(String),
    RenderPixel { x: i32, y: i32 },
    Exit,
}

pub fn get_message() -> ApplicationMessage {
    external_lib::poll_messages()[1].clone()
}

#[frb(mirror(Numbers, Sequences))]
pub struct _Numbers(pub Vec<i32>);

pub fn repeat_number(num: i32, times: usize) -> Numbers {
    external_lib::repeat_number(num, times)
}

pub fn repeat_sequence(seq: i32, times: usize) -> Sequences {
    external_lib::repeat_sequences(seq, times)
}

pub fn first_number(nums: Numbers) -> Option<i32> {
    nums.0.first().copied()
}

pub fn first_sequence(seqs: Sequences) -> Option<i32> {
    seqs.0.first().copied()
}
