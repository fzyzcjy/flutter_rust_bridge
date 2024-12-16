use crate::codegen::ir::mir::field::MirField;
use crate::codegen::ir::mir::func::MirFuncInput;
use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
use crate::codegen::ir::mir::ty::primitive_list::MirTypePrimitiveList;
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::parser::mir::parser::function::real::{FunctionParser, FunctionPartialInfo};
use itertools::Itertools;
use MirTypePrimitive::{Isize, Usize, I64, U64};

impl<'a, 'b> FunctionParser<'a, 'b> {
    pub(super) fn transform_fn_info(&mut self, info: FunctionPartialInfo) -> FunctionPartialInfo {
        FunctionPartialInfo {
            inputs: (info.inputs.into_iter())
                .map(|x| MirFuncInput {
                    inner: MirField {
                        ty: transform_primitive_list_param(x.inner.ty),
                        ..x.inner
                    },
                    ..x
                })
                .collect_vec(),
            ..info
        }
    }
}

fn transform_primitive_list_param(ty: MirType) -> MirType {
    if let MirType::PrimitiveList(inner) = &ty {
        match inner.primitive {
            U64 | I64 | Usize | Isize => ty,
            _ => MirType::PrimitiveList(MirTypePrimitiveList {
                strict_dart_type: false,
                ..inner.clone()
            }),
        }
    } else {
        ty
    }
}
