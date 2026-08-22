use super::rust_opaque::RUST_OPAQUE_AS_PRIMITIVE;
use crate::codegen::ir::mir::func::OwnershipMode;
use crate::codegen::ir::mir::llfetime_aware_type::MirLifetimeAwareType;
use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
use crate::codegen::ir::mir::ty::rust_opaque::{MirTypeRustOpaque, NameComponent};
use crate::codegen::ir::mir::ty::{MirContext, MirType, MirTypeTrait};
use crate::utils::namespace::Namespace;

crate::mir! {
pub struct MirTypeRustAutoOpaqueImplicit {
    pub ownership_mode: OwnershipMode,
    pub inner: MirTypeRustOpaque,
    pub raw: MirRustAutoOpaqueRaw,
    pub reason: Option<MirTypeRustAutoOpaqueImplicitReason>,
    pub ignore: bool,
}

/// Original type without any transformation
pub struct MirRustAutoOpaqueRaw {
    pub string: MirLifetimeAwareType,
    pub segments: Vec<NameComponent>,
}

#[derive(Copy)]
pub enum MirTypeRustAutoOpaqueImplicitReason {
    StructOrEnumRequireOpaque,
}
}

impl MirTypeTrait for MirTypeRustAutoOpaqueImplicit {
    fn visit_children_types<F: FnMut(&MirType) -> bool>(
        &self,
        f: &mut F,
        mir_context: &impl MirContext,
    ) {
        MirType::RustOpaque(self.inner.clone()).visit_types(f, mir_context)
    }

    fn safe_ident(&self) -> String {
        format!("Auto_{}_{}", self.ownership_mode, self.inner.safe_ident())
    }

    fn rust_api_type(&self) -> String {
        match self.ownership_mode {
            // Different mechanisms for Owned vs Ref/RefMut
            OwnershipMode::Owned => self.raw.string.with_static_lifetime().clone(),
            OwnershipMode::Ref | OwnershipMode::RefMut => self.inner.rust_api_type(),
        }
    }

    fn self_namespace(&self) -> Option<Namespace> {
        Some(self.inner.namespace.clone())
    }

    fn as_primitive(&self) -> Option<&MirTypePrimitive> {
        Some(&RUST_OPAQUE_AS_PRIMITIVE)
    }

    fn should_ignore(&self, _mir_context: &impl MirContext) -> bool {
        self.ignore
    }
}

impl MirTypeRustAutoOpaqueImplicit {
    pub(crate) fn needs_move(&self) -> bool {
        self.ownership_mode == OwnershipMode::Owned
    }

    pub(crate) fn sanitized_type(&self) -> String {
        self.inner.sanitized_type()
    }
}
