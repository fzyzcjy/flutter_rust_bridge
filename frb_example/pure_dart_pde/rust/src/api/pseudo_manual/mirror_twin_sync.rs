// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `mirror.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

// Mirroring example:
// The goal of mirroring is to use external objects without needing to convert them with an intermediate type
// In this case, the struct ApplicationSettings is defined in another crate (called external-lib)

// To use an external type with mirroring, it MUST be imported publicly (aka. re-export)
use crate::auxiliary::sample_types::{MyEnum, MyStruct};
use crate::frb_generated::StreamSink;
use flutter_rust_bridge::frb;
pub use frb_example_pure_dart_example_external_lib::{
    ApplicationEnv, ApplicationEnvVar, ApplicationMessage, ApplicationMode, ApplicationSettings,
    HashMapValue, ListOfNestedRawStringMirrored, NestedRawStringMirrored, Numbers,
    RawStringEnumMirrored, RawStringMirrored, Sequences, StructWithHashMap,
};
use std::collections::{HashMap, HashSet};

// To mirror an external struct, you need to define a placeholder type with the same definition
#[frb(mirror(ApplicationSettings))]
pub struct _ApplicationSettingsTwinSync {
    pub name: String,
    pub version: String,
    pub mode: ApplicationMode,
    pub env: Box<ApplicationEnv>,
    pub env_optional: Option<ApplicationEnv>,
}

#[frb(mirror(ApplicationMode))]
pub enum _ApplicationModeTwinSync {
    Standalone,
    Embedded,
}

#[frb(mirror(ApplicationEnvVar))]
pub struct _ApplicationEnvVarTwinSync(pub String, pub bool);

#[frb(mirror(ApplicationEnv))]
pub struct _ApplicationEnvTwinSync {
    pub vars: Vec<ApplicationEnvVar>,
}

#[frb(mirror(HashMapValue))]
pub struct _HashMapValue {
    pub inner: String,
}

#[frb(mirror(StructWithHashMap))]
pub struct _StructWithHashMap {
    pub map: HashMap<String, HashMapValue>,
}

// This function can directly return an object of the external type ApplicationSettings because it has a mirror
#[flutter_rust_bridge::frb(sync)]
pub fn get_app_settings_twin_sync() -> ApplicationSettings {
    frb_example_pure_dart_example_external_lib::get_app_settings()
}

// This function can return a Result, that includes an object of the external type ApplicationSettings because it has a mirror
#[flutter_rust_bridge::frb(sync)]
pub fn get_fallible_app_settings_twin_sync() -> anyhow::Result<ApplicationSettings> {
    Ok(frb_example_pure_dart_example_external_lib::get_app_settings())
}

// Similarly, receiving an object from Dart works. Please note that the mirror definition must match entirely and the original struct must have all its fields public.
#[flutter_rust_bridge::frb(sync)]
pub fn is_app_embedded_twin_sync(app_settings: ApplicationSettings) -> bool {
    // info!("env: {:?}", app_settings.env.vars);
    matches!(app_settings.mode, ApplicationMode::Embedded)
}

// use a stream of a mirrored type
#[flutter_rust_bridge::frb(sync)]
pub fn app_settings_stream_twin_sync(sink: StreamSink<ApplicationSettings>) {
    let app_settings = frb_example_pure_dart_example_external_lib::get_app_settings();
    sink.add(app_settings).unwrap();
}

// use a stream of a vec of mirrored type
#[flutter_rust_bridge::frb(sync)]
pub fn app_settings_vec_stream_twin_sync(sink: StreamSink<Vec<ApplicationSettings>>) {
    let app_settings = vec![
        frb_example_pure_dart_example_external_lib::get_app_settings(),
        frb_example_pure_dart_example_external_lib::get_app_settings(),
    ];
    sink.add(app_settings).unwrap();
}

pub struct MirrorStructTwinSync {
    pub a: ApplicationSettings,
    pub b: MyStruct,
    pub c: Vec<MyEnum>,
    pub d: Vec<ApplicationSettings>,
}

// use a Struct consisting of mirror types as argument to a Stream
#[flutter_rust_bridge::frb(sync)]
pub fn mirror_struct_stream_twin_sync(sink: StreamSink<MirrorStructTwinSync>) {
    let val = MirrorStructTwinSync {
        a: frb_example_pure_dart_example_external_lib::get_app_settings(),
        b: MyStruct { content: true },
        c: vec![MyEnum::True, MyEnum::False],
        d: vec![
            frb_example_pure_dart_example_external_lib::get_app_settings(),
            frb_example_pure_dart_example_external_lib::get_app_settings(),
        ],
    };
    sink.add(val).unwrap();
}

// usa a tuple of Mirror types for a StreamSink
#[flutter_rust_bridge::frb(sync)]
pub fn mirror_tuple_stream_twin_sync(
    sink: StreamSink<(ApplicationSettings, RawStringEnumMirrored)>,
) {
    let tuple = (
        frb_example_pure_dart_example_external_lib::get_app_settings(),
        RawStringEnumMirrored::Raw(RawStringMirrored {
            value: String::from("test"),
        }),
    );
    sink.add(tuple).unwrap();
}

#[frb(mirror(ApplicationMessage))]
pub enum _ApplicationMessageTwinSync {
    DisplayMessage(String),
    RenderPixel { x: i32, y: i32 },
    Exit,
}

#[flutter_rust_bridge::frb(sync)]
pub fn get_message_twin_sync() -> ApplicationMessage {
    frb_example_pure_dart_example_external_lib::poll_messages()[1].clone()
}

#[frb(mirror(Numbers, Sequences))]
pub struct _NumbersTwinSync(pub Vec<i32>);

#[flutter_rust_bridge::frb(sync)]
pub fn repeat_number_twin_sync(num: i32, times: usize) -> Numbers {
    frb_example_pure_dart_example_external_lib::repeat_number(num, times)
}

#[flutter_rust_bridge::frb(sync)]
pub fn repeat_sequence_twin_sync(seq: i32, times: usize) -> Sequences {
    frb_example_pure_dart_example_external_lib::repeat_sequences(seq, times)
}

#[flutter_rust_bridge::frb(sync)]
pub fn first_number_twin_sync(nums: Numbers) -> Option<i32> {
    nums.0.first().copied()
}

#[flutter_rust_bridge::frb(sync)]
pub fn first_sequence_twin_sync(seqs: Sequences) -> Option<i32> {
    seqs.0.first().copied()
}

#[frb(mirror(RawStringMirrored))]
pub struct _RawStringMirroredTwinSync {
    pub r#value: String,
}

#[frb(mirror(NestedRawStringMirrored))]
pub struct _NestedRawStringMirroredTwinSync {
    pub raw: RawStringMirrored,
}

#[frb(mirror(RawStringEnumMirrored))]
pub enum _RawStringEnumMirroredTwinSync {
    Raw(RawStringMirrored),
    Nested(NestedRawStringMirrored),
    ListOfNested(ListOfNestedRawStringMirrored),
}

#[frb(mirror(ListOfNestedRawStringMirrored))]
pub struct _ListOfRawNestedStringMirroredTwinSync {
    pub raw: Vec<NestedRawStringMirrored>,
}

#[flutter_rust_bridge::frb(sync)]
pub fn test_raw_string_mirrored_twin_sync() -> RawStringMirrored {
    RawStringMirrored {
        r#value: "test".to_owned(),
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn test_nested_raw_string_mirrored_twin_sync() -> NestedRawStringMirrored {
    NestedRawStringMirrored {
        raw: RawStringMirrored {
            r#value: "test".to_owned(),
        },
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn test_raw_string_enum_mirrored_twin_sync(nested: bool) -> RawStringEnumMirrored {
    if nested {
        RawStringEnumMirrored::Nested(NestedRawStringMirrored {
            raw: RawStringMirrored {
                r#value: "test".to_owned(),
            },
        })
    } else {
        RawStringEnumMirrored::Raw(RawStringMirrored {
            r#value: "test".to_owned(),
        })
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn test_list_of_raw_nested_string_mirrored_twin_sync() -> ListOfNestedRawStringMirrored {
    ListOfNestedRawStringMirrored {
        raw: vec![NestedRawStringMirrored {
            raw: RawStringMirrored {
                r#value: "test".to_owned(),
            },
        }],
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn test_fallible_of_raw_string_mirrored_twin_sync() -> anyhow::Result<Vec<RawStringMirrored>> {
    Ok(vec![RawStringMirrored {
        r#value: "test".to_owned(),
    }])
}

#[flutter_rust_bridge::frb(sync)]
pub fn test_list_of_nested_enums_mirrored_twin_sync() -> Vec<RawStringEnumMirrored> {
    vec![
        RawStringEnumMirrored::Nested(NestedRawStringMirrored {
            raw: RawStringMirrored {
                r#value: "test".to_owned(),
            },
        }),
        RawStringEnumMirrored::Raw(RawStringMirrored {
            r#value: "test".to_owned(),
        }),
    ]
}

// TODO rm (use the auto-generated sync code)
// #[flutter_rust_bridge::frb(sync)] pub fn sync_return_mirror_twin_sync() -> SyncReturn<ApplicationSettings> {
//     SyncReturn(frb_example_pure_dart_example_external_lib::get_app_settings())
// }

pub struct AnotherTwinSync {
    pub a: String,
}

pub struct ContainsMirroredSubStructTwinSync {
    pub test: RawStringMirrored,
    pub test2: AnotherTwinSync,
}

#[flutter_rust_bridge::frb(sync)]
pub fn test_contains_mirrored_sub_struct_twin_sync() -> ContainsMirroredSubStructTwinSync {
    ContainsMirroredSubStructTwinSync {
        test: RawStringMirrored {
            r#value: "test".to_owned(),
        },
        test2: AnotherTwinSync {
            a: "test".to_owned(),
        },
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn test_hashmap_with_mirrored_value_twin_sync() -> StructWithHashMap {
    StructWithHashMap {
        map: {
            [(
                "key".to_owned(),
                HashMapValue {
                    inner: "value".to_owned(),
                },
            )]
            .into()
        },
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn mirror_enum_stream_twin_sync(sink: StreamSink<ApplicationMode>) {
    sink.add(ApplicationMode::Embedded).unwrap();
    sink.add(ApplicationMode::Standalone).unwrap();
}

#[flutter_rust_bridge::frb(sync)]
pub fn mirror_option_enum_stream_twin_sync(sink: StreamSink<Option<ApplicationMode>>) {
    sink.add(Some(ApplicationMode::Embedded)).unwrap();
    sink.add(None).unwrap();
    sink.add(Some(ApplicationMode::Standalone)).unwrap();
}

#[flutter_rust_bridge::frb(sync)]
pub fn mirror_vec_enum_stream_twin_sync(sink: StreamSink<Vec<ApplicationMode>>) {
    sink.add(vec![ApplicationMode::Embedded]).unwrap();
    sink.add(vec![ApplicationMode::Standalone]).unwrap();
}

#[flutter_rust_bridge::frb(sync)]
pub fn mirror_map_enum_stream_twin_sync(sink: StreamSink<HashMap<u8, ApplicationMode>>) {
    sink.add(HashMap::from([
        (0, ApplicationMode::Embedded),
        (1, ApplicationMode::Standalone),
    ]))
    .unwrap();
}

#[flutter_rust_bridge::frb(sync)]
pub fn mirror_set_enum_stream_twin_sync(sink: StreamSink<HashSet<ApplicationMode>>) {
    sink.add(HashSet::from([
        ApplicationMode::Embedded,
        ApplicationMode::Standalone,
    ]))
    .unwrap();
}

#[flutter_rust_bridge::frb(sync)]
pub fn mirror_array_enum_stream_twin_sync(sink: StreamSink<[ApplicationMode; 2]>) {
    sink.add([ApplicationMode::Embedded, ApplicationMode::Standalone])
        .unwrap();
}
