use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::dart2rust::misc::dart_wire_type_from_rust_wire_type_or_wasm;
use crate::codegen::generator::wire::dart::spec_generator::dart2rust::ty::rust_opaque::dart_or_rust_opaque_dart_wire_type;
use crate::codegen::generator::wire::dart::spec_generator::dart2rust::ty::WireDartGeneratorDart2RustTrait;
use crate::codegen::ir::ty::ownership::IrTypeOwnershipMode;
use crate::codegen::ir::ty::IrTypeTrait;

impl<'a> WireDartGeneratorDart2RustTrait for RustAutoOpaqueWireDartGenerator<'a> {}
