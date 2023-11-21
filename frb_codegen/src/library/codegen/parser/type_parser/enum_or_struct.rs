use crate::codegen::ir::namespace::NamespacedName;
use crate::codegen::ir::pack::IrStructPool;
use crate::codegen::ir::ty::IrType;
use crate::codegen::parser::type_parser::unencodable::{
    parse_path_type_to_unencodable, SplayedSegment,
};
use std::collections::{HashMap, HashSet};
use syn::{Ident, TypePath};

pub(super) trait EnumOrStructParser<Id: From<NamespacedName>, Obj, SrcObj> {
    fn parse(
        &mut self,
        type_path: &TypePath,
        splayed_segments: &[SplayedSegment],
        last_segment: &SplayedSegment,
    ) -> anyhow::Result<Option<IrType>> {
        if let (name, _) = last_segment {
            if let Some(src_object) = self.src_objects().get(*name) {
                let ident: Id = NamespacedName::new(TODO, name.to_string()).into();

                if (self.parser_info().parsing_or_parsed_objects).insert(ident.clone().0) {
                    let (name, wrapper_name) =
                        compute_name_and_wrapper_name(&src_object.0.ident, src_object.0.mirror);

                    match self.parse_inner(src_object)? {
                        Some(parsed_object) => {
                            (self.parser_info().object_pool).insert(ident.clone(), parsed_object)
                        }
                        None => {
                            return Ok(Some(parse_path_type_to_unencodable(
                                type_path,
                                splayed_segments,
                            )))
                        }
                    };
                }

                return Ok(Some(self.construct_output(ident)?));
            }
        }

        Ok(None)
    }

    fn parse_inner(
        &mut self,
        src_object: &SrcObj,
        name: NamespacedName,
        wrapper_name: Option<NamespacedName>,
    ) -> anyhow::Result<Option<Obj>>;

    fn construct_output(&self, ident: Id) -> anyhow::Result<IrType>;

    fn src_objects(&self) -> &HashMap<String, &SrcObj>;

    fn parser_info(&mut self) -> &mut EnumOrStructParserInfo<Id, Obj>;
}

#[derive(Clone, Debug, Default)]
pub(super) struct EnumOrStructParserInfo<Id, Obj> {
    parsing_or_parsed_objects: HashSet<NamespacedName>,
    pub(super) object_pool: HashMap<Id, Obj>,
}

fn compute_name_and_wrapper_name(
    ident: &Ident,
    mirror: bool,
) -> (NamespacedName, Option<NamespacedName>) {
    let name = ident.to_string();
    let wrapper_name = if mirror {
        Some(format!("mirror_{name}"))
    } else {
        None
    };
    // (name, wrapper_name) // TODO
    (TODO, TODO)
}
