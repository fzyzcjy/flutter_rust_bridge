pub(crate) mod boxed;
pub(crate) mod dart_fn;
pub(crate) mod dart_opaque;
pub(crate) mod delegate;
pub(crate) mod dynamic;
pub(crate) mod enumeration;
pub(crate) mod general_list;
pub(crate) mod optional;
pub(crate) mod primitive;
pub(crate) mod primitive_list;
pub(crate) mod record;
pub(crate) mod rust_auto_opaque_implicit;
pub(crate) mod rust_opaque;
pub(crate) mod structure;
pub(crate) mod trait_def;

use crate::codegen::ir::mir::pack::{MirEnumPool, MirPack, MirStructPool};
use crate::codegen::ir::mir::ty::delegate::MirTypeDelegate;
use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
use crate::utils::namespace::Namespace;
use enum_dispatch::enum_dispatch;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};

crate::mir! {
#[no_serde]
// Remark: "Ty" instead of "Type", since "type" is a reserved word in Rust.
#[enum_dispatch(MirTypeTrait)]
pub enum MirType {
    // alphabetical order
    Boxed(boxed::MirTypeBoxed),
    DartFn(dart_fn::MirTypeDartFn),
    DartOpaque(dart_opaque::MirTypeDartOpaque),
    Delegate(delegate::MirTypeDelegate),
    Dynamic(dynamic::MirTypeDynamic),
    EnumRef(enumeration::MirTypeEnumRef),
    GeneralList(general_list::MirTypeGeneralList),
    Optional(optional::MirTypeOptional),
    Primitive(primitive::MirTypePrimitive),
    PrimitiveList(primitive_list::MirTypePrimitiveList),
    Record(record::MirTypeRecord),
    RustAutoOpaqueImplicit(rust_auto_opaque_implicit::MirTypeRustAutoOpaqueImplicit),
    RustOpaque(rust_opaque::MirTypeRustOpaque),
    StructRef(structure::MirTypeStructRef),
    TraitDef(trait_def::MirTypeTraitDef),
}
}

impl MirType {
    pub fn visit_types<F: FnMut(&MirType) -> bool>(
        &self,
        f: &mut F,
        mir_context: &impl MirContext,
    ) {
        if f(self) {
            return;
        }
        self.visit_children_types(f, mir_context);
    }

    #[inline]
    pub fn is_struct_or_enum_or_record(&self) -> bool {
        matches!(
            self,
            MirType::StructRef(_) | MirType::EnumRef(_) | MirType::Record(_)
        )
    }

    #[inline]
    pub fn is_primitive(&self) -> bool {
        self.as_primitive().is_some()
    }

    #[inline]
    pub fn is_array(&self) -> bool {
        matches!(self, MirType::Delegate(MirTypeDelegate::Array(_)))
    }
}

#[enum_dispatch]
pub trait MirTypeTrait {
    fn visit_children_types<F: FnMut(&MirType) -> bool>(
        &self,
        f: &mut F,
        mir_context: &impl MirContext,
    );

    /// A string that can be used as an identifier safely, i.e. without any special characters inside
    fn safe_ident(&self) -> String;

    /// Why `rust_api_type` is in `ir` while `dart_api_type` is in `generator::api_dart` -
    /// Because the former is intrinsic information of a parsed Rust type, while the latter is
    /// part of the code to be generated.
    fn rust_api_type(&self) -> String;

    #[inline]
    fn as_primitive(&self) -> Option<&MirTypePrimitive> {
        None
    }

    fn self_namespace(&self) -> Option<Namespace> {
        None
    }

    fn should_ignore(&self, _mir_context: &impl MirContext) -> bool {
        false
    }

    // TODO move
    fn cloned_getter_semantics_reasonable(&self) -> bool {
        false
    }
}

impl From<Box<MirType>> for MirType {
    fn from(val: Box<MirType>) -> Self {
        *val
    }
}

impl Serialize for MirType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let len = 3;
        let mut state = serializer.serialize_struct("MirType", len)?;

        fn ser<S: Serializer, T: Serialize + MirTypeTrait>(
            state: &mut <S as Serializer>::SerializeStruct,
            ty: &str,
            data: &T,
        ) -> Result<(), S::Error> {
            state.serialize_field("type", ty)?;
            state.serialize_field("safe_ident", &data.safe_ident())?;
            state.serialize_field("data", data)?;
            Ok(())
        }
        match self {
            MirType::Boxed(inner) => ser::<S, _>(&mut state, "Boxed", inner),
            MirType::DartFn(inner) => ser::<S, _>(&mut state, "DartFn", inner),
            MirType::DartOpaque(inner) => ser::<S, _>(&mut state, "DartOpaque", inner),
            MirType::Delegate(inner) => ser::<S, _>(&mut state, "Delegate", inner),
            MirType::Dynamic(inner) => ser::<S, _>(&mut state, "Dynamic", inner),
            MirType::EnumRef(inner) => ser::<S, _>(&mut state, "EnumRef", inner),
            MirType::GeneralList(inner) => ser::<S, _>(&mut state, "GeneralList", inner),
            MirType::Optional(inner) => ser::<S, _>(&mut state, "Optional", inner),
            MirType::Primitive(inner) => ser::<S, _>(&mut state, "Primitive", inner),
            MirType::PrimitiveList(inner) => ser::<S, _>(&mut state, "PrimitiveList", inner),
            MirType::Record(inner) => ser::<S, _>(&mut state, "Record", inner),
            MirType::RustAutoOpaqueImplicit(inner) => {
                ser::<S, _>(&mut state, "RustAutoOpaque", inner)
            }
            MirType::RustOpaque(inner) => ser::<S, _>(&mut state, "RustOpaque", inner),
            MirType::StructRef(inner) => ser::<S, _>(&mut state, "StructRef", inner),
            MirType::TraitDef(inner) => ser::<S, _>(&mut state, "TraitDef", inner),
        }?;

        state.end()
    }
}

pub(crate) trait MirContext {
    fn struct_pool(&self) -> &MirStructPool;

    fn enum_pool(&self) -> &MirEnumPool;
}

impl MirContext for MirPack {
    fn struct_pool(&self) -> &MirStructPool {
        &self.struct_pool
    }

    fn enum_pool(&self) -> &MirEnumPool {
        &self.enum_pool
    }
}
