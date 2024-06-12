use crate::codegen::ir::hir::flat::function::HirFlatFunction;
use crate::codegen::ir::mir::custom_ser_des::MirCustomSerDes;
use crate::codegen::parser::mir::parser::attribute::FrbAttributes;
use crate::codegen::parser::mir::parser::function;
use crate::codegen::parser::mir::parser::ty::TypeParser;
use itertools::Itertools;

pub(crate) fn parse(
    src_fns: &[HirFlatFunction],
    type_parser: &mut TypeParser,
) -> anyhow::Result<Vec<MirCustomSerDes>> {
    todo!();
    // let interest_src_fns = (src_fns.iter())
    //     .filter_map(|f| {
    //         let attrs = match FrbAttributes::parse(f.item_fn.attrs()) {
    //             Ok(value) => value,
    //             Err(err) => return Some(Err(err)),
    //         };
    //         if attrs.custom_ser_des().is_some() {
    //             Some(Ok(f))
    //         } else {
    //             None
    //         }
    //     })
    //     .collect::<anyhow::Result<Vec<_>>>()?;
    //
    // function::real::parse(&interest_src_fns)?;
}

// fn parse_one() -> anyhow::Result<Option<MirCustomSerDes>> {
//     TODO;
// }
