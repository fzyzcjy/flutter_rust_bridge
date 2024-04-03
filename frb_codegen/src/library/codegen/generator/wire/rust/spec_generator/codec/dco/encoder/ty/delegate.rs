use crate::codegen::generator::wire::rust::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::encoder::misc::{
    generate_impl_into_dart, generate_impl_into_into_dart,
};
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::encoder::ty::enumeration::{
    generate_enum_access_object_core, parse_wrapper_name_into_dart_name_and_self_path,
};
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::encoder::ty::WireRustCodecDcoGeneratorEncoderTrait;
use crate::codegen::ir::ty::delegate::{
    IrTypeDelegate, IrTypeDelegateArray, IrTypeDelegateMap, IrTypeDelegatePrimitiveEnum,
    IrTypeDelegateSet,
};
use crate::codegen::ir::ty::{IrType, IrTypeTrait};
use itertools::Itertools;

impl<'a> WireRustCodecDcoGeneratorEncoderTrait for DelegateWireRustCodecDcoGenerator<'a> {
    fn intodart_type(&self, ir_pack: &crate::codegen::ir::pack::IrPack) -> String {
        match &self.ir {
            IrTypeDelegate::PrimitiveEnum(IrTypeDelegatePrimitiveEnum { ir, .. }) => {
                let inner = WireRustCodecDcoGenerator::new(IrType::from(ir.clone()), self.context);
                inner.intodart_type(ir_pack)
            }
            IrTypeDelegate::Set(IrTypeDelegateSet { inner }) => {
                let inner =
                    WireRustCodecDcoGenerator::new(IrType::from(inner.clone()), self.context);
                format!(
                    "std::collections::HashSet<{}>",
                    inner.intodart_type(ir_pack)
                )
            }
            IrTypeDelegate::Array(IrTypeDelegateArray { mode, length, .. }) => {
                let inner = match mode {
                    crate::codegen::ir::ty::delegate::IrTypeDelegateArrayMode::General(inner) => {
                        WireRustCodecDcoGenerator::new(IrType::from(inner.clone()), self.context)
                    }
                    crate::codegen::ir::ty::delegate::IrTypeDelegateArrayMode::Primitive(inner) => {
                        WireRustCodecDcoGenerator::new(IrType::from(inner.clone()), self.context)
                    }
                };
                format!("[{}; {}]", inner.intodart_type(ir_pack), length)
            }
            IrTypeDelegate::Map(IrTypeDelegateMap { key, value, .. }) => {
                let key_inner =
                    WireRustCodecDcoGenerator::new(IrType::from(key.clone()), self.context);
                let value_inner =
                    WireRustCodecDcoGenerator::new(IrType::from(value.clone()), self.context);
                format!(
                    "std::collections::HashMap<{}, {}>",
                    key_inner.intodart_type(ir_pack),
                    value_inner.intodart_type(ir_pack)
                )
            }
            // default trait implementation
            _ => self.ir.rust_api_type(),
        }
    }

    // the function signature is not covered while the whole body is covered - looks like a bug in coverage tool
    // frb-coverage:ignore-start
    fn generate_impl_into_dart(&self) -> Option<String> {
        // frb-coverage:ignore-end
        if let IrTypeDelegate::PrimitiveEnum(IrTypeDelegatePrimitiveEnum { ir, .. }) = &self.ir {
            let src = ir.get(self.context.ir_pack);
            let (name, self_path) =
                parse_wrapper_name_into_dart_name_and_self_path(&src.name, &src.wrapper_name);

            let self_ref = generate_enum_access_object_core(ir, "self".to_owned(), self.context);
            let variants = src
                .variants()
                .iter()
                .enumerate()
                .map(|(idx, variant)| {
                    format!("{}::{} => {}.into_dart(),", self_path, variant.name, idx)
                })
                .collect_vec()
                .join("\n");

            let body = format!(
                "match {self_ref} {{
                    {variants}
                }}"
            );

            return Some(
                generate_impl_into_dart(&name, &body)
                    + &generate_impl_into_into_dart(&src.name.rust_style(), &src.wrapper_name),
            );
        }
        None
    }
}
