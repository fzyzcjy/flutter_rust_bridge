use crate::codegen::generator::api_dart::spec_generator::base::ApiDartGenerator;
use crate::codegen::generator::codec::sse::lang::*;
use crate::codegen::generator::codec::sse::ty::*;
use crate::codegen::ir::ty::delegate::{
    IrTypeDelegatePrimitiveEnum, IrTypeDelegateSet, IrTypeDelegateStreamSink, IrTypeDelegateTime,
};
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use convert_case::{Case, Casing};
use itertools::Itertools;

impl<'a> CodecSseTyTrait for DelegateCodecSseTy<'a> {
    fn generate_encode(&self, lang: &Lang) -> Option<String> {
        let inner_expr = match lang {
            Lang::DartLang(_) => match &self.ir {
                IrTypeDelegate::Array(_) => "self.inner".to_owned(),
                IrTypeDelegate::String => "utf8.encoder.convert(self)".to_owned(),
                IrTypeDelegate::Char => "self".to_owned(),
                IrTypeDelegate::PrimitiveEnum(_) => "self.index".to_owned(),
                IrTypeDelegate::Backtrace => {
                    return Some(format!("{};", lang.throw_unreachable("")));
                }
                IrTypeDelegate::AnyhowException => "self.message".to_owned(),
                IrTypeDelegate::Map(_) => {
                    "self.entries.map((e) => (e.key, e.value)).toList()".to_owned()
                }
                IrTypeDelegate::Set(ir) => {
                    generate_set_to_list(ir, self.context.as_api_dart_context(), "self")
                }
                IrTypeDelegate::Time(ir) => match ir {
                    IrTypeDelegateTime::Utc
                    | IrTypeDelegateTime::Local
                    | IrTypeDelegateTime::Naive => {
                        "PlatformInt64Util.from(self.microsecondsSinceEpoch)".to_owned()
                    }
                    IrTypeDelegateTime::Duration => {
                        "PlatformInt64Util.from(self.inMicroseconds)".to_owned()
                    }
                },
                IrTypeDelegate::Uuid => "self.toBytes()".to_owned(),
                IrTypeDelegate::StreamSink(ir) => {
                    generate_stream_sink_setup_and_serialize(ir, "self")
                }
                IrTypeDelegate::BigPrimitive(_) => "self.toString()".to_owned(),
                IrTypeDelegate::RustAutoOpaqueExplicit(ir) => "self",
            },
            Lang::RustLang(_) => match &self.ir {
                IrTypeDelegate::Array(_) => {
                    "{ let boxed: Box<[_]> = Box::new(self); boxed.into_vec() }".to_owned()
                }
                IrTypeDelegate::String => "self.into_bytes()".to_owned(),
                IrTypeDelegate::Char => "self.to_string()".to_owned(),
                IrTypeDelegate::PrimitiveEnum(ir) => {
                    let src = ir.ir.get(self.context.ir_pack);
                    let variants = (src.variants.iter().enumerate())
                        .map(|(idx, variant)| {
                            (
                                format!("{}::{}", src.name.rust_style(), variant.name),
                                format!("{idx}"),
                            )
                        })
                        .collect_vec();
                    lang.switch_expr(
                        "self",
                        &variants,
                        Some(format!("{};", lang.throw_unimplemented(""))),
                    )
                }
                IrTypeDelegate::Backtrace => r#"format!("{:?}", self)"#.to_owned(),
                IrTypeDelegate::AnyhowException => r#"format!("{:?}", self)"#.to_owned(),
                IrTypeDelegate::Map(_) => "self.into_iter().collect()".to_owned(),
                IrTypeDelegate::Set(_) => "self.into_iter().collect()".to_owned(),
                IrTypeDelegate::Time(ir) => match ir {
                    IrTypeDelegateTime::Utc | IrTypeDelegateTime::Local => {
                        "self.timestamp_micros()".to_owned()
                    }
                    IrTypeDelegateTime::Naive => "self.and_utc().timestamp_micros()".to_owned(),
                    IrTypeDelegateTime::Duration => {
                        r#"self.num_microseconds().expect("cannot get microseconds from time")"#
                            .to_owned()
                    }
                },
                IrTypeDelegate::Uuid => "self.as_bytes().to_vec()".to_owned(),
                IrTypeDelegate::StreamSink(_) => return Some(lang.throw_unimplemented("")),
                IrTypeDelegate::BigPrimitive(_) => "self.to_string()".to_owned(),
                IrTypeDelegate::RustAutoOpaqueExplicit(ir) => "rust_auto_opaque_explicit_encode(inner)",
            },
        };
        Some(simple_delegate_encode(
            lang,
            &self.ir.get_delegate(),
            &inner_expr,
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
                IrTypeDelegate::Char => "inner".to_owned(),
                IrTypeDelegate::PrimitiveEnum(inner) => {
                    format!(
                        "{}.values[inner]",
                        ApiDartGenerator::new(inner.ir.clone(), self.context.as_api_dart_context())
                            .dart_api_type()
                    )
                }
                IrTypeDelegate::Backtrace => "inner".to_owned(),
                IrTypeDelegate::AnyhowException => "AnyhowException(inner)".to_owned(),
                IrTypeDelegate::Map(_) => {
                    "Map.fromEntries(inner.map((e) => MapEntry(e.$1, e.$2)))".to_owned()
                }
                IrTypeDelegate::Set(_) => "Set.from(inner)".to_owned(),
                IrTypeDelegate::Time(ir) => match ir {
                    IrTypeDelegateTime::Utc
                    | IrTypeDelegateTime::Local
                    | IrTypeDelegateTime::Naive => {
                        format!(
                            "DateTime.fromMicrosecondsSinceEpoch(inner.toInt(), isUtc: {is_utc})",
                            is_utc =
                                matches!(ir, IrTypeDelegateTime::Naive | IrTypeDelegateTime::Utc),
                        )
                    }
                    IrTypeDelegateTime::Duration => {
                        "Duration(microseconds: inner.toInt())".to_owned()
                    }
                },
                IrTypeDelegate::Uuid => "UuidValue.fromByteList(inner)".to_owned(),
                IrTypeDelegate::StreamSink(_) => {
                    return Some(format!("{};", lang.throw_unreachable("")));
                }
                IrTypeDelegate::BigPrimitive(_) => "BigInt.parse(inner)".to_owned(),
                IrTypeDelegate::RustAutoOpaqueExplicit(ir) => "inner",
            },
            Lang::RustLang(_) => match &self.ir {
                IrTypeDelegate::Array(_) => {
                    "flutter_rust_bridge::for_generated::from_vec_to_array(inner)".to_owned()
                }
                IrTypeDelegate::String => "String::from_utf8(inner).unwrap()".to_owned(),
                IrTypeDelegate::Char => "inner.chars().next().unwrap()".to_owned(),
                IrTypeDelegate::PrimitiveEnum(inner) => {
                    rust_decode_primitive_enum(inner, self.context.ir_pack, "inner")
                }
                IrTypeDelegate::Backtrace => {
                    return Some(format!("{};", lang.throw_unreachable("")));
                }
                IrTypeDelegate::AnyhowException => {
                    r#"flutter_rust_bridge::for_generated::anyhow::anyhow!("{}", inner)"#.to_owned()
                }
                IrTypeDelegate::Map(_) => "inner.into_iter().collect()".to_owned(),
                IrTypeDelegate::Set(_) => "inner.into_iter().collect()".to_owned(),
                IrTypeDelegate::Time(ir) => {
                    let naive = "chrono::DateTime::from_timestamp_micros(inner).expect(\"invalid or out-of-range datetime\").naive_utc()";
                    let utc = format!("chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset({naive}, chrono::Utc)");
                    match ir {
                        IrTypeDelegateTime::Naive => naive.to_owned(),
                        IrTypeDelegateTime::Utc => utc,
                        IrTypeDelegateTime::Local => {
                            format!("chrono::DateTime::<chrono::Local>::from({utc})")
                        }
                        IrTypeDelegateTime::Duration => {
                            "chrono::Duration::microseconds(inner)".to_owned()
                        }
                    }
                }
                IrTypeDelegate::Uuid => {
                    r#"uuid::Uuid::from_slice(&inner).expect("fail to decode uuid")"#.to_owned()
                }
                IrTypeDelegate::StreamSink(_) => "StreamSink::deserialize(inner)".to_owned(),
                IrTypeDelegate::BigPrimitive(_) => "inner.parse().unwrap()".to_owned(),
                IrTypeDelegate::RustAutoOpaqueExplicit(ir) => "rust_auto_opaque_explicit_decode(inner)",
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
        lang.call_decode(inner_ty),
        var_decl = lang.var_decl()
    )
}

pub(crate) fn rust_decode_primitive_enum(
    inner: &IrTypeDelegatePrimitiveEnum,
    ir_pack: &IrPack,
    var_name: &str,
) -> String {
    let enu = inner.ir.get(ir_pack);
    let variants = (enu.variants().iter().enumerate())
        .map(|(idx, variant)| format!("{} => {}::{},", idx, enu.name.rust_style(), variant.name))
        .collect_vec()
        .join("\n");

    format!(
        "match {var_name} {{
            {}
            _ => unreachable!(\"Invalid variant for {}: {{}}\", {var_name}),
        }}",
        variants, enu.name.name
    )
}

pub(crate) fn generate_unimplemented_in_sse_message(ir: &IrType) -> String {
    format!("The type {ir:?} is not yet supported in serialized mode, please use full_dep mode, and feel free to create an issue")
}

pub(crate) fn generate_set_to_list(
    ir: &IrTypeDelegateSet,
    context: ApiDartGeneratorContext,
    inner: &str,
) -> String {
    let mut ans = format!("{inner}.toList()");
    if let Primitive(_) = &*ir.inner {
        ans = format!(
            "{}.fromList({ans})",
            ApiDartGenerator::new(
                IrTypeDelegate::Set(ir.to_owned()).get_delegate().clone(),
                context
            )
            .dart_api_type()
        );
    }
    ans
}

pub(crate) fn generate_stream_sink_setup_and_serialize(
    ir: &IrTypeDelegateStreamSink,
    var_name: &str,
) -> String {
    let codec = ir.codec;
    let codec_lower = codec.to_string().to_case(Case::Snake);
    let inner_ty = ir.inner.safe_ident();

    let codec_code = format!(
        "{codec}Codec(decodeSuccessData: {codec_lower}_decode_{inner_ty}, decodeErrorData: null)"
    );

    format!("{var_name}.setupAndSerialize(codec: {codec_code})")
}
