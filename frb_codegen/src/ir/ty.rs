use crate::ir::*;
use enum_dispatch::enum_dispatch;
use ApiType::*;

/// Remark: "Ty" instead of "Type", since "type" is a reserved word in Rust.
#[enum_dispatch(ApiTypeChild)]
#[derive(Debug, Clone)]
pub enum ApiType {
    Primitive(ApiTypePrimitive),
    Delegate(ApiTypeDelegate),
    PrimitiveList(ApiTypePrimitiveList),
    Optional(ApiTypeOptional),
    GeneralList(ApiTypeGeneralList),
    StructRef(ApiTypeStructRef),
    Boxed(ApiTypeBoxed),
    EnumRef(ApiTypeEnumRef),
}

impl ApiType {
    pub fn visit_types<F: FnMut(&ApiType) -> bool>(&self, f: &mut F, api_file: &ApiFile) {
        if f(self) {
            return;
        }

        match &self {
            PrimitiveList(inner) => {
                f(&ApiType::Primitive(inner.primitive.clone()));
            }
            GeneralList(inner) => inner.inner.visit_types(f, api_file),
            StructRef(struct_ref) => {
                for field in &struct_ref.get(api_file).fields {
                    field.ty.visit_types(f, api_file);
                }
            }
            Boxed(inner) => inner.inner.visit_types(f, api_file),
            Delegate(d) => d.get_delegate().visit_types(f, api_file),
            Optional(inner) => inner.inner.visit_types(f, api_file),
            EnumRef(enu) => {
                let enu = enu.get(api_file);
                for variant in enu.variants() {
                    if let ApiVariantKind::Struct(st) = &variant.kind {
                        st.fields
                            .iter()
                            .for_each(|field| field.ty.visit_types(f, api_file));
                    }
                }
            }
            Primitive(_) => {}
        }
    }

    #[inline]
    pub fn required_modifier(&self) -> &'static str {
        match self {
            Optional(_) => "",
            _ => "required ",
        }
    }

    // api_fill functions target this type instead of the delegate.
    #[inline]
    pub fn optional_inner(&self) -> &ApiType {
        match self {
            Optional(inner) => &inner.inner,
            _ => self,
        }
    }

    /// Additional indirection for types put behind a vector
    #[inline]
    pub fn optional_ptr_modifier(&self) -> &'static str {
        match self {
            Optional(_) | Delegate(ApiTypeDelegate::String) => "*mut ",
            _ => "",
        }
    }
}

#[enum_dispatch]
pub trait ApiTypeChild {
    fn safe_ident(&self) -> String;

    fn dart_api_type(&self) -> String;

    fn dart_wire_type(&self) -> String;

    fn rust_api_type(&self) -> String;

    fn rust_wire_type(&self) -> String;

    fn rust_wire_modifier(&self) -> String {
        if self.rust_wire_is_pointer() {
            "*mut ".to_string()
        } else {
            "".to_string()
        }
    }

    fn rust_wire_is_pointer(&self) -> bool {
        false
    }
}

pub fn optional_boundary_index(types: &[&ApiType]) -> Option<usize> {
    types
        .iter()
        .enumerate()
        .find(|ty| matches!(ty.1, Optional(_)))
        .and_then(|(idx, _)| {
            (&types[idx..])
                .iter()
                .all(|ty| matches!(ty, Optional(_)))
                .then(|| idx)
        })
}
