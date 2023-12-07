use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::rust2dart::misc::gen_wire2api_simple_type_cast;
use crate::codegen::generator::wire::dart::spec_generator::rust2dart::ty::WireDartGeneratorRust2DartTrait;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;

impl<'a> WireDartGeneratorRust2DartTrait for PrimitiveWireDartGenerator<'a> {}
