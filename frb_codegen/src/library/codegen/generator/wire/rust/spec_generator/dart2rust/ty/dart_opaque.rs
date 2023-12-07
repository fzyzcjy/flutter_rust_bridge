use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::{Target, TargetOrCommon};
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::dart2rust::ty::WireRustGeneratorDart2RustTrait;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::{
    ExternFunc, ExternFuncParam,
};
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::ir::ty::IrTypeTrait;
use crate::misc::consts::HANDLER_NAME;

impl<'a> WireRustGeneratorDart2RustTrait for DartOpaqueWireRustGenerator<'a> {}
