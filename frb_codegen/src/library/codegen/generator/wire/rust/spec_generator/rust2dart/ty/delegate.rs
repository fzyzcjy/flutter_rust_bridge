use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::rust2dart::ty::WireRustGeneratorRust2DartTrait;
use crate::codegen::ir::ty::delegate::{IrTypeDelegate, IrTypeDelegatePrimitiveEnum};
use itertools::Itertools;

impl<'a> WireRustGeneratorRust2DartTrait for DelegateWireRustGenerator<'a> {}
