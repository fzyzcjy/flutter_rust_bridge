use crate::codegen::generator::api_dart::base::*;
use crate::codegen::ir::ty::delegate::{
    IrTypeDelegate, IrTypeDelegateArray, IrTypeDelegatePrimitiveEnum, IrTypeDelegateTime,
};
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::{IrType, IrTypeTrait};
use convert_case::{Case, Casing};
use enum_dispatch::enum_dispatch;
use itertools::Itertools;

#[enum_dispatch]
pub(crate) trait ApiDartGeneratorRustWireTypeTrait {
    fn rust_wire_type(&self) -> String;
}

impl<'a> ApiDartGeneratorRustWireTypeTrait for BoxedApiDartGenerator<'a> {
    fn rust_wire_type(&self) -> String {
        todo!()
    }
}

impl<'a> ApiDartGeneratorRustWireTypeTrait for DartOpaqueApiDartGenerator<'a> {
    fn rust_wire_type(&self) -> String {
        todo!()
    }
}

impl<'a> ApiDartGeneratorRustWireTypeTrait for DelegateApiDartGenerator<'a> {
    fn rust_wire_type(&self) -> String {
        todo!()
    }
}

impl IrTypeDelegateArray {
    pub(crate) fn rust_wire_type(&self, context: ApiDartGeneratorContext) -> String {
        todo!()
    }
}

impl<'a> ApiDartGeneratorRustWireTypeTrait for DynamicApiDartGenerator<'a> {
    fn rust_wire_type(&self) -> String {
        todo!()
    }
}

impl<'a> ApiDartGeneratorRustWireTypeTrait for EnumRefApiDartGenerator<'a> {
    fn rust_wire_type(&self) -> String {
        todo!()
    }
}

impl<'a> ApiDartGeneratorRustWireTypeTrait for GeneralListApiDartGenerator<'a> {
    fn rust_wire_type(&self) -> String {
        todo!()
    }
}

impl<'a> ApiDartGeneratorRustWireTypeTrait for OptionalApiDartGenerator<'a> {
    fn rust_wire_type(&self) -> String {
        todo!()
    }
}

impl<'a> ApiDartGeneratorRustWireTypeTrait for OptionalListApiDartGenerator<'a> {
    fn rust_wire_type(&self) -> String {
        todo!()
    }
}

impl<'a> ApiDartGeneratorRustWireTypeTrait for PrimitiveApiDartGenerator<'a> {
    fn rust_wire_type(&self) -> String {
        todo!()
    }
}

impl<'a> ApiDartGeneratorRustWireTypeTrait for PrimitiveListApiDartGenerator<'a> {
    fn rust_wire_type(&self) -> String {
        todo!()
    }
}

impl<'a> ApiDartGeneratorRustWireTypeTrait for RecordApiDartGenerator<'a> {
    fn rust_wire_type(&self) -> String {
        todo!()
    }
}

impl<'a> ApiDartGeneratorRustWireTypeTrait for RustOpaqueApiDartGenerator<'a> {
    fn rust_wire_type(&self) -> String {
        todo!()
    }
}

impl<'a> ApiDartGeneratorRustWireTypeTrait for StructRefApiDartGenerator<'a> {
    fn rust_wire_type(&self) -> String {
        todo!()
    }
}

impl<'a> ApiDartGeneratorRustWireTypeTrait for UnencodableApiDartGenerator<'a> {
    fn rust_wire_type(&self) -> String {
        todo!()
    }
}
