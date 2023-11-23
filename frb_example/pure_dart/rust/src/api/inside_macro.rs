// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync"]}

macro_rules! generate_struct {
    ($name:ident) => {
        pub struct $name {
            pub data: i32,
        }

        pub fn func_macro_struct(arg: $name) -> $name {
            arg
        }
    };
}

generate_struct!(MacroStruct);
