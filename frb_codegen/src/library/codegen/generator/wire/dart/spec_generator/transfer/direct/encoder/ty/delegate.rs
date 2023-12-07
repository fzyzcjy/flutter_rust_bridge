use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::rust2dart::misc::generate_impl_into_into_dart;
use crate::codegen::generator::wire::rust::spec_generator::rust2dart::ty::enumeration::parse_wrapper_name_into_dart_name_and_self_path;
use crate::codegen::generator::wire::rust::spec_generator::rust2dart::ty::WireRustGeneratorRust2DartTrait;
use crate::codegen::ir::ty::delegate::{IrTypeDelegate, IrTypeDelegatePrimitiveEnum};
use crate::forward_delegate_primitive_enum;
use itertools::Itertools;

impl<'a> WireRustGeneratorRust2DartTrait for DelegateWireRustGenerator<'a> {}
