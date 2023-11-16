use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::codegen::ir::ty::unencodable::IrTypeUnencodable;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::{EnumRef, StructRef};
use crate::codegen::parser::function_parser::FunctionParser;
use crate::codegen::parser::type_parser::unencodable::{splay_segments, ArgsRefs};
use syn::*;

/// Represents a function's output type
#[derive(Debug, Clone)]
pub(super) enum FuncOutput {
    ResultType { ok: IrType, error: Option<IrType> },
    Type(IrType),
}

impl<'a, 'b> FunctionParser<'a, 'b> {
    /// Attempts to parse the type from the return part of a function signature. There is a special
    /// case for top-level `Result` types.
    pub(super) fn parse_fn_output_type(&mut self, ty: &Type) -> anyhow::Result<Option<FuncOutput>> {
        let ty = &self.type_parser.resolve_alias(ty).clone();

        Ok(if let Type::Path(type_path) = ty {
            match self.type_parser.parse_type_path(&type_path) {
                Ok(IrType::Unencodable(IrTypeUnencodable { segments, .. })) => {
                    match splay_segments(&segments).last() {
                        Some(("Result", Some(ArgsRefs::Generic(args)))) => {
                            parse_fn_output_type_result(args)
                        }
                        _ => None, // unencodable types not implemented
                    }
                }
                Ok(result) => Some(FuncOutput::Type(result)),
                Err(..) => None,
            }
        } else {
            let ir_ty = self.type_parser.parse_type(ty)?;
            Some(FuncOutput::Type(ir_ty))
        })
    }
}

fn parse_fn_output_type_result(args: &[IrType]) -> Option<FuncOutput> {
    let ok = args.first().unwrap();

    let is_anyhow = args.len() == 1
        || args.iter().any(|x| match x {
            IrType::Unencodable(IrTypeUnencodable { string, .. }) => string == "anyhow :: Error",
            _ => false,
        });
    let error = if is_anyhow {
        Some(IrType::Delegate(IrTypeDelegate::Anyhow))
    } else {
        args.last().cloned()
    };

    let error = if let Some(StructRef(mut struct_ref)) = error {
        struct_ref.is_exception = true;
        Some(StructRef(struct_ref))
    } else if let Some(EnumRef(mut enum_ref)) = error {
        enum_ref.is_exception = true;
        Some(EnumRef(enum_ref))
    } else {
        error
    };

    Some(FuncOutput::ResultType {
        ok: ok.clone(),
        error,
    })
}
