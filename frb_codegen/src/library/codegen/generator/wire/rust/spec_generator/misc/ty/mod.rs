use crate::codegen::generator::wire::rust::spec_generator::base::*;
use convert_case::{Case, Casing};

mod boxed;
mod dart_fn;
mod dart_opaque;
mod delegate;
mod dynamic;
mod enumeration;
mod general_list;
mod optional;
mod ownership;
mod primitive;
mod primitive_list;
mod record;
mod rust_auto_opaque;
mod rust_opaque;
mod structure;
mod unencodable;

use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::codec::structs::CodecMode;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub(crate) trait WireRustGeneratorMiscTrait {
    fn wrapper_struct_name(&self) -> Option<String> {
        None
    }

    fn generate_static_checks(&self) -> Option<String> {
        None
    }

    fn generate_imports(&self) -> Option<Vec<String>> {
        None
    }

    fn generate_related_funcs(&self) -> Acc<WireRustOutputCode> {
        Default::default()
    }

    fn generate_wire_func_call_decode(&self, name: &str, codec_mode: CodecMode) -> String {
        format!(
            "{name}.{code_mode}_decode()",
            code_mode = codec_mode.to_string().to_case(Case::Snake)
        )
    }
}
