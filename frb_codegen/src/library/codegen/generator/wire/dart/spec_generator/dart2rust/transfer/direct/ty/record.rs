use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::rust2dart::ty::WireRustGeneratorRust2DartTrait;
use crate::codegen::ir::pack::IrPack;
use itertools::Itertools;

impl<'a> WireRustGeneratorRust2DartTrait for RecordWireRustGenerator<'a> {}
