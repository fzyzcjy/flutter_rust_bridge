// NOTE: This file is mimicking how a human developer writes tests, 
// and is auto-generated from `inside_macro.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync"]}

use flutter_rust_bridge::frb;

macro_rules! generate_struct {
    ($name:ident) => {
        pub struct $name {
            pub data: i32,
        }

        pub async fn func_macro_struct_twin_rust_async(arg: $name) -> $name {
            arg
        }
    };
}

generate_struct!(MacroStruct);

macro_rules! generate_another_struct {
    () => {
        #[frb]
        pub struct AnotherMacroStructTwinRustAsync {
            pub data: i32,
            #[frb(non_final)]
            pub non_final_data: i32,
        }
        #[allow(unused)]
        pub async fn another_macro_struct_twin_rust_async() -> AnotherMacroStructTwinRustAsync {
            AnotherMacroStructTwinRustAsync {
                data: 123,
                non_final_data: 0,
            }
        }
    };
}

generate_another_struct!();
