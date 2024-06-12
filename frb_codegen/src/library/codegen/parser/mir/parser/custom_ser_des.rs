use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::mir::custom_ser_des::{MirCustomSerDes, MirCustomSerDesHalf};
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::parser::attribute::{FrbAttributeSerDes, FrbAttributes};
use crate::codegen::parser::mir::parser::function;
use crate::codegen::parser::mir::parser::ty::TypeParser;
use itertools::Itertools;

pub(crate) fn parse(
    src_fns: &[HirFlatFunction],
    type_parser: &mut TypeParser,
) -> anyhow::Result<Vec<MirCustomSerDes>> {
    let infos = (src_fns.iter())
        .map(|f| parse_function(f))
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .flatten();

    let ans = infos
        .into_group_map_by(|info| *info.rust_api_type.clone())
        .into_iter()
        .map(|(_, pair)| MirCustomSerDes {
            inner_type: TODO,
            rust_api_type: TODO,
            dart_api_type: TODO,
            dart2rust: TODO,
            rust2dart: TODO,
        })
        .collect_vec();

    Ok(ans)
}

fn parse_function(func: &HirFlatFunction) -> anyhow::Result<Option<Info>> {
    let attrs = FrbAttributes::parse(func.item_fn.attrs())?;

    if let Some(info) = attrs.dart2rust() {
        return Ok(Some(parse_function_inner(
            func,
            info,
            Direction::Dart2Rust,
        )?));
    }
    if let Some(info) = attrs.rust2dart() {
        return Ok(Some(parse_function_inner(
            func,
            info,
            Direction::Rust2Dart,
        )?));
    }

    Ok(None)
}

fn parse_function_inner(
    function: &HirFlatFunction,
    attr_ser_des: FrbAttributeSerDes,
    direction: Direction,
) -> anyhow::Result<Info> {
    Ok(Info {
        inner_type: TODO,
        rust_api_type: TODO,
        dart_api_type: attr_ser_des.dart_type,
        direction,
        half: MirCustomSerDesHalf {
            dart_code: attr_ser_des.dart_code,
            rust_function: TODO,
        },
    })
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
