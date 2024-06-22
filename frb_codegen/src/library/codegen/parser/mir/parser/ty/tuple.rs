use crate::codegen::ir::mir::field::MirField;
use crate::codegen::ir::mir::ident::MirIdent;
use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
use crate::codegen::ir::mir::ty::record::MirTypeRecord;
use crate::codegen::ir::mir::ty::structure::{MirStruct, MirStructIdent, MirTypeStructRef};
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::ir::mir::ty::MirType::Primitive;
use crate::codegen::parser::mir::parser::ty::TypeParserWithContext;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use crate::utils::namespace::NamespacedName;
use anyhow::Result;
use itertools::Itertools;
use syn::TypeTuple;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_tuple(&mut self, type_tuple: &TypeTuple) -> anyhow::Result<MirType> {
        if type_tuple.elems.is_empty() {
            return Ok(Primitive(MirTypePrimitive::Unit));
        }

        let values = (type_tuple.elems.iter())
            .map(|elem| self.parse_type(elem))
            .collect::<Result<Vec<_>>>()?;

        Ok(MirType::Record(self.create_mir_record(values)))
    }

    pub(crate) fn create_mir_record(&mut self, values: Vec<MirType>) -> MirTypeRecord {
        let namespace = self.context.initiated_namespace.clone();

        let safe_ident = format!(
            "__record__{}",
            (values.iter().map(MirType::safe_ident)).join("_")
        );

        self.inner.struct_parser_info.object_pool.insert(
            MirStructIdent(NamespacedName::new(namespace.clone(), safe_ident.clone())),
            MirStruct {
                name: NamespacedName::new(namespace.clone(), safe_ident.clone()),
                wrapper_name: None,
                is_fields_named: true,
                dart_metadata: vec![],
                ignore: false,
                generate_hash: true,
                generate_eq: true,
                ui_state: false,
                comments: vec![],
                fields: values
                    .iter()
                    .enumerate()
                    .map(|(idx, ty)| MirField {
                        ty: ty.clone(),
                        name: MirIdent::new(format!("field{idx}"), None),
                        is_final: true,
                        is_rust_public: None,
                        comments: vec![],
                        default: None,
                        settings: Default::default(),
                    })
                    .collect(),
            },
        );

        MirTypeRecord {
            inner: MirTypeStructRef {
                // name: safe_ident,
                // freezed: false,
                // empty: false,
                ident: MirStructIdent(NamespacedName::new(namespace, safe_ident)),
                is_exception: false,
            },
            values: values.into_boxed_slice(),
        }
    }
}
