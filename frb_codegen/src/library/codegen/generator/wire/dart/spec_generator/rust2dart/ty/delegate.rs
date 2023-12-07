use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::rust2dart::misc::gen_wire2api_simple_type_cast;
use crate::codegen::generator::wire::dart::spec_generator::rust2dart::ty::WireDartGeneratorRust2DartTrait;
use crate::codegen::ir::ty::delegate::{
    IrTypeDelegate, IrTypeDelegateArrayMode, IrTypeDelegatePrimitiveEnum, IrTypeDelegateTime,
};
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::library::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;

impl<'a> WireDartGeneratorRust2DartTrait for DelegateWireDartGenerator<'a> {}
