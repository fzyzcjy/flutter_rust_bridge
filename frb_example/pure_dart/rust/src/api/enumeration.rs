use crate::api::misc_example::Weekdays;
use flutter_rust_bridge::{frb, ZeroCopyBuffer};
use log::info;

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
    B(Vec<i32>),
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

#[derive(Debug)]
pub struct Note {
    #[frb(default = "Weekdays.Sunday")]
    pub day: Box<Weekdays>,
    pub body: String,
}

pub fn print_note(note: Note) -> ZeroCopyBuffer<Vec<u8>> {
    info!("{:#?}", note);
    ZeroCopyBuffer(vec![1, 2, 3])
}

pub fn handle_return_enum(input: String) -> Option<Weekdays> {
    match input.as_str() {
        "Monday" => Some(Weekdays::Monday),
        "Tuesday" => Some(Weekdays::Tuesday),
        "Wednesday" => Some(Weekdays::Wednesday),
        "Thursday" => Some(Weekdays::Thursday),
        "Friday" => Some(Weekdays::Friday),
        "Saturday" => Some(Weekdays::Saturday),
        "Sunday" => Some(Weekdays::Sunday),
        _ => None,
    }
}

pub fn handle_enum_parameter(weekday: Weekdays) -> Weekdays {
    info!("The weekday is {:?}", weekday);
    weekday
}

#[derive(Debug, Clone)]
pub enum Speed {
    Unknown,
    GPS(f64),
}

#[derive(Debug, Clone)]
pub enum Distance {
    Unknown,
    Map(f64),
}

#[derive(Debug, Clone)]
pub enum Measure {
    Speed(Box<Speed>),
    Distance(Box<Distance>),
}

pub fn multiply_by_ten(measure: Measure) -> Option<Measure> {
    match measure {
        Measure::Speed(b) => match *b {
            Speed::GPS(v) => Some(Measure::Speed(Box::new(Speed::GPS(v * 10.)))),
            Speed::Unknown => None,
        },
        Measure::Distance(b) => match *b {
            Distance::Map(v) => Some(Measure::Distance(Box::new(Distance::Map(v * 10.)))),
            Distance::Unknown => None,
        },
    }
}
