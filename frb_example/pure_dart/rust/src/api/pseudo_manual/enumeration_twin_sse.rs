// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `enumeration.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use crate::api::pseudo_manual::misc_example_twin_sse::WeekdaysTwinSse;
use flutter_rust_bridge::frb;
use log::info;

pub enum EnumSimpleTwinSse {
    A,
    B,
}

#[flutter_rust_bridge::frb(serialize)]
pub fn func_enum_simple_twin_sse(arg: EnumSimpleTwinSse) -> EnumSimpleTwinSse {
    arg
}

pub enum EnumWithItemMixedTwinSse {
    A,
    B(Vec<u8>),
    C { c_field: String },
}

#[flutter_rust_bridge::frb(serialize)]
pub fn func_enum_with_item_mixed_twin_sse(
    arg: EnumWithItemMixedTwinSse,
) -> EnumWithItemMixedTwinSse {
    arg
}

pub enum EnumWithItemTupleTwinSse {
    A(Vec<u8>),
    B(Vec<i32>),
}

#[flutter_rust_bridge::frb(serialize)]
pub fn func_enum_with_item_tuple_twin_sse(
    arg: EnumWithItemTupleTwinSse,
) -> EnumWithItemTupleTwinSse {
    arg
}

pub enum EnumWithItemStructTwinSse {
    A { a_field: Vec<u8> },
    B { b_field: Vec<i32> },
}

#[flutter_rust_bridge::frb(serialize)]
pub fn func_enum_with_item_struct_twin_sse(
    arg: EnumWithItemStructTwinSse,
) -> EnumWithItemStructTwinSse {
    arg
}

// #1674
pub enum EnumWithDiscriminantTwinSse {
    OneHundred = 100,
    Fifty = 50,
}

#[flutter_rust_bridge::frb(serialize)]
pub fn func_enum_with_discriminant_twin_sse(
    arg: EnumWithDiscriminantTwinSse,
) -> EnumWithDiscriminantTwinSse {
    arg
}

#[frb]
#[derive(Debug)]
pub struct NoteTwinSse {
    #[frb(default = "WeekdaysTwinSse.Sunday")]
    pub day: Box<WeekdaysTwinSse>,
    pub body: String,
}

#[flutter_rust_bridge::frb(serialize)]
pub fn print_note_twin_sse(note: NoteTwinSse) -> Vec<u8> {
    info!("{:#?}", note);
    vec![1, 2, 3]
}

#[flutter_rust_bridge::frb(serialize)]
pub fn handle_return_enum_twin_sse(input: String) -> Option<WeekdaysTwinSse> {
    match input.as_str() {
        "Monday" => Some(WeekdaysTwinSse::Monday),
        "Tuesday" => Some(WeekdaysTwinSse::Tuesday),
        "Wednesday" => Some(WeekdaysTwinSse::Wednesday),
        "Thursday" => Some(WeekdaysTwinSse::Thursday),
        "Friday" => Some(WeekdaysTwinSse::Friday),
        "Saturday" => Some(WeekdaysTwinSse::Saturday),
        "Sunday" => Some(WeekdaysTwinSse::Sunday),
        _ => None,
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub fn handle_enum_parameter_twin_sse(weekday: WeekdaysTwinSse) -> WeekdaysTwinSse {
    info!("The weekday is {:?}", weekday);
    weekday
}

#[derive(Debug, Clone)]
pub enum SpeedTwinSse {
    Unknown,
    GPS(f64),
}

#[derive(Debug, Clone)]
pub enum DistanceTwinSse {
    Unknown,
    Map(f64),
}

#[derive(Debug, Clone)]
pub enum MeasureTwinSse {
    Speed(Box<SpeedTwinSse>),
    Distance(Box<DistanceTwinSse>),
}

#[flutter_rust_bridge::frb(serialize)]
pub fn multiply_by_ten_twin_sse(measure: MeasureTwinSse) -> Option<MeasureTwinSse> {
    match measure {
        MeasureTwinSse::Speed(b) => match *b {
            SpeedTwinSse::GPS(v) => {
                Some(MeasureTwinSse::Speed(Box::new(SpeedTwinSse::GPS(v * 10.))))
            }
            SpeedTwinSse::Unknown => None,
        },
        MeasureTwinSse::Distance(b) => match *b {
            DistanceTwinSse::Map(v) => Some(MeasureTwinSse::Distance(Box::new(
                DistanceTwinSse::Map(v * 10.),
            ))),
            DistanceTwinSse::Unknown => None,
        },
    }
}

#[frb]
#[derive(Debug)]
pub enum KitchenSinkTwinSse {
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
        #[frb(default = "KitchenSinkTwinSse.empty()")] Box<KitchenSinkTwinSse>,
    ),
    Optional(
        /// Comment on anonymous field
        #[frb(default = -1)]
        Option<i32>,
        Option<i32>,
    ),
    Buffer(Vec<u8>),
    Enums(#[frb(default = "WeekdaysTwinSse.Sunday")] WeekdaysTwinSse),
}

#[flutter_rust_bridge::frb(serialize)]
pub fn handle_enum_struct_twin_sse(val: KitchenSinkTwinSse) -> KitchenSinkTwinSse {
    use KitchenSinkTwinSse::*;
    use WeekdaysTwinSse::*;
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
        Nested(val, _nested) => Nested(inc(val), Box::new(KitchenSinkTwinSse::Empty)),
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
