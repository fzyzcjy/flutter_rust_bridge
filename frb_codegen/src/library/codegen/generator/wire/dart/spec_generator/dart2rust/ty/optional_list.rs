use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::dart2rust::ty::general_list::general_or_optional_list_dart_wire_type;
use crate::codegen::generator::wire::dart::spec_generator::dart2rust::ty::WireDartGeneratorDart2RustTrait;
use crate::codegen::ir::ty::IrTypeTrait;

impl<'a> WireDartGeneratorDart2RustTrait for OptionalListWireDartGenerator<'a> {}
