use std::collections::HashSet;

use crate::{ir::*, target::Target};
use enum_dispatch::enum_dispatch;
use IrType::*;

/// Remark: "Ty" instead of "Type", since "type" is a reserved word in Rust.
#[enum_dispatch(IrTypeTrait)]
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum IrType {
    Primitive(IrTypePrimitive),
    Delegate(IrTypeDelegate),
    PrimitiveList(IrTypePrimitiveList),
    Optional(IrTypeOptional),
    GeneralList(IrTypeGeneralList),
    StructRef(IrTypeStructRef),
    Boxed(IrTypeBoxed),
    EnumRef(IrTypeEnumRef),
    SyncReturn(IrTypeSyncReturn),
    Opaque(IrTypeOpaque),
    ImplTrait(IrTypeImplTrait),
}

impl IrType {
    pub fn visit_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_file: &IrFile) {
        if f(self) {
            return;
        }

        self.visit_children_types(f, ir_file);
    }

    pub fn distinct_types(&self, ir_file: &IrFile) -> Vec<IrType> {
        let mut seen_idents = HashSet::new();
        let mut ans = Vec::new();
        self.visit_types(
            &mut |ty| {
                let ident = ty.safe_ident();
                let contains = seen_idents.contains(&ident);
                if !contains {
                    seen_idents.insert(ident);
                    ans.push(match ty {
                        IrType::ImplTrait(i) => i.clone().to_enum_ir_type(),

                        any => any.clone(),
                    });
                }
                contains
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
            _ => None,
        }
    }

    #[inline]
    pub fn is_primitive(&self) -> bool {
        self.as_primitive().is_some()
    }

    #[inline]
    pub fn is_array(&self) -> bool {
        matches!(self, IrType::Delegate(IrTypeDelegate::Array(_),))
    }

    #[inline]
    pub fn is_need_wrap_box(&self) -> bool {
        matches!(self, StructRef(_) | EnumRef(_) | ImplTrait(_))
    }

    #[inline]
    pub fn is_opaque(&self) -> bool {
        matches!(self, Opaque(_))
    }

    /// In WASM, these types belong to the JS scope-local heap, **NOT** the Rust heap
    /// and therefore do not implement [Send].
    #[inline]
    pub fn is_js_value(&self) -> bool {
        match self {
            Self::GeneralList(_)
            | Self::StructRef(_)
            | Self::EnumRef(_)
            | Self::ImplTrait(_)
            | Self::Opaque(_)
            | Self::Delegate(IrTypeDelegate::PrimitiveEnum { .. }) => true,
            Self::Boxed(IrTypeBoxed { inner, .. }) => inner.is_js_value(),
            _ => false,
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

pub fn optional_boundary_index(types: &[&IrType]) -> Option<usize> {
    types
        .iter()
        .enumerate()
        .find(|ty| matches!(ty.1, Optional(_)))
        .and_then(|(idx, _)| {
            types[idx..]
                .iter()
                .all(|ty| matches!(ty, Optional(_)))
                .then_some(idx)
        })
}
