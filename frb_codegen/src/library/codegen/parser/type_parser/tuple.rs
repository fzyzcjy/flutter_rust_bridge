use crate::codegen::mir::field::IrField;
use crate::codegen::mir::ident::IrIdent;
use crate::codegen::mir::namespace::NamespacedName;
use crate::codegen::mir::ty::primitive::IrTypePrimitive;
use crate::codegen::mir::ty::record::IrTypeRecord;
use crate::codegen::mir::ty::structure::{IrStruct, IrStructIdent, IrTypeStructRef};
use crate::codegen::mir::ty::IrType;
use crate::codegen::mir::ty::IrType::Primitive;
use crate::codegen::parser::type_parser::TypeParserWithContext;
use crate::library::codegen::mir::ty::IrTypeTrait;
use anyhow::Result;
use itertools::Itertools;
use syn::TypeTuple;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_tuple(&mut self, type_tuple: &TypeTuple) -> anyhow::Result<IrType> {
        if type_tuple.elems.is_empty() {
            return Ok(Primitive(IrTypePrimitive::Unit));
        }

        let values = (type_tuple.elems.iter())
            .map(|elem| self.parse_type(elem))
            .collect::<Result<Vec<_>>>()?;

        Ok(IrType::Record(self.create_ir_record(values)))
    }

    pub(crate) fn create_ir_record(&mut self, values: Vec<IrType>) -> IrTypeRecord {
        let namespace = self.context.initiated_namespace.clone();

        let safe_ident = format!(
            "__record__{}",
            (values.iter().map(IrType::safe_ident)).join("_")
        );

        self.inner.struct_parser_info.object_pool.insert(
            IrStructIdent(NamespacedName::new(namespace.clone(), safe_ident.clone())),
            IrStruct {
                name: NamespacedName::new(namespace.clone(), safe_ident.clone()),
                wrapper_name: None,
                is_fields_named: true,
                dart_metadata: vec![],
                ignore: false,
                generate_hash: true,
                generate_eq: true,
                comments: vec![],
                fields: values
                    .iter()
                    .enumerate()
                    .map(|(idx, ty)| IrField {
                        ty: ty.clone(),
                        name: IrIdent::new(format!("field{idx}")),
                        is_final: true,
                        is_rust_public: None,
                        comments: vec![],
                        default: None,
                        settings: Default::default(),
                    })
                    .collect(),
            },
        );

        IrTypeRecord {
            inner: IrTypeStructRef {
                // name: safe_ident,
                // freezed: false,
                // empty: false,
                ident: IrStructIdent(NamespacedName::new(namespace, safe_ident)),
                is_exception: false,
            },
            values: values.into_boxed_slice(),
        }
    }
}
