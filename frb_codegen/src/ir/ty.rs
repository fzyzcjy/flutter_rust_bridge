use crate::{ir::*, target::Target};
use convert_case::{Case, Casing};
use enum_dispatch::enum_dispatch;
use std::collections::HashSet;
use IrType::*;

crate::ir! {
/// Remark: "Ty" instead of "Type", since "type" is a reserved word in Rust.
#[enum_dispatch(IrTypeTrait)]
pub enum IrType {
    Primitive(IrTypePrimitive),
    Delegate(IrTypeDelegate),
    PrimitiveList(IrTypePrimitiveList),
    Optional(IrTypeOptional),
    OptionalList(IrTypeOptionalList),
    GeneralList(IrTypeGeneralList),
    StructRef(IrTypeStructRef),
    Boxed(IrTypeBoxed),
    EnumRef(IrTypeEnumRef),
    SyncReturn(IrTypeSyncReturn),
    DartOpaque(IrTypeDartOpaque),
    RustOpaque(IrTypeRustOpaque),
    Dynamic(IrTypeDynamic),
    Record(IrTypeRecord),
    Unencodable(IrTypeUnencodable),
}
}

impl IrType {
    pub fn get_rust_name(&self) -> String {
        let type_raw_name = self.safe_ident();
        if !self.is_list(true) {
            type_raw_name.to_case(Case::Pascal)
        } else {
            type_raw_name
        }
    }

    pub fn visit_self_types_recursively<F: FnMut(&IrType) -> bool>(
        &self,
        f: &mut F,
        ir_file: &IrFile,
    ) {
        if f(self) {
            return;
        }
        self.visit_children_types(f, ir_file);
    }

    pub fn get_core_type(&self) -> &IrType {
        match self {
            IrType::Boxed(s) => s.inner.get_core_type(),
            IrType::Optional(s) => s.inner.get_core_type(),
            IrType::SyncReturn(s) => s.inner().get_core_type(),
            _ => self,
        }
    }

    // TODO: combine with `visit_types_recursively_used_in_the_func`?
    // Return all types(fields) used in this type, including itself.
    pub fn get_all_distinct_types(
        &self,
        diff_by_safe_ident: bool,
        ir_file: &IrFile,
    ) -> Vec<IrType> {
        let mut seen = HashSet::new();
        let mut seen_idents = HashSet::new();
        let mut ans = Vec::new();
        self.visit_self_types_recursively(
            &mut |ty| {
                if diff_by_safe_ident {
                    let ident = ty.safe_ident();
                    let contains = seen_idents.contains(&ident);
                    if !contains {
                        seen_idents.insert(ident);
                        ans.push(ty.clone());
                    }
                    contains
                } else {
                    let contains = seen.contains(ty);
                    if !contains {
                        seen.insert(ty.clone());
                        ans.push(ty.clone());
                    }
                    contains
                }
            },
            ir_file,
        );
        ans
    }

    #[inline]
    pub fn dart_required_modifier(&self) -> &'static str {
        match self {
            Optional(_) => "",
            _ => "required ",
        }
    }

    /// Additional indirection for types put behind a vector
    #[inline]
    pub fn rust_ptr_modifier(&self) -> &'static str {
        match self {
            Optional(_) | Delegate(IrTypeDelegate::String) => "*mut ",
            _ => "",
        }
    }

    #[inline]
    pub fn as_primitive(&self) -> Option<&IrTypePrimitive> {
        match self {
            Primitive(repr) | Delegate(IrTypeDelegate::PrimitiveEnum { repr, .. }) => Some(repr),
            #[cfg(feature = "chrono")]
            Delegate(IrTypeDelegate::Time(_)) => Some(&IrTypePrimitive::I64),
            _ => None,
        }
    }

    #[inline]
    pub fn is_primitive(&self) -> bool {
        self.as_primitive().is_some()
    }

    #[inline]
    pub fn is_array(&self) -> bool {
        matches!(self, IrType::Delegate(IrTypeDelegate::Array(_)))
    }

    #[inline]
    pub fn is_list(&self, exclude_string_time_list: bool) -> bool {
        if exclude_string_time_list {
            return matches!(self, |GeneralList(_)| PrimitiveList(_));
        }
        matches!(
            self,
            IrType::Delegate(IrTypeDelegate::StringList)
                | Delegate(IrTypeDelegate::TimeList(_))
                | GeneralList(_)
                | PrimitiveList(_)
        )
    }

    #[inline]
    pub fn is_struct_ref_or_enum_ref_or_record(&self) -> bool {
        matches!(self, StructRef(_) | EnumRef(_) | Record(_))
    }

    #[inline]
    pub fn is_struct_ref(&self) -> bool {
        matches!(self, StructRef(_))
    }

    #[inline]
    pub fn is_enum_ref(&self) -> bool {
        matches!(self, EnumRef(_))
    }

    #[inline]
    pub fn is_rust_opaque(&self) -> bool {
        matches!(self, RustOpaque(_))
    }

    #[inline]
    pub fn is_sync_rust_opaque(&self) -> bool {
        match self {
            SyncReturn(sync) => sync.clone().into_inner().is_rust_opaque(),
            _ => false,
        }
    }

    #[inline]
    pub fn is_dart_opaque(&self) -> bool {
        matches!(self, DartOpaque(_))
    }

    /// In WASM, these types belong to the JS scope-local heap, **NOT** the Rust heap and
    /// therefore do not implement [Send]. More specifically, these are types wasm-bindgen
    /// can't handle yet.
    pub fn is_js_value(&self) -> bool {
        match self {
            Self::GeneralList(_)
            | Self::OptionalList(_)
            | Self::StructRef(_)
            | Self::EnumRef(_)
            | Self::RustOpaque(_)
            | Self::DartOpaque(_)
            | Self::Record(_) => true,
            Self::Boxed(IrTypeBoxed { inner, .. }) => inner.is_js_value(),
            Self::Delegate(inner) => inner.get_delegate().is_js_value(),
            Self::Optional(inner) => inner.inner.is_js_value(),
            Self::Primitive(_) | Self::PrimitiveList(_) => false,
            Self::SyncReturn(_) | Self::Dynamic(_) | Self::Unencodable(_) => unreachable!(),
        }
    }

    #[inline]
    pub fn is_sync_return(&self) -> bool {
        matches!(self, IrType::SyncReturn(_))
    }

    #[inline]
    pub fn is_boxed(&self) -> bool {
        matches!(self, IrType::Boxed(_))
    }

    pub fn mirrored_nested(&self) -> Option<String> {
        match self {
            Self::StructRef(struct_ref) => Some(struct_ref.name.clone()),
            Self::Boxed(IrTypeBoxed { inner, .. }) => inner.mirrored_nested(),
            _ => None,
        }
    }
}

#[enum_dispatch]
pub trait IrTypeTrait {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_file: &IrFile);
    fn safe_ident(&self) -> String;
    fn dart_api_type(&self) -> String;
    fn dart_wire_type(&self, target: Target) -> String;
    fn rust_api_type(&self) -> String;
    fn rust_wire_type(&self, target: Target) -> String;
    fn intodart_type(&self, _ir_file: &IrFile) -> String {
        self.rust_api_type()
    }

    fn rust_wire_modifier(&self, target: Target) -> String {
        if self.rust_wire_is_pointer(target) {
            "*mut ".to_string()
        } else {
            "".to_string()
        }
    }

    fn rust_wire_is_pointer(&self, _target: Target) -> bool {
        false
    }

    fn dart_param_type(&self) -> &'static str {
        "dynamic"
    }
}

pub fn optional_boundary_index(fields: &[IrField]) -> Option<usize> {
    fields
        .iter()
        .enumerate()
        .find(|(_, field)| field.is_optional())
        .and_then(|(idx, _)| {
            fields[idx..]
                .iter()
                .all(|field| field.is_optional())
                .then_some(idx)
        })
}
