use crate::codegen::generator::api_dart::spec_generator::class::ty::rust_opaque::generalized_rust_opaque_generate_class;
use crate::codegen::generator::api_dart::spec_generator::class::ty::ApiDartGeneratorClassTrait;
use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use crate::codegen::ir::ty::IrType::RustAutoOpaque;
use crate::codegen::ir::ty::IrTypeTrait;
use crate::library::codegen::generator::api_dart::spec_generator::base::*;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;

impl<'a> ApiDartGeneratorClassTrait for RustAutoOpaqueApiDartGenerator<'a> {
    fn generate_class(&self) -> Option<ApiDartGeneratedClass> {
        Some(generalized_rust_opaque_generate_class(
            self.ir.clone().into(),
            self.ir.namespace.clone(),
            // Even if the `modifier` is different, the class should be the same
            Some(format!(
                "RustAutoOpaque_DeduplicateKey_{}_{}",
                self.ir.namespace.joined_path,
                self.ir.inner.safe_ident()
            )),
            self.context,
            "RustAutoOpaque",
        ))
    }
}
