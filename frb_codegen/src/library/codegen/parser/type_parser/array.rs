use crate::codegen::ir::namespace::Namespace;
use crate::codegen::ir::ty::delegate::{
    IrTypeDelegate, IrTypeDelegateArray, IrTypeDelegateArrayMode,
};
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::{Delegate, Primitive};
use crate::codegen::parser::type_parser::TypeParserWithContext;
use anyhow::bail;
use std::collections::HashMap;
use syn::Expr;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_array(
        &mut self,
        type_array: &syn::TypeArray,
    ) -> anyhow::Result<IrType> {
        let length: usize = match &type_array.len {
            Expr::Lit(lit) => match &lit.lit {
                syn::Lit::Int(x) => x.base10_parse()?,
                _ => bail!("Cannot parse array length"),
            },
            _ => bail!("Cannot parse array length"),
        };

        let mode = match self.parse_type(&type_array.elem)? {
            Primitive(primitive) => IrTypeDelegateArrayMode::Primitive(primitive),
            others => IrTypeDelegateArrayMode::General(Box::new(others)),
        };

        let namespace = (self.inner.array_parser_info.namespace_of_parsed_types)
            .entry((mode.clone(), length))
            .or_insert(self.context.initiated_namespace.clone())
            .to_owned();

        Ok(Delegate(IrTypeDelegate::Array(IrTypeDelegateArray {
            namespace,
            length,
            mode,
        })))
    }
}

#[derive(Clone, Debug, Default)]
pub(super) struct ArrayParserInfo {
    namespace_of_parsed_types: HashMap<(IrTypeDelegateArrayMode, usize), Namespace>,
}
