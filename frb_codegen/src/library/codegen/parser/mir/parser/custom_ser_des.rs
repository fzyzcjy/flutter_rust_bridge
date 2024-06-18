use crate::codegen::generator::codec::structs::CodecMode;
use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::mir::custom_ser_des::{MirCustomSerDes, MirCustomSerDesHalf};
use crate::codegen::ir::mir::ty::rust_opaque::RustOpaqueCodecMode;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::parser::attribute::{FrbAttributeSerDes, FrbAttributes};
use crate::codegen::parser::mir::parser::ty::{TypeParser, TypeParserParsingContext};
use crate::codegen::parser::mir::ParseMode;
use crate::if_then_some;
use crate::utils::namespace::{Namespace, NamespacedName};
use anyhow::ensure;
use itertools::Itertools;
use syn::{FnArg, ReturnType};

pub(crate) struct PartialContext {
    pub rust_output_path_namespace: Namespace,
    pub default_stream_sink_codec: CodecMode,
    pub default_rust_opaque_codec: RustOpaqueCodecMode,
    pub enable_lifetime: bool,
    pub type_64bit_int: bool,
    pub parse_mode: ParseMode,
}

pub(crate) fn parse(
    src_fns: &[HirFlatFunction],
    type_parser: &mut TypeParser,
    partial_context: &PartialContext,
) -> anyhow::Result<Vec<MirCustomSerDes>> {
    let infos = (src_fns.iter())
        .map(|f| parse_function(f, type_parser, partial_context))
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .flatten();

    let ans = infos
        .into_group_map_by(|info| *info.rust_api_type.clone())
        .into_values()
        .map(merge_pair)
        .collect_vec();

    Ok(ans)
}

fn parse_function(
    func: &HirFlatFunction,
    type_parser: &mut TypeParser,
    partial_context: &PartialContext,
) -> anyhow::Result<Option<Info>> {
    let attrs = FrbAttributes::parse(func.item_fn.attrs())?;

    if let Some(info) = attrs.dart2rust() {
        return Ok(Some(parse_function_inner(
            func,
            info,
            Direction::Dart2Rust,
            type_parser,
            partial_context,
        )?));
    }
    if let Some(info) = attrs.rust2dart() {
        return Ok(Some(parse_function_inner(
            func,
            info,
            Direction::Rust2Dart,
            type_parser,
            partial_context,
        )?));
    }

    Ok(None)
}

fn parse_function_inner(
    func: &HirFlatFunction,
    attr_ser_des: FrbAttributeSerDes,
    direction: Direction,
    type_parser: &mut TypeParser,
    partial_context: &PartialContext,
) -> anyhow::Result<Info> {
    let sig = func.item_fn.sig();

    ensure!(sig.inputs.len() == 1);
    let input_ty = if_then_some!(let FnArg::Typed(pat_type) = sig.inputs.first().unwrap().clone(), *pat_type.ty).unwrap();
    let output_ty = if_then_some!(let ReturnType::Type(_, ty) = sig.output.clone(), *ty).unwrap();

    let context = TypeParserParsingContext {
        initiated_namespace: func.namespace.clone(),
        func_attributes: FrbAttributes::parse(&[])?,
        struct_or_enum_attributes: None,
        owner: None,
        rust_output_path_namespace: partial_context.rust_output_path_namespace.clone(),
        default_stream_sink_codec: partial_context.default_stream_sink_codec,
        default_rust_opaque_codec: partial_context.default_rust_opaque_codec,
        enable_lifetime: partial_context.enable_lifetime,
        type_64bit_int: partial_context.type_64bit_int,
        forbid_type_self: false,
        parse_mode: partial_context.parse_mode,
    };

    let input_ty = Box::new(type_parser.parse_type(&input_ty, &context)?);
    let output_ty = Box::new(type_parser.parse_type(&output_ty, &context)?);

    let (rust_api_type, inner_type) = match direction {
        Direction::Rust2Dart => (input_ty, output_ty),
        Direction::Dart2Rust => (output_ty, input_ty),
    };

    Ok(Info {
        inner_type,
        rust_api_type,
        dart_api_type: attr_ser_des.dart_type,
        direction,
        half: MirCustomSerDesHalf {
            dart_code: attr_ser_des.dart_code,
            rust_function: NamespacedName::new(func.namespace.clone(), func.item_fn.name()),
        },
    })
}

fn merge_pair(pair: Vec<Info>) -> MirCustomSerDes {
    let [a, b]: [Info; 2] = (pair.try_into())
        .unwrap_or_else(|_| panic!("Expect a pair of serializer and deserializer"));
    let (dart2rust, rust2dart) = if a.direction == Direction::Dart2Rust {
        (a, b)
    } else {
        (b, a)
    };

    MirCustomSerDes {
        inner_type: dart2rust.inner_type.clone(),
        rust_api_type: dart2rust.rust_api_type.clone(),
        dart_api_type: dart2rust.dart_api_type.clone(),
        dart2rust: dart2rust.half,
        rust2dart: rust2dart.half,
    }
}

struct Info {
    inner_type: Box<MirType>,
    rust_api_type: Box<MirType>,
    dart_api_type: String,
    direction: Direction,
    half: MirCustomSerDesHalf,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Rust2Dart,
    Dart2Rust,
}
