use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::rust2dart::ty::WireRustGeneratorRust2DartTrait;
use crate::codegen::ir::namespace::NamespacedName;
use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::enumeration::IrVariantKind;
use crate::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;

impl<'a> WireRustGeneratorRust2DartTrait for EnumRefWireRustGenerator<'a> {}
