use syn::{PathSegment, Type, TypePath};

pub(crate) fn convert_ident_str(ty: &Type) -> Option<String> {
    if let Type::Path(TypePath { qself: _, path }) = ty {
        if let Some(PathSegment { ident, .. }) = path.segments.first() {
            return Some(ident.to_string());
        }
    }

    // Unhandled case, return None
    None
}
