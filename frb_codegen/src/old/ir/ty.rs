use crate::{ir::*, target::Target};
use nnum_dispatch::enum_dispatch;
use std::collections::HashSet;
use IrType::*;

impl IrType {
    // TODO note it has *duplicate* with IrPack
    pub fn distinct_types(&self, ir_pack: &IrPack) -> Vec<IrType> {
        let mut seen_idents = HashSet::new();
        let mut ans = Vec::new();
        self.visit_types(
            &mut |ty| {
                let ident = ty.safe_ident();
                let contains = seen_idents.contains(&ident);
                if !contains {
                    seen_idents.insert(ident);
                    ans.push(ty.clone());
                }
                contains
            },
            ir_pack,
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
    pub fn is_struct(&self) -> bool {
        matches!(self, StructRef(_) | EnumRef(_) | Record(_))
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
    // fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_pack: &IrPack); // moved
    fn safe_ident(&self) -> String;
    fn dart_api_type(&self) -> String;
    fn dart_wire_type(&self, target: Target) -> String;
    fn rust_api_type(&self) -> String;
    fn rust_wire_type(&self, target: Target) -> String;
    fn intodart_type(&self, _ir_pack: &IrPack) -> String {
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
