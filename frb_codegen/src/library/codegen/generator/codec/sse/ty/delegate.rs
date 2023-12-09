use crate::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::codegen::generator::codec::sse::lang::*;
use crate::codegen::generator::codec::sse::ty::*;
use crate::codegen::ir::ty::delegate::IrTypeDelegatePrimitiveEnum;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use itertools::Itertools;

impl<'a> CodecSseTyTrait for DelegateCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> Option<String> {
        let inner_expr = match lang {
            Lang::DartLang(_) => match &self.ir {
                IrTypeDelegate::Array(_) => "self.inner",
                IrTypeDelegate::String => "utf8.encoder.convert(self)",
                IrTypeDelegate::PrimitiveEnum(_) => "self.index",
                IrTypeDelegate::Backtrace | IrTypeDelegate::Anyhow => "NOT_USED",
                IrTypeDelegate::Time(_) | IrTypeDelegate::Uuid => {
                    lang.throw_unimplemented(UNIMPLEMENTED_MESSAGE)
                }
            },
            Lang::RustLang(_) => match &self.ir {
                IrTypeDelegate::Array(_) => "self",
                IrTypeDelegate::String => "self.into_bytes()",
                IrTypeDelegate::PrimitiveEnum(_) => "self as _",
                IrTypeDelegate::Backtrace => "TODO",
                IrTypeDelegate::Anyhow => "TODO",
                IrTypeDelegate::Time(_) | IrTypeDelegate::Uuid => {
                    lang.throw_unimplemented(UNIMPLEMENTED_MESSAGE)
                }
            },
        };
        Some(simple_delegate_encode(
            lang,
            &self.ir.get_delegate(),
            inner_expr,
        ))
    }

    fn generate_decode(&self, lang: &Lang) -> Option<String> {
        let wrapper_expr = match lang {
            Lang::DartLang(_) => match &self.ir {
                IrTypeDelegate::Array(_) => format!(
                    "{}(inner)",
                    ApiDartGenerator::new(self.ir.clone(), self.context.as_api_dart_context())
                        .dart_api_type()
                ),
                IrTypeDelegate::String => "utf8.decoder.convert(inner)".to_owned(),
                IrTypeDelegate::PrimitiveEnum(inner) => {
                    format!(
                        "{}.values[inner]",
                        ApiDartGenerator::new(inner.ir.clone(), self.context.as_api_dart_context())
                            .dart_api_type()
                    )
                }
                IrTypeDelegate::Backtrace => "inner".to_owned(),
                IrTypeDelegate::Time(_) | IrTypeDelegate::Uuid => {
                    lang.throw_unimplemented(UNIMPLEMENTED_MESSAGE)
                }
                IrTypeDelegate::Anyhow => "AnyhowException(inner)".to_owned(),
            },
            Lang::RustLang(_) => match &self.ir {
                IrTypeDelegate::Array(_) => {
                    "flutter_rust_bridge::for_generated::from_vec_to_array(inner)".to_owned()
                }
                IrTypeDelegate::String => "String::from_utf8(inner).unwrap()".to_owned(),
                IrTypeDelegate::PrimitiveEnum(inner) => {
                    rust_decode_primitive_enum(inner, self.context.ir_pack)
                }
                IrTypeDelegate::Backtrace | IrTypeDelegate::Anyhow => "NOT_USED".to_owned(),
                IrTypeDelegate::Time(_) | IrTypeDelegate::Uuid => {
                    lang.throw_unimplemented(UNIMPLEMENTED_MESSAGE)
                }
            },
        };
        Some(simple_delegate_decode(
            lang,
            &self.ir.get_delegate(),
            &wrapper_expr,
        ))
    }
}

pub(super) fn simple_delegate_encode(lang: &Lang, inner_ty: &IrType, inner_expr: &str) -> String {
    format!("{};", lang.call_encode(inner_ty, inner_expr))
}

pub(super) fn simple_delegate_decode(lang: &Lang, inner_ty: &IrType, wrapper_expr: &str) -> String {
    format!(
        "{var_decl} inner = {};
        return {wrapper_expr};",
        lang.call_decode(&inner_ty),
        var_decl = lang.var_decl()
    )
}

pub(crate) fn rust_decode_primitive_enum(
    inner: &IrTypeDelegatePrimitiveEnum,
    ir_pack: &IrPack,
) -> String {
    let enu = inner.ir.get(ir_pack);
    let variants = (enu.variants().iter().enumerate())
        .map(|(idx, variant)| format!("{} => {}::{},", idx, enu.name.rust_style(), variant.name))
        .collect_vec()
        .join("\n");

    format!(
        "match self {{
            {}
            _ => unreachable!(\"Invalid variant for {}: {{}}\", self),
        }}",
        variants, enu.name.name
    )
}

const UNIMPLEMENTED_MESSAGE: &str =
    "not yet supported in serialized mode, feel free to create an issue";
