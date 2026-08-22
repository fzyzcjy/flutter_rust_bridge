use crate::codegen::ir::mir::ty::MirType;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use crate::utils::namespace::NamespacedName;

crate::mir! {
pub struct MirCustomSerDes {
    pub inner_type: Box<MirType>,
    pub rust_api_type: Box<MirType>,
    pub dart_api_type: String,
    pub dart2rust: MirCustomSerDesHalf,
    pub rust2dart: MirCustomSerDesHalf,
}

pub struct MirCustomSerDesHalf {
    pub dart_code: String,
    pub rust_function: NamespacedName,
}
}

impl MirCustomSerDes {
    pub(crate) fn cleared_rust_api_type(&self) -> String {
        if let MirType::RustAutoOpaqueImplicit(ty) = &*self.rust_api_type {
            ty.raw.string.with_original_lifetime().to_owned()
        } else {
            self.rust_api_type.rust_api_type()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{MirCustomSerDes, MirCustomSerDesHalf};
    use crate::codegen::ir::mir::func::OwnershipMode;
    use crate::codegen::ir::mir::llfetime_aware_type::MirLifetimeAwareType;
    use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
    use crate::codegen::ir::mir::ty::rust_auto_opaque_implicit::{
        MirRustAutoOpaqueRaw, MirTypeRustAutoOpaqueImplicit,
    };
    use crate::codegen::ir::mir::ty::rust_opaque::{
        MirRustOpaqueInner, MirTypeRustOpaque, RustOpaqueCodecMode,
    };
    use crate::codegen::ir::mir::ty::MirType;
    use crate::utils::namespace::{Namespace, NamespacedName};

    fn custom_ser_des(rust_api_type: MirType) -> MirCustomSerDes {
        MirCustomSerDes {
            inner_type: Box::new(MirType::Primitive(MirTypePrimitive::U8)),
            rust_api_type: Box::new(rust_api_type),
            dart_api_type: "Object".into(),
            dart2rust: MirCustomSerDesHalf {
                dart_code: String::new(),
                rust_function: NamespacedName::new(Namespace::default(), "decode".into()),
            },
            rust2dart: MirCustomSerDesHalf {
                dart_code: String::new(),
                rust_function: NamespacedName::new(Namespace::default(), "encode".into()),
            },
        }
    }

    /// Preserves original lifetimes only for implicit Rust auto-opaque API types.
    #[test]
    fn clears_rust_api_type_with_the_correct_lifetime_policy() {
        let implicit = MirType::RustAutoOpaqueImplicit(MirTypeRustAutoOpaqueImplicit {
            ownership_mode: OwnershipMode::Owned,
            inner: MirTypeRustOpaque {
                namespace: Namespace::default(),
                inner: MirRustOpaqueInner(MirLifetimeAwareType::new("Thing<'a>".into())),
                codec: RustOpaqueCodecMode::Nom,
                dart_api_type: None,
                brief_name: false,
            },
            raw: MirRustAutoOpaqueRaw {
                string: MirLifetimeAwareType::new("Thing<'a>".into()),
                segments: vec![],
            },
            reason: None,
            ignore: false,
        });

        assert_eq!(
            custom_ser_des(implicit).cleared_rust_api_type(),
            "Thing<'a>"
        );
        assert_eq!(
            custom_ser_des(MirType::Primitive(MirTypePrimitive::U8)).cleared_rust_api_type(),
            "u8"
        );
    }
}
