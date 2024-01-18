// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `enumeration.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

use crate::api::pseudo_manual::misc_example_twin_rust_async::WeekdaysTwinRustAsync;
use flutter_rust_bridge::frb;
use log::info;

pub enum EnumSimpleTwinRustAsync {
    A,
    B,
}

pub async fn func_enum_simple_twin_rust_async(
    arg: EnumSimpleTwinRustAsync,
) -> EnumSimpleTwinRustAsync {
    arg
}

pub enum EnumWithItemMixedTwinRustAsync {
    A,
    B(Vec<u8>),
    C { c_field: String },
}

pub async fn func_enum_with_item_mixed_twin_rust_async(
    arg: EnumWithItemMixedTwinRustAsync,
) -> EnumWithItemMixedTwinRustAsync {
    arg
}

pub enum EnumWithItemTupleTwinRustAsync {
    A(Vec<u8>),
    B(Vec<i32>),
}

pub async fn func_enum_with_item_tuple_twin_rust_async(
    arg: EnumWithItemTupleTwinRustAsync,
) -> EnumWithItemTupleTwinRustAsync {
    arg
}

pub enum EnumWithItemStructTwinRustAsync {
    A { a_field: Vec<u8> },
    B { b_field: Vec<i32> },
}

pub async fn func_enum_with_item_struct_twin_rust_async(
    arg: EnumWithItemStructTwinRustAsync,
) -> EnumWithItemStructTwinRustAsync {
    arg
}

// #1674
pub enum EnumWithDiscriminantTwinRustAsync {
    OneHundred = 100,
    Fifty = 50,
}

pub async fn func_enum_with_discriminant_twin_rust_async(
    arg: EnumWithDiscriminantTwinRustAsync,
) -> EnumWithDiscriminantTwinRustAsync {
    arg
}

#[frb]
#[derive(Debug)]
pub struct NoteTwinRustAsync {
    #[frb(default = "WeekdaysTwinRustAsync.Sunday")]
    pub day: Box<WeekdaysTwinRustAsync>,
    pub body: String,
}

pub async fn print_note_twin_rust_async(note: NoteTwinRustAsync) -> Vec<u8> {
    info!("{:#?}", note);
    vec![1, 2, 3]
}

pub async fn handle_return_enum_twin_rust_async(input: String) -> Option<WeekdaysTwinRustAsync> {
    match input.as_str() {
        "Monday" => Some(WeekdaysTwinRustAsync::Monday),
        "Tuesday" => Some(WeekdaysTwinRustAsync::Tuesday),
        "Wednesday" => Some(WeekdaysTwinRustAsync::Wednesday),
        "Thursday" => Some(WeekdaysTwinRustAsync::Thursday),
        "Friday" => Some(WeekdaysTwinRustAsync::Friday),
        "Saturday" => Some(WeekdaysTwinRustAsync::Saturday),
        "Sunday" => Some(WeekdaysTwinRustAsync::Sunday),
        _ => None,
    }
}

pub async fn handle_enum_parameter_twin_rust_async(
    weekday: WeekdaysTwinRustAsync,
) -> WeekdaysTwinRustAsync {
    info!("The weekday is {:?}", weekday);
    weekday
}

#[derive(Debug, Clone)]
pub enum SpeedTwinRustAsync {
    Unknown,
    GPS(f64),
}

#[derive(Debug, Clone)]
pub enum DistanceTwinRustAsync {
    Unknown,
    Map(f64),
}

#[derive(Debug, Clone)]
pub enum MeasureTwinRustAsync {
    Speed(Box<SpeedTwinRustAsync>),
    Distance(Box<DistanceTwinRustAsync>),
}

pub async fn multiply_by_ten_twin_rust_async(
    measure: MeasureTwinRustAsync,
) -> Option<MeasureTwinRustAsync> {
    match measure {
        MeasureTwinRustAsync::Speed(b) => match *b {
            SpeedTwinRustAsync::GPS(v) => Some(MeasureTwinRustAsync::Speed(Box::new(
                SpeedTwinRustAsync::GPS(v * 10.),
            ))),
            SpeedTwinRustAsync::Unknown => None,
        },
        MeasureTwinRustAsync::Distance(b) => match *b {
            DistanceTwinRustAsync::Map(v) => Some(MeasureTwinRustAsync::Distance(Box::new(
                DistanceTwinRustAsync::Map(v * 10.),
            ))),
            DistanceTwinRustAsync::Unknown => None,
        },
    }
}

#[frb]
#[derive(Debug)]
pub enum KitchenSinkTwinRustAsync {
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
        #[frb(default = "KitchenSinkTwinRustAsync.empty()")] Box<KitchenSinkTwinRustAsync>,
    ),
    Optional(
        /// Comment on anonymous field
        #[frb(default = -1)]
        Option<i32>,
        Option<i32>,
    ),
    Buffer(Vec<u8>),
    Enums(#[frb(default = "WeekdaysTwinRustAsync.Sunday")] WeekdaysTwinRustAsync),
}

pub async fn handle_enum_struct_twin_rust_async(
    val: KitchenSinkTwinRustAsync,
) -> KitchenSinkTwinRustAsync {
    use KitchenSinkTwinRustAsync::*;
    use WeekdaysTwinRustAsync::*;
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
        Nested(val, _nested) => Nested(inc(val), Box::new(KitchenSinkTwinRustAsync::Empty)),
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
