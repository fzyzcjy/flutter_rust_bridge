use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::{Target, TargetOrCommon};
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::decoder::impl_new_with_nullptr::generate_impl_new_with_nullptr_code_block;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::decoder::misc::rust_wire_type_add_prefix_or_js_value;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::decoder::ty::WireRustGeneratorDart2RustTrait;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::ExternFunc;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::base::*;
use crate::codegen::generator::wire::rust::spec_generator::transfer::cst::decoder::ty::WireRustTransferCstGeneratorDecoderTrait;
use crate::codegen::ir::field::IrField;
use crate::codegen::ir::ty::enumeration::{IrEnum, IrEnumMode, IrVariant, IrVariantKind};
use crate::codegen::ir::ty::IrType;
use itertools::Itertools;

impl<'a> WireRustTransferCstGeneratorDecoderTrait for EnumRefWireRustTransferCstGenerator<'a> {
    fn generate_wire2api_class(&self) -> Option<String> {
        let src = self.ir.get(self.context.ir_pack);
        if src.mode == IrEnumMode::Simple {
            return None;
        }

        let variants = src.variants();

        let variant_structs = variants
            .iter()
            .map(|variant| self.generate_wire2api_class_variant(variant))
            .join("\n\n");

        let union_fields = variants
            .iter()
            .map(|variant| {
                format!(
                    "{0}: *mut wire_{1}_{0},",
                    variant.name, self.ir.ident.0.name
                )
            })
            .join("\n");

        let rust_wire_type = self.rust_wire_type(Target::Io);

        Some(format!(
            "#[repr(C)]
            #[derive(Clone)]
            pub struct {rust_wire_type} {{ tag: i32, kind: *mut {name}Kind }}

            #[repr(C)]
            pub union {name}Kind {{
                {union_fields}
            }}

            {variant_structs}",
            name = self.ir.ident.0.name,
        ))
    }

    fn generate_impl_wire2api_body(&self) -> Acc<Option<String>> {
        let enu = self.ir.get(self.context.ir_pack);
        Acc::new(|target| {
            if matches!(target, TargetOrCommon::Common) {
                return None;
            }

            let wasm = target == TargetOrCommon::Wasm;
            let variants = enu
                .variants()
                .iter()
                .enumerate()
                .map(|(idx, variant)| {
                    generate_impl_wire2api_body_variant(enu, target, idx, variant)
                })
                .join("\n");

            Some(format!(
                "{}
                match self{} {{
                    {}
                    _ => unreachable!(),
                }}",
                if wasm {
                    "let self_ = self.unchecked_into::<flutter_rust_bridge::for_generated::js_sys::Array>();"
                } else {
                    ""
                },
                if wasm {
                    "_.get(0).unchecked_into_f64() as _"
                } else {
                    ".tag"
                },
                variants,
            ))
        })
    }

    fn generate_impl_new_with_nullptr(&self) -> Option<WireRustOutputCode> {
        let src = self.ir.get(self.context.ir_pack);

        let inflators = src
            .variants()
            .iter()
            .filter_map(|variant| self.generate_impl_new_with_nullptr_variant(variant))
            .collect_vec();

        Some(WireRustOutputCode {
            body: generate_impl_new_with_nullptr_code_block(
                self.ir.clone(),
                self.context,
                "Self { tag: -1, kind: core::ptr::null_mut() }",
                true,
            ),
            extern_funcs: inflators,
        })
    }

    fn rust_wire_type(&self, target: Target) -> String {
        rust_wire_type_add_prefix_or_js_value(&self.ir, target)
    }
}
