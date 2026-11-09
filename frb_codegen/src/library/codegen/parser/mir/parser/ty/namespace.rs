use crate::codegen::parser::mir::parser::ty::TypeParserWithContext;
use crate::utils::namespace::Namespace;
use itertools::Itertools;
use syn::visit_mut::VisitMut;
use syn::{Path, Type};

impl TypeParserWithContext<'_, '_, '_> {
    pub(crate) fn parse_namespace_by_name(&mut self, raw: &str) -> Option<Namespace> {
        self.parse_struct_namespace(raw)
            .or_else(|| self.parse_enum_namespace(raw))
    }

    /// Namespaces of the structs/enums referenced in the generic arguments of `ty`,
    /// e.g. the namespace of `D` for `C<D>`.
    pub(crate) fn parse_generic_arg_namespaces(&mut self, ty: &Type) -> Vec<Namespace> {
        extract_generic_arg_type_names(ty)
            .iter()
            .filter_map(|name| self.parse_namespace_by_name(name))
            .unique()
            .collect()
    }
}

/// e.g. `a::C<b::D, Vec<E>>` -> `["D", "Vec", "E"]`
fn extract_generic_arg_type_names(ty: &Type) -> Vec<String> {
    let mut collector = TypeNameCollector::default();
    let mut ty = ty.clone();
    if let Type::Path(type_path) = &mut ty {
        // Skip the outer type itself, only look into its generic arguments
        for segment in type_path.path.segments.iter_mut() {
            collector.visit_path_arguments_mut(&mut segment.arguments);
        }
    } else {
        collector.visit_type_mut(&mut ty);
    }
    collector.0.into_iter().unique().collect()
}

#[derive(Default)]
struct TypeNameCollector(Vec<String>);

impl VisitMut for TypeNameCollector {
    fn visit_path_mut(&mut self, path: &mut Path) {
        if let Some(segment) = path.segments.last() {
            self.0.push(segment.ident.to_string());
        }
        syn::visit_mut::visit_path_mut(self, path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(raw: &str) -> Vec<String> {
        extract_generic_arg_type_names(&syn::parse_str(raw).unwrap())
    }

    /// Collects simple, nested, multiple and path-qualified generic arguments, but not the outer type.
    #[test]
    fn test_extract_generic_arg_type_names() {
        assert_eq!(names("C<D>"), vec!["D"]);
        assert_eq!(names("C<Vec<D>>"), vec!["Vec", "D"]);
        assert_eq!(names("C<D, F>"), vec!["D", "F"]);
        assert_eq!(names("a::C<crate::b::D, D>"), vec!["D"]);
        assert!(names("C").is_empty());
    }
}
