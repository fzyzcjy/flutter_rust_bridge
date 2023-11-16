use crate::codegen::generator::dart_api::base::*;
use enum_dispatch::enum_dispatch;
use syn::token::Dyn;

#[enum_dispatch]
pub(crate) trait DartApiGeneratorDeclTrait {
    fn dart_api_type(&self) -> String;
}

impl DartApiGeneratorDeclTrait for BoxedDartApiGenerator {
    fn dart_api_type(&self) -> String {
        self.inner.dart_api_type()
    }
}

impl DartApiGeneratorDeclTrait for DartOpaqueDartApiGenerator {
    fn dart_api_type(&self) -> String {
        todo!()
    }
}

impl DartApiGeneratorDeclTrait for DelegateDartApiGenerator {
    fn dart_api_type(&self) -> String {
        todo!()
    }
}

impl DartApiGeneratorDeclTrait for DynamicDartApiGenerator {
    fn dart_api_type(&self) -> String {
        todo!()
    }
}

impl DartApiGeneratorDeclTrait for EnumRefDartApiGenerator {
    fn dart_api_type(&self) -> String {
        todo!()
    }
}

impl DartApiGeneratorDeclTrait for GeneralListDartApiGenerator {
    fn dart_api_type(&self) -> String {
        todo!()
    }
}

impl DartApiGeneratorDeclTrait for OptionalDartApiGenerator {
    fn dart_api_type(&self) -> String {
        todo!()
    }
}

impl DartApiGeneratorDeclTrait for OptionalListDartApiGenerator {
    fn dart_api_type(&self) -> String {
        todo!()
    }
}

impl DartApiGeneratorDeclTrait for PrimitiveDartApiGenerator {
    fn dart_api_type(&self) -> String {
        todo!()
    }
}

impl DartApiGeneratorDeclTrait for PrimitiveListDartApiGenerator {
    fn dart_api_type(&self) -> String {
        todo!()
    }
}

impl DartApiGeneratorDeclTrait for RecordDartApiGenerator {
    fn dart_api_type(&self) -> String {
        todo!()
    }
}

impl DartApiGeneratorDeclTrait for RustOpaqueDartApiGenerator {
    fn dart_api_type(&self) -> String {
        todo!()
    }
}

impl DartApiGeneratorDeclTrait for StructRefDartApiGenerator {
    fn dart_api_type(&self) -> String {
        todo!()
    }
}

impl DartApiGeneratorDeclTrait for UnencodableDartApiGenerator {
    fn dart_api_type(&self) -> String {
        todo!()
    }
}
