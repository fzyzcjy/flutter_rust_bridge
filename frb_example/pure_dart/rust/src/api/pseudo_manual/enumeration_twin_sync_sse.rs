// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `enumeration.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use crate::api::pseudo_manual::misc_example_twin_sync_sse::WeekdaysTwinSyncSse;
use flutter_rust_bridge::frb;
use log::info;

pub enum EnumSimpleTwinSyncSse {
    A,
    B,
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn func_enum_simple_twin_sync_sse(arg: EnumSimpleTwinSyncSse) -> EnumSimpleTwinSyncSse {
    arg
}

pub enum EnumWithItemMixedTwinSyncSse {
    A,
    B(Vec<u8>),
    C { c_field: String },
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn func_enum_with_item_mixed_twin_sync_sse(
    arg: EnumWithItemMixedTwinSyncSse,
) -> EnumWithItemMixedTwinSyncSse {
    arg
}

pub enum EnumWithItemTupleTwinSyncSse {
    A(Vec<u8>),
    B(Vec<i32>),
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn func_enum_with_item_tuple_twin_sync_sse(
    arg: EnumWithItemTupleTwinSyncSse,
) -> EnumWithItemTupleTwinSyncSse {
    arg
}

pub enum EnumWithItemStructTwinSyncSse {
    A { a_field: Vec<u8> },
    B { b_field: Vec<i32> },
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn func_enum_with_item_struct_twin_sync_sse(
    arg: EnumWithItemStructTwinSyncSse,
) -> EnumWithItemStructTwinSyncSse {
    arg
}

// #1674
pub enum EnumWithDiscriminantTwinSyncSse {
    OneHundred = 100,
    Fifty = 50,
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn func_enum_with_discriminant_twin_sync_sse(
    arg: EnumWithDiscriminantTwinSyncSse,
) -> EnumWithDiscriminantTwinSyncSse {
    arg
}

#[frb]
#[derive(Debug)]
pub struct NoteTwinSyncSse {
    #[frb(default = "WeekdaysTwinSyncSse.Sunday")]
    pub day: Box<WeekdaysTwinSyncSse>,
    pub body: String,
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn print_note_twin_sync_sse(note: NoteTwinSyncSse) -> Vec<u8> {
    info!("{:#?}", note);
    vec![1, 2, 3]
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn handle_return_enum_twin_sync_sse(input: String) -> Option<WeekdaysTwinSyncSse> {
    match input.as_str() {
        "Monday" => Some(WeekdaysTwinSyncSse::Monday),
        "Tuesday" => Some(WeekdaysTwinSyncSse::Tuesday),
        "Wednesday" => Some(WeekdaysTwinSyncSse::Wednesday),
        "Thursday" => Some(WeekdaysTwinSyncSse::Thursday),
        "Friday" => Some(WeekdaysTwinSyncSse::Friday),
        "Saturday" => Some(WeekdaysTwinSyncSse::Saturday),
        "Sunday" => Some(WeekdaysTwinSyncSse::Sunday),
        _ => None,
    }
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn handle_enum_parameter_twin_sync_sse(weekday: WeekdaysTwinSyncSse) -> WeekdaysTwinSyncSse {
    info!("The weekday is {:?}", weekday);
    weekday
}

#[derive(Debug, Clone)]
pub enum SpeedTwinSyncSse {
    Unknown,
    GPS(f64),
}

#[derive(Debug, Clone)]
pub enum DistanceTwinSyncSse {
    Unknown,
    Map(f64),
}

#[derive(Debug, Clone)]
pub enum MeasureTwinSyncSse {
    Speed(Box<SpeedTwinSyncSse>),
    Distance(Box<DistanceTwinSyncSse>),
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn multiply_by_ten_twin_sync_sse(measure: MeasureTwinSyncSse) -> Option<MeasureTwinSyncSse> {
    match measure {
        MeasureTwinSyncSse::Speed(b) => match *b {
            SpeedTwinSyncSse::GPS(v) => Some(MeasureTwinSyncSse::Speed(Box::new(
                SpeedTwinSyncSse::GPS(v * 10.),
            ))),
            SpeedTwinSyncSse::Unknown => None,
        },
        MeasureTwinSyncSse::Distance(b) => match *b {
            DistanceTwinSyncSse::Map(v) => Some(MeasureTwinSyncSse::Distance(Box::new(
                DistanceTwinSyncSse::Map(v * 10.),
            ))),
            DistanceTwinSyncSse::Unknown => None,
        },
    }
}

#[frb]
#[derive(Debug)]
pub enum KitchenSinkTwinSyncSse {
    /// Comment on variant
    Empty,
    #[frb(unimpl_variant_attr)]
    Primitives {
        /// Dart field comment
        #[frb(default = -1)]
        int32: i32,
        float64: f64,
        boolean: bool,
    },
    Nested(
        i32,
        #[frb(default = "KitchenSinkTwinSyncSse.empty()")] Box<KitchenSinkTwinSyncSse>,
    ),
    Optional(
        /// Comment on anonymous field
        #[frb(default = -1)]
        Option<i32>,
        Option<i32>,
    ),
    Buffer(Vec<u8>),
    Enums(#[frb(default = "WeekdaysTwinSyncSse.Sunday")] WeekdaysTwinSyncSse),
}

#[flutter_rust_bridge::frb(serialize)]
#[flutter_rust_bridge::frb(sync)]
pub fn handle_enum_struct_twin_sync_sse(val: KitchenSinkTwinSyncSse) -> KitchenSinkTwinSyncSse {
    use KitchenSinkTwinSyncSse::*;
    use WeekdaysTwinSyncSse::*;
    let inc = |x| x + 1;
    match val {
        Primitives {
            int32,
            float64,
            boolean,
        } => Primitives {
            int32: int32 + 1,
            float64: float64 + 1.,
            boolean: !boolean,
        },
        Nested(val, _nested) => Nested(inc(val), Box::new(KitchenSinkTwinSyncSse::Empty)),
        Optional(a, b) => Optional(a.map(inc), b.map(inc)),
        Buffer(mut buf) => {
            buf.push(1);
            Buffer(buf)
        }
        Enums(day) => Enums(match day {
            Monday => Tuesday,
            Tuesday => Wednesday,
            Wednesday => Thursday,
            Thursday => Friday,
            Friday => Saturday,
            Saturday => Sunday,
            Sunday => Monday,
        }),
        _ => val,
    }
}
