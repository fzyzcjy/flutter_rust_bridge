use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::IrTypeTrait;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub(crate) trait WireRustGeneratorRustWireTypeTrait {
    fn rust_wire_type(&self, target: Target) -> String;
}

const JS_VALUE: &str = "JsValue";

impl<'a> WireRustGeneratorRustWireTypeTrait for BoxedWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        if target == Target::Wasm && self.ir.inner.is_primitive() {
            JS_VALUE.into()
        } else {
            WireRustGenerator::new(*self.ir.inner.clone(), self.context.clone())
                .rust_wire_type(target)
        }
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for DartOpaqueWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        rust_wire_type_add_prefix_or_js_value(&self.ir, target)
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for DelegateWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        match (&self.ir, target) {
            (IrTypeDelegate::String, Target::Wasm) => "String".into(),
            (IrTypeDelegate::StringList, Target::Io) => "wire_StringList".to_owned(),
            (IrTypeDelegate::StringList, Target::Wasm) => JS_VALUE.into(),
            _ => self.ir.get_delegate().rust_wire_type(target),
        }
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for DynamicWireRustGenerator<'a> {
    fn rust_wire_type(&self, _target: Target) -> String {
        unreachable!("Functions cannot receive dynamic parameters.")
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for EnumRefWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        rust_wire_type_add_prefix_or_js_value(&self.ir, target)
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for GeneralListWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        rust_wire_type_add_prefix_or_js_value(&self.ir, target)
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for OptionalWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        let inner_rust_wire_type =
            WireRustGenerator::new(*self.ir.inner.clone(), self.context.clone())
                .rust_wire_type(target);

        if self.ir.inner.rust_wire_is_pointer(target)
            || (target == Target::Wasm)
                && (self.ir.inner.is_js_value() || self.is_primitive() || self.is_boxed_primitive())
        {
            inner_rust_wire_type
        } else {
            format!("Option<{}>", inner_rust_wire_type)
        }
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for OptionalListWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        rust_wire_type_add_prefix_or_js_value(&self.ir, target)
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for PrimitiveWireRustGenerator<'a> {
    fn rust_wire_type(&self, _target: Target) -> String {
        self.ir.rust_api_type()
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for PrimitiveListWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        if let Target::Wasm = target {
            match self.ir.primitive {
                IrTypePrimitive::Bool | IrTypePrimitive::Unit => JS_VALUE.into(),
                _ => format!("Box<[{}]>", self.ir.primitive.rust_api_type()),
            }
        } else {
            format!("wire_{}", self.ir.safe_ident())
        }
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for RecordWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        rust_wire_type_add_prefix_or_js_value(&self.ir, target)
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for RustOpaqueWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        rust_wire_type_add_prefix_or_js_value(&self.ir, target)
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for StructRefWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        rust_wire_type_add_prefix_or_js_value(&self.ir, target)
    }
}

impl<'a> WireRustGeneratorRustWireTypeTrait for UnencodableWireRustGenerator<'a> {
    fn rust_wire_type(&self, _target: Target) -> String {
        unreachable!()
    }
}

fn rust_wire_type_add_prefix_or_js_value<T: IrTypeTrait>(ir: &T, target: Target) -> String {
    if let Target::Wasm = target {
        JS_VALUE.into()
    } else {
        format!("wire_{}", ir.safe_ident())
    }
}
