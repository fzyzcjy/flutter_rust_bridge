use crate::codegen::generator::wire::rust::base::*;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub(crate) trait WireRustGeneratorRustWireTypeTrait {
    fn rust_wire_type(&self) -> String;
}

impl<'a> WireRustGeneratorRustWireTypeTrait for BoxedWireRustGenerator<'a> {
    fn rust_wire_type(&self) -> String {
        todo!()
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for DartOpaqueWireRustGenerator<'a> {
    fn rust_wire_type(&self) -> String {
        todo!()
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for DelegateWireRustGenerator<'a> {
    fn rust_wire_type(&self) -> String {
        todo!()
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for DynamicWireRustGenerator<'a> {
    fn rust_wire_type(&self) -> String {
        todo!()
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for EnumRefWireRustGenerator<'a> {
    fn rust_wire_type(&self) -> String {
        todo!()
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for GeneralListWireRustGenerator<'a> {
    fn rust_wire_type(&self) -> String {
        todo!()
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for OptionalWireRustGenerator<'a> {
    fn rust_wire_type(&self) -> String {
        todo!()
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for OptionalListWireRustGenerator<'a> {
    fn rust_wire_type(&self) -> String {
        todo!()
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for PrimitiveWireRustGenerator<'a> {
    fn rust_wire_type(&self) -> String {
        todo!()
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for PrimitiveListWireRustGenerator<'a> {
    fn rust_wire_type(&self) -> String {
        todo!()
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for RecordWireRustGenerator<'a> {
    fn rust_wire_type(&self) -> String {
        todo!()
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for RustOpaqueWireRustGenerator<'a> {
    fn rust_wire_type(&self) -> String {
        todo!()
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for StructRefWireRustGenerator<'a> {
    fn rust_wire_type(&self) -> String {
        todo!()
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for UnencodableWireRustGenerator<'a> {
    fn rust_wire_type(&self) -> String {
        todo!()
    }
}
