use crate::codegen::ir::namespace::Namespace;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::{IrContext, IrType, IrTypeTrait};
use serde::Serialize;
use strum_macros::{Display, EnumIter};

crate::ir! {
pub struct IrTypeRustOpaque {
    pub namespace: Namespace,
    pub inner: Box<IrType>,
    pub codec: RustOpaqueCodecMode,
    pub brief_name: bool,
}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Hash, Display, EnumIter)]
pub(crate) enum RustOpaqueCodecMode {
    Nom,
    Moi,
}

impl RustOpaqueCodecMode {
    pub(crate) fn arc_ty(self) -> &'static str {
        match self {
            RustOpaqueCodecMode::Nom => "StdArc",
            RustOpaqueCodecMode::Moi => "MoiArc",
        }
    }

    pub(crate) fn needs_unsafe_block(self) -> bool {
        self == RustOpaqueCodecMode::Nom
    }
}

impl IrTypeRustOpaque {
    pub fn new(
        namespace: Namespace,
        inner: IrType,
        codec: RustOpaqueCodecMode,
        brief_name: bool,
    ) -> Self {
        Self {
            namespace,
            inner: Box::new(inner),
            codec,
            brief_name,
        }
    }

    pub(crate) fn get_delegate(&self) -> IrType {
        Self::DELEGATE_TYPE.clone()
    }

    pub(crate) const DELEGATE_TYPE: IrType = IrType::Primitive(IrTypePrimitive::Usize);
}

impl IrTypeTrait for IrTypeRustOpaque {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(
        &self,
        f: &mut F,
        ir_context: &impl IrContext,
    ) {
        self.get_delegate().visit_types(f, ir_context)
    }

    fn safe_ident(&self) -> String {
        format!("RustOpaque_{}", self.inner.safe_ident())
    }

    fn rust_api_type(&self) -> String {
        format!(
            "flutter_rust_bridge::RustOpaque<{}, {}RustOpaqueCodec>",
            self.inner.rust_api_type(),
            self.codec.to_string(),
        )
    }

    fn self_namespace(&self) -> Option<Namespace> {
        Some(self.namespace.clone())
    }
}
