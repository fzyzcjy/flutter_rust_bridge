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

    /// Additional indirection for types put behind a vector
    #[inline]
    pub fn rust_ptr_modifier(&self) -> &'static str {
        match self {
            Optional(_) | Delegate(IrTypeDelegate::String) => "*mut ",
            _ => "",
        }
    }

    // -> is_struct_or_enum_or_record
    // #[inline]
    // pub fn is_struct(&self) -> bool {
    //     matches!(self, StructRef(_) | EnumRef(_) | Record(_))
    // }

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

    fn dart_param_type(&self) -> &'static str {
        "dynamic"
    }
}
