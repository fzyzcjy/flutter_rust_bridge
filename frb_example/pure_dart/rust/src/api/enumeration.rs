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

#[frb]
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
