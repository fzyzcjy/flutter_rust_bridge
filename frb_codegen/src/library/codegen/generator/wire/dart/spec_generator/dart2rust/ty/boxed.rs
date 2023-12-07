use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::is_js_value;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::dart2rust::ty::primitive::dart_native_type_of_primitive;
use crate::codegen::generator::wire::dart::spec_generator::dart2rust::ty::WireDartGeneratorDart2RustTrait;
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGenerator;
use crate::codegen::ir::ty::rust_opaque::IrTypeRustOpaque;
use crate::codegen::ir::ty::IrType::StructRef;
use crate::codegen::ir::ty::{IrType, IrTypeTrait};
use crate::library::codegen::generator::wire::rust::spec_generator::dart2rust::ty::WireRustGeneratorDart2RustTrait;

impl<'a> WireDartGeneratorDart2RustTrait for BoxedWireDartGenerator<'a> {}
