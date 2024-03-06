// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "rustAsync", "sse", "sync sse", "rustAsync sse"]}

use flutter_rust_bridge::frb;

macro_rules! generate_struct {
    ($name:ident) => {
        pub struct $name {
            pub data: i32,
        }

        pub fn func_macro_struct_twin_normal(arg: $name) -> $name {
            arg
        }
    };
}

generate_struct!(MacroStruct);

macro_rules! generate_another_struct {
    () => {
        #[frb]
        pub struct AnotherMacroStructTwinNormal {
            pub data: i32,
            #[frb(non_final)]
            pub non_final_data: i32,
        }
        #[allow(unused)]
        pub fn another_macro_struct_twin_normal() -> AnotherMacroStructTwinNormal {
            AnotherMacroStructTwinNormal {
                data: 123,
                non_final_data: 0,
            }
        }
    };
}

generate_another_struct!();
