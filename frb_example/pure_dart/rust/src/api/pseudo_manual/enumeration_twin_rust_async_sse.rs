// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `enumeration.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use crate::api::pseudo_manual::misc_example_twin_rust_async_sse::WeekdaysTwinRustAsyncSse;
use flutter_rust_bridge::frb;
use log::info;

pub enum EnumSimpleTwinRustAsyncSse {
    A,
    B,
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn func_enum_simple_twin_rust_async_sse(
    arg: EnumSimpleTwinRustAsyncSse,
) -> EnumSimpleTwinRustAsyncSse {
    arg
}

pub enum EnumWithItemMixedTwinRustAsyncSse {
    A,
    B(Vec<u8>),
    C { c_field: String },
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn func_enum_with_item_mixed_twin_rust_async_sse(
    arg: EnumWithItemMixedTwinRustAsyncSse,
) -> EnumWithItemMixedTwinRustAsyncSse {
    arg
}

pub enum EnumWithItemTupleTwinRustAsyncSse {
    A(Vec<u8>),
    B(Vec<i32>),
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn func_enum_with_item_tuple_twin_rust_async_sse(
    arg: EnumWithItemTupleTwinRustAsyncSse,
) -> EnumWithItemTupleTwinRustAsyncSse {
    arg
}

pub enum EnumWithItemStructTwinRustAsyncSse {
    A { a_field: Vec<u8> },
    B { b_field: Vec<i32> },
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn func_enum_with_item_struct_twin_rust_async_sse(
    arg: EnumWithItemStructTwinRustAsyncSse,
) -> EnumWithItemStructTwinRustAsyncSse {
    arg
}

// #1674
pub enum EnumWithDiscriminantTwinRustAsyncSse {
    OneHundred = 100,
    Fifty = 50,
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn func_enum_with_discriminant_twin_rust_async_sse(
    arg: EnumWithDiscriminantTwinRustAsyncSse,
) -> EnumWithDiscriminantTwinRustAsyncSse {
    arg
}

#[frb]
#[derive(Debug)]
pub struct NoteTwinRustAsyncSse {
    #[frb(default = "WeekdaysTwinRustAsyncSse.Sunday")]
    pub day: Box<WeekdaysTwinRustAsyncSse>,
    pub body: String,
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn print_note_twin_rust_async_sse(note: NoteTwinRustAsyncSse) -> Vec<u8> {
    info!("{:#?}", note);
    vec![1, 2, 3]
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn handle_return_enum_twin_rust_async_sse(
    input: String,
) -> Option<WeekdaysTwinRustAsyncSse> {
    match input.as_str() {
        "Monday" => Some(WeekdaysTwinRustAsyncSse::Monday),
        "Tuesday" => Some(WeekdaysTwinRustAsyncSse::Tuesday),
        "Wednesday" => Some(WeekdaysTwinRustAsyncSse::Wednesday),
        "Thursday" => Some(WeekdaysTwinRustAsyncSse::Thursday),
        "Friday" => Some(WeekdaysTwinRustAsyncSse::Friday),
        "Saturday" => Some(WeekdaysTwinRustAsyncSse::Saturday),
        "Sunday" => Some(WeekdaysTwinRustAsyncSse::Sunday),
        _ => None,
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn handle_enum_parameter_twin_rust_async_sse(
    weekday: WeekdaysTwinRustAsyncSse,
) -> WeekdaysTwinRustAsyncSse {
    info!("The weekday is {:?}", weekday);
    weekday
}

#[derive(Debug, Clone)]
pub enum SpeedTwinRustAsyncSse {
    Unknown,
    GPS(f64),
}

#[derive(Debug, Clone)]
pub enum DistanceTwinRustAsyncSse {
    Unknown,
    Map(f64),
}

#[derive(Debug, Clone)]
pub enum MeasureTwinRustAsyncSse {
    Speed(Box<SpeedTwinRustAsyncSse>),
    Distance(Box<DistanceTwinRustAsyncSse>),
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn multiply_by_ten_twin_rust_async_sse(
    measure: MeasureTwinRustAsyncSse,
) -> Option<MeasureTwinRustAsyncSse> {
    match measure {
        MeasureTwinRustAsyncSse::Speed(b) => match *b {
            SpeedTwinRustAsyncSse::GPS(v) => Some(MeasureTwinRustAsyncSse::Speed(Box::new(
                SpeedTwinRustAsyncSse::GPS(v * 10.),
            ))),
            SpeedTwinRustAsyncSse::Unknown => None,
        },
        MeasureTwinRustAsyncSse::Distance(b) => match *b {
            DistanceTwinRustAsyncSse::Map(v) => Some(MeasureTwinRustAsyncSse::Distance(Box::new(
                DistanceTwinRustAsyncSse::Map(v * 10.),
            ))),
            DistanceTwinRustAsyncSse::Unknown => None,
        },
    }
}

#[frb]
#[derive(Debug)]
pub enum KitchenSinkTwinRustAsyncSse {
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
        #[frb(default = "KitchenSinkTwinRustAsyncSse.empty()")] Box<KitchenSinkTwinRustAsyncSse>,
    ),
    Optional(
        /// Comment on anonymous field
        #[frb(default = -1)]
        Option<i32>,
        Option<i32>,
    ),
    Buffer(Vec<u8>),
    Enums(#[frb(default = "WeekdaysTwinRustAsyncSse.Sunday")] WeekdaysTwinRustAsyncSse),
}

#[flutter_rust_bridge::frb(serialize)]
pub async fn handle_enum_struct_twin_rust_async_sse(
    val: KitchenSinkTwinRustAsyncSse,
) -> KitchenSinkTwinRustAsyncSse {
    use KitchenSinkTwinRustAsyncSse::*;
    use WeekdaysTwinRustAsyncSse::*;
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
        Nested(val, _nested) => Nested(inc(val), Box::new(KitchenSinkTwinRustAsyncSse::Empty)),
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
