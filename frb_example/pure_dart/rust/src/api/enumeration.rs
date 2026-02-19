use crate::api::misc_example::WeekdaysTwinNormal;
use crate::frb_generated::StreamSink;
use flutter_rust_bridge::frb;
use log::info;
use serde::{Deserialize, Serialize};

pub enum EnumSimpleTwinNormal {
    A,
    B,
}

pub fn func_enum_simple_twin_normal(arg: EnumSimpleTwinNormal) -> EnumSimpleTwinNormal {
    arg
}

pub enum EnumWithItemMixedTwinNormal {
    A,
    B(Vec<u8>),
    C { c_field: String },
}

pub fn func_enum_with_item_mixed_twin_normal(
    arg: EnumWithItemMixedTwinNormal,
) -> EnumWithItemMixedTwinNormal {
    arg
}

pub enum EnumWithItemTupleTwinNormal {
    A(Vec<u8>),
    B(i32),
}

pub fn func_enum_with_item_tuple_twin_normal(
    arg: EnumWithItemTupleTwinNormal,
) -> EnumWithItemTupleTwinNormal {
    arg
}

pub enum EnumWithItemStructTwinNormal {
    A { a_field: Vec<u8> },
    B { b_field: Vec<i32> },
}

pub fn func_enum_with_item_struct_twin_normal(
    arg: EnumWithItemStructTwinNormal,
) -> EnumWithItemStructTwinNormal {
    arg
}

// #1674
pub enum EnumWithDiscriminantTwinNormal {
    OneHundred = 100,
    Fifty = 50,
}

pub fn func_enum_with_discriminant_twin_normal(
    arg: EnumWithDiscriminantTwinNormal,
) -> EnumWithDiscriminantTwinNormal {
    arg
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChangeTwinNormal<T> {
    Created { data: T },
    Updated { id: String, data: T },
    Deleted { id: String },
}

pub type ChangeStringTwinNormal = ChangeTwinNormal<String>;

pub fn func_change_twin_normal(arg: ChangeStringTwinNormal) -> ChangeStringTwinNormal {
    arg
}

pub type ChangeMapTwinNormal = ChangeTwinNormal<std::collections::HashMap<String, String>>;

pub fn func_change_map_twin_normal(arg: ChangeMapTwinNormal) -> ChangeMapTwinNormal {
    arg
}

// Test nested generic enum inside StreamSink - this tests auto-naming for nested generics
pub type ChangeMapSinkTwinNormal = StreamSink<ChangeTwinNormal<std::collections::HashMap<String, String>>>;

pub fn func_change_map_sink_twin_normal(arg: ChangeMapSinkTwinNormal) -> ChangeMapSinkTwinNormal {
    arg
}

// Test case for bug: generic enum with #[frb(ignore)] and concrete type aliases with #[frb(non_opaque)]
// The template enum is ignored, but concrete instances through type aliases should generate Dart code
#[derive(Debug, Clone, Serialize, Deserialize)]
#[flutter_rust_bridge::frb(ignore)]
pub enum ChangeIgnoredTwinNormal<T> {
    Created { data: T },
    Updated { id: String, data: T },
    Deleted { id: String },
}

#[frb(non_opaque)]
pub type StringChangeTwinNormal = ChangeIgnoredTwinNormal<String>;

#[frb(non_opaque)]
pub type MapChangeTwinNormal = ChangeIgnoredTwinNormal<std::collections::HashMap<String, String>>;

pub fn func_block_change_twin_normal(arg: StringChangeTwinNormal) -> StringChangeTwinNormal {
    arg
}

pub fn func_map_change_twin_normal(arg: MapChangeTwinNormal) -> MapChangeTwinNormal {
    arg
}

pub struct BatchTwinNormal<T> {
    items: Vec<T>,
}

#[frb(non_opaque)]
pub type BatchTwinNormalChangeMapTwinNormal = BatchTwinNormal<ChangeIgnoredTwinNormal<std::collections::HashMap<String, String>>>;

pub fn func_batch_change_map_twin_normal(arg: BatchTwinNormalChangeMapTwinNormal) -> BatchTwinNormalChangeMapTwinNormal {
    arg
}

#[frb(non_opaque)]
pub struct AnotherBatchTwinNormal<T> {
    items: T,
}


#[frb(non_opaque)]
pub type AnotherBatchTwinNormalChangeMapTwinNormal = AnotherBatchTwinNormal<ChangeIgnoredTwinNormal<std::collections::HashMap<String, String>>>;

pub fn func_another_batch_twin_normal_change_map_twin_normal(arg: AnotherBatchTwinNormalChangeMapTwinNormal) -> AnotherBatchTwinNormalChangeMapTwinNormal {
    arg
}

#[frb]
#[derive(Debug)]
pub struct NoteTwinNormal {
    #[frb(default = "WeekdaysTwinNormal.Sunday")]
    pub day: Box<WeekdaysTwinNormal>,
    pub body: String,
}

pub fn print_note_twin_normal(note: NoteTwinNormal) -> Vec<u8> {
    info!("{:#?}", note);
    vec![1, 2, 3]
}

pub fn handle_return_enum_twin_normal(input: String) -> Option<WeekdaysTwinNormal> {
    match input.as_str() {
        "Monday" => Some(WeekdaysTwinNormal::Monday),
        "Tuesday" => Some(WeekdaysTwinNormal::Tuesday),
        "Wednesday" => Some(WeekdaysTwinNormal::Wednesday),
        "Thursday" => Some(WeekdaysTwinNormal::Thursday),
        "Friday" => Some(WeekdaysTwinNormal::Friday),
        "Saturday" => Some(WeekdaysTwinNormal::Saturday),
        "Sunday" => Some(WeekdaysTwinNormal::Sunday),
        _ => None,
    }
}

pub fn handle_enum_parameter_twin_normal(weekday: WeekdaysTwinNormal) -> WeekdaysTwinNormal {
    info!("The weekday is {:?}", weekday);
    weekday
}

#[derive(Debug, Clone)]
pub enum SpeedTwinNormal {
    Unknown,
    GPS(f64),
}

#[derive(Debug, Clone)]
pub enum DistanceTwinNormal {
    Unknown,
    Map(f64),
}

#[derive(Debug, Clone)]
pub enum MeasureTwinNormal {
    Speed(Box<SpeedTwinNormal>),
    Distance(Box<DistanceTwinNormal>),
}

pub fn multiply_by_ten_twin_normal(measure: MeasureTwinNormal) -> Option<MeasureTwinNormal> {
    match measure {
        MeasureTwinNormal::Speed(b) => match *b {
            SpeedTwinNormal::GPS(v) => Some(MeasureTwinNormal::Speed(Box::new(
                SpeedTwinNormal::GPS(v * 10.),
            ))),
            SpeedTwinNormal::Unknown => None,
        },
        MeasureTwinNormal::Distance(b) => match *b {
            DistanceTwinNormal::Map(v) => Some(MeasureTwinNormal::Distance(Box::new(
                DistanceTwinNormal::Map(v * 10.),
            ))),
            DistanceTwinNormal::Unknown => None,
        },
    }
}

#[frb]
#[derive(Debug)]
pub enum KitchenSinkTwinNormal {
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
        #[frb(default = "KitchenSinkTwinNormal.empty()")] Box<KitchenSinkTwinNormal>,
    ),
    Optional(
        /// Comment on anonymous field
        #[frb(default = -1)]
        Option<i32>,
        Option<i32>,
    ),
    Buffer(Vec<u8>),
    Enums(#[frb(default = "WeekdaysTwinNormal.Sunday")] WeekdaysTwinNormal),
}

pub fn handle_enum_struct_twin_normal(val: KitchenSinkTwinNormal) -> KitchenSinkTwinNormal {
    use KitchenSinkTwinNormal::*;
    use WeekdaysTwinNormal::*;
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
        Nested(val, _nested) => Nested(inc(val), Box::new(KitchenSinkTwinNormal::Empty)),
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
