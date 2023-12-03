use crate::codegen::generator::api_dart::spec_generator::class::ty::rust_opaque::generalized_rust_opaque_generate_class;
use crate::codegen::generator::api_dart::spec_generator::class::ty::ApiDartGeneratorClassTrait;
use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use crate::codegen::ir::ty::IrType::RustAutoOpaque;
use crate::library::codegen::generator::api_dart::spec_generator::base::*;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;

impl<'a> ApiDartGeneratorClassTrait for RustAutoOpaqueApiDartGenerator<'a> {}
