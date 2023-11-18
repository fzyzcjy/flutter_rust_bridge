use crate::codegen::generator::misc::{is_js_value, Target};
use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::IrTypeTrait;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub(crate) trait WireRustGeneratorInfoTrait {
    fn rust_wire_type(&self, target: Target) -> String;

    fn rust_wire_modifier(&self, target: Target) -> String {
        if self.rust_wire_is_pointer(target) {
            "*mut ".to_string()
        } else {
            "".to_string()
        }
    }

    fn rust_wire_is_pointer(&self, _target: Target) -> bool {
        false
    }
}

const JS_VALUE: &str = "JsValue";

impl<'a> WireRustGeneratorInfoTrait for BoxedWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        if target == Target::Wasm && self.ir.inner.is_primitive() {
            JS_VALUE.into()
        } else {
            WireRustGenerator::new(self.ir.inner.clone(), self.context).rust_wire_type(target)
        }
    }

    fn rust_wire_is_pointer(&self, target: Target) -> bool {
        (target != Target::Wasm)
            || !is_js_value(&self.ir.inner)
                && !self.ir.inner.is_array()
                && !self.ir.inner.is_primitive()
    }
}

impl<'a> WireRustGeneratorInfoTrait for DartOpaqueWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        rust_wire_type_add_prefix_or_js_value(&self.ir, target)
    }
}

impl<'a> WireRustGeneratorInfoTrait for DelegateWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        match (&self.ir, target) {
            (IrTypeDelegate::String, Target::Wasm) => "String".into(),
            (IrTypeDelegate::StringList, Target::Io) => "wire_StringList".to_owned(),
            (IrTypeDelegate::StringList, Target::Wasm) => JS_VALUE.into(),
            _ => WireRustGenerator::new(self.ir.get_delegate().clone(), self.context)
                .rust_wire_type(target),
        }
    }

    fn rust_wire_is_pointer(&self, target: Target) -> bool {
        WireRustGenerator::new(self.ir.get_delegate(), self.context).rust_wire_is_pointer(target)
    }
}

impl<'a> WireRustGeneratorInfoTrait for DynamicWireRustGenerator<'a> {
    fn rust_wire_type(&self, _target: Target) -> String {
        unreachable!("Functions cannot receive dynamic parameters.")
    }
}

impl<'a> WireRustGeneratorInfoTrait for EnumRefWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        rust_wire_type_add_prefix_or_js_value(&self.ir, target)
    }
}

impl<'a> WireRustGeneratorInfoTrait for GeneralListWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        rust_wire_type_add_prefix_or_js_value(&self.ir, target)
    }

    fn rust_wire_is_pointer(&self, target: Target) -> bool {
        target != Target::Wasm
    }
}

impl<'a> WireRustGeneratorInfoTrait for OptionalWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        let inner_generator = WireRustGenerator::new(self.ir.inner.clone(), self.context);

        if inner_generator.rust_wire_is_pointer(target)
            || (target == Target::Wasm)
                && (is_js_value(&self.ir.inner)
                    || self.ir.is_primitive()
                    || self.ir.is_boxed_primitive())
        {
            inner_generator.rust_wire_type(target)
        } else {
            format!("Option<{}>", inner_generator.rust_wire_type(target))
        }
    }

    fn rust_wire_is_pointer(&self, target: Target) -> bool {
        target != Target::Wasm
            || WireRustGenerator::new(self.ir.inner.clone(), self.context)
                .rust_wire_is_pointer(target)
    }
}

impl<'a> WireRustGeneratorInfoTrait for OptionalListWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        rust_wire_type_add_prefix_or_js_value(&self.ir, target)
    }

    fn rust_wire_is_pointer(&self, target: Target) -> bool {
        target != Target::Wasm
    }
}

impl<'a> WireRustGeneratorInfoTrait for PrimitiveWireRustGenerator<'a> {
    fn rust_wire_type(&self, _target: Target) -> String {
        self.ir.rust_api_type()
    }
}

impl<'a> WireRustGeneratorInfoTrait for PrimitiveListWireRustGenerator<'a> {
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

    fn rust_wire_is_pointer(&self, target: Target) -> bool {
        target != Target::Wasm
    }
}

impl<'a> WireRustGeneratorInfoTrait for RecordWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        rust_wire_type_add_prefix_or_js_value(&self.ir, target)
    }
}

impl RecordWireRustGenerator<'_> {
    pub(crate) fn as_struct_generator(&self) -> StructRefWireRustGenerator {
        StructRefWireRustGenerator {
            ir: self.ir.inner.clone(),
            context: self.context,
        }
    }
}

impl<'a> WireRustGeneratorInfoTrait for RustOpaqueWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        rust_wire_type_add_prefix_or_js_value(&self.ir, target)
    }
}

impl<'a> WireRustGeneratorInfoTrait for StructRefWireRustGenerator<'a> {
    fn rust_wire_type(&self, target: Target) -> String {
        rust_wire_type_add_prefix_or_js_value(&self.ir, target)
    }
}

impl<'a> WireRustGeneratorInfoTrait for UnencodableWireRustGenerator<'a> {
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
