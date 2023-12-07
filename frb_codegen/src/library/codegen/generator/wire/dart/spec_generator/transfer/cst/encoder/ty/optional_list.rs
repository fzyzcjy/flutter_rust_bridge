use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::encoder::ty::general_list::general_or_optional_list_dart_wire_type;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::cst::encoder::ty::WireDartTransferCstGeneratorEncoderTrait;
use crate::codegen::ir::ty::IrTypeTrait;

impl<'a> WireDartTransferCstGeneratorEncoderTrait for OptionalListWireDartTransferCstGenerator<'a> {}
