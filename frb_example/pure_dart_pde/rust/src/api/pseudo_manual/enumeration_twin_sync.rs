// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `enumeration.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

use crate::api::pseudo_manual::misc_example_twin_sync::WeekdaysTwinSync;
use flutter_rust_bridge::frb;
use log::info;

pub enum EnumSimpleTwinSync {
    A,
    B,
}

#[flutter_rust_bridge::frb(sync)]
pub fn func_enum_simple_twin_sync(arg: EnumSimpleTwinSync) -> EnumSimpleTwinSync {
    arg
}

pub enum EnumWithItemMixedTwinSync {
    A,
    B(Vec<u8>),
    C { c_field: String },
}

#[flutter_rust_bridge::frb(sync)]
pub fn func_enum_with_item_mixed_twin_sync(
    arg: EnumWithItemMixedTwinSync,
) -> EnumWithItemMixedTwinSync {
    arg
}

pub enum EnumWithItemTupleTwinSync {
    A(Vec<u8>),
    B(Vec<i32>),
}

#[flutter_rust_bridge::frb(sync)]
pub fn func_enum_with_item_tuple_twin_sync(
    arg: EnumWithItemTupleTwinSync,
) -> EnumWithItemTupleTwinSync {
    arg
}

pub enum EnumWithItemStructTwinSync {
    A { a_field: Vec<u8> },
    B { b_field: Vec<i32> },
}

#[flutter_rust_bridge::frb(sync)]
pub fn func_enum_with_item_struct_twin_sync(
    arg: EnumWithItemStructTwinSync,
) -> EnumWithItemStructTwinSync {
    arg
}

// #1674
pub enum EnumWithDiscriminantTwinSync {
    OneHundred = 100,
    Fifty = 50,
}

#[flutter_rust_bridge::frb(sync)]
pub fn func_enum_with_discriminant_twin_sync(
    arg: EnumWithDiscriminantTwinSync,
) -> EnumWithDiscriminantTwinSync {
    arg
}

#[frb]
#[derive(Debug)]
pub struct NoteTwinSync {
    #[frb(default = "WeekdaysTwinSync.Sunday")]
    pub day: Box<WeekdaysTwinSync>,
    pub body: String,
}

#[flutter_rust_bridge::frb(sync)]
pub fn print_note_twin_sync(note: NoteTwinSync) -> Vec<u8> {
    info!("{:#?}", note);
    vec![1, 2, 3]
}

#[flutter_rust_bridge::frb(sync)]
pub fn handle_return_enum_twin_sync(input: String) -> Option<WeekdaysTwinSync> {
    match input.as_str() {
        "Monday" => Some(WeekdaysTwinSync::Monday),
        "Tuesday" => Some(WeekdaysTwinSync::Tuesday),
        "Wednesday" => Some(WeekdaysTwinSync::Wednesday),
        "Thursday" => Some(WeekdaysTwinSync::Thursday),
        "Friday" => Some(WeekdaysTwinSync::Friday),
        "Saturday" => Some(WeekdaysTwinSync::Saturday),
        "Sunday" => Some(WeekdaysTwinSync::Sunday),
        _ => None,
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn handle_enum_parameter_twin_sync(weekday: WeekdaysTwinSync) -> WeekdaysTwinSync {
    info!("The weekday is {:?}", weekday);
    weekday
}

#[derive(Debug, Clone)]
pub enum SpeedTwinSync {
    Unknown,
    GPS(f64),
}

#[derive(Debug, Clone)]
pub enum DistanceTwinSync {
    Unknown,
    Map(f64),
}

#[derive(Debug, Clone)]
pub enum MeasureTwinSync {
    Speed(Box<SpeedTwinSync>),
    Distance(Box<DistanceTwinSync>),
}

#[flutter_rust_bridge::frb(sync)]
pub fn multiply_by_ten_twin_sync(measure: MeasureTwinSync) -> Option<MeasureTwinSync> {
    match measure {
        MeasureTwinSync::Speed(b) => match *b {
            SpeedTwinSync::GPS(v) => Some(MeasureTwinSync::Speed(Box::new(SpeedTwinSync::GPS(
                v * 10.,
            )))),
            SpeedTwinSync::Unknown => None,
        },
        MeasureTwinSync::Distance(b) => match *b {
            DistanceTwinSync::Map(v) => Some(MeasureTwinSync::Distance(Box::new(
                DistanceTwinSync::Map(v * 10.),
            ))),
            DistanceTwinSync::Unknown => None,
        },
    }
}

#[frb]
#[derive(Debug)]
pub enum KitchenSinkTwinSync {
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
        #[frb(default = "KitchenSinkTwinSync.empty()")] Box<KitchenSinkTwinSync>,
    ),
    Optional(
        /// Comment on anonymous field
        #[frb(default = -1)]
        Option<i32>,
        Option<i32>,
    ),
    Buffer(Vec<u8>),
    Enums(#[frb(default = "WeekdaysTwinSync.Sunday")] WeekdaysTwinSync),
}

#[flutter_rust_bridge::frb(sync)]
pub fn handle_enum_struct_twin_sync(val: KitchenSinkTwinSync) -> KitchenSinkTwinSync {
    use KitchenSinkTwinSync::*;
    use WeekdaysTwinSync::*;
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
        Nested(val, _nested) => Nested(inc(val), Box::new(KitchenSinkTwinSync::Empty)),
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
