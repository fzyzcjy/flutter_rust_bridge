use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::dart::base::*;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub(crate) trait WireDartGeneratorInfoTrait {
    fn dart_wire_type(&self, target: Target) -> String;
}

impl<'a> WireDartGeneratorInfoTrait for BoxedWireDartGenerator<'a> {
    fn dart_wire_type(&self, target: Target) -> String {
        todo!()
    }
}

impl<'a> WireDartGeneratorInfoTrait for DartOpaqueWireDartGenerator<'a> {
    fn dart_wire_type(&self, target: Target) -> String {
        todo!()
    }
}

impl<'a> WireDartGeneratorInfoTrait for DelegateWireDartGenerator<'a> {
    fn dart_wire_type(&self, target: Target) -> String {
        todo!()
    }
}

impl<'a> WireDartGeneratorInfoTrait for DynamicWireDartGenerator<'a> {
    fn dart_wire_type(&self, target: Target) -> String {
        todo!()
    }
}

impl<'a> WireDartGeneratorInfoTrait for EnumRefWireDartGenerator<'a> {
    fn dart_wire_type(&self, target: Target) -> String {
        todo!()
    }
}

impl<'a> WireDartGeneratorInfoTrait for GeneralListWireDartGenerator<'a> {
    fn dart_wire_type(&self, target: Target) -> String {
        todo!()
    }
}

impl<'a> WireDartGeneratorInfoTrait for OptionalWireDartGenerator<'a> {
    fn dart_wire_type(&self, target: Target) -> String {
        todo!()
    }
}

impl<'a> WireDartGeneratorInfoTrait for OptionalListWireDartGenerator<'a> {
    fn dart_wire_type(&self, target: Target) -> String {
        todo!()
    }
}

impl<'a> WireDartGeneratorInfoTrait for PrimitiveWireDartGenerator<'a> {
    fn dart_wire_type(&self, target: Target) -> String {
        todo!()
    }
}

impl<'a> WireDartGeneratorInfoTrait for PrimitiveListWireDartGenerator<'a> {
    fn dart_wire_type(&self, target: Target) -> String {
        todo!()
    }
}

impl<'a> WireDartGeneratorInfoTrait for RecordWireDartGenerator<'a> {
    fn dart_wire_type(&self, target: Target) -> String {
        todo!()
    }
}

impl<'a> WireDartGeneratorInfoTrait for RustOpaqueWireDartGenerator<'a> {
    fn dart_wire_type(&self, target: Target) -> String {
        todo!()
    }
}

impl<'a> WireDartGeneratorInfoTrait for StructRefWireDartGenerator<'a> {
    fn dart_wire_type(&self, target: Target) -> String {
        todo!()
    }
}

impl<'a> WireDartGeneratorInfoTrait for UnencodableWireDartGenerator<'a> {
    fn dart_wire_type(&self, _target: Target) -> String {
        unreachable!()
    }
}
