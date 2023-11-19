use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::misc::ty::WireRustGeneratorMiscTrait;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::IrTypeTrait;

impl<'a> WireRustGeneratorMiscTrait for PrimitiveListWireRustGenerator<'a> {}
