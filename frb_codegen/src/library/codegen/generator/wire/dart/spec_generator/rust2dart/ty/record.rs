use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::rust2dart::ty::WireDartGeneratorRust2DartTrait;
use crate::library::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;

impl<'a> WireDartGeneratorRust2DartTrait for RecordWireDartGenerator<'a> {}
