use crate::codegen::generator::codec::structs::CodecModePack;
use crate::codegen::ir::mir::comment::MirComment;
use crate::codegen::ir::mir::field::MirField;
use crate::codegen::ir::mir::ty::delegate::{MirTypeDelegate, MirTypeDelegatePrimitiveEnum};
use crate::codegen::ir::mir::ty::primitive::MirTypePrimitive;
use crate::codegen::ir::mir::ty::{MirContext, MirType, MirTypeTrait};
use crate::if_then_some;
use crate::utils::namespace::NamespacedName;
use convert_case::{Case, Casing};
use itertools::Itertools;

crate::mir! {
pub struct MirFunc {
    pub name: NamespacedName,
    pub dart_name: Option<String>,
    pub id: Option<i32>,
    pub inputs: Vec<MirFuncInput>,
    pub output: MirFuncOutput,
    pub owner: MirFuncOwnerInfo,
    pub mode: MirFuncMode,
    pub stream_dart_await: bool,
    pub rust_async: bool,
    pub initializer: bool,
    pub arg_mode: MirFuncArgMode,
    pub accessor: Option<MirFuncAccessorMode>,
    pub comments: Vec<MirComment>,
    pub codec_mode_pack: CodecModePack,
    pub rust_call_code: Option<String>,
    // When multiple func with same name, the one with higher priority will evict the other one
    pub override_priority: MirFuncOverridePriority,
    // Currently, we use serde only for tests. Since lineno can be unstable, we skip this field for comparison
    #[serde(skip_serializing)]
    pub src_lineno_pseudo: usize,
}

pub struct MirFuncInput {
    pub ownership_mode: Option<OwnershipMode>,
    pub inner: MirField,
}

pub struct MirFuncOutput {
    pub normal: MirType,
    pub error: Option<MirType>,
}

#[derive(Copy)]
pub enum MirFuncMode {
    Normal,
    Sync,
}

#[derive(Copy, Ord, PartialOrd, Default)]
pub struct MirFuncOverridePriority(pub i32);

#[derive(Copy)]
pub enum MirFuncArgMode {
    Positional,
    Named,
}

pub enum MirFuncOwnerInfo {
    Function,
    Method(MirFuncOwnerInfoMethod),
}

pub struct MirFuncOwnerInfoMethod {
    pub(crate) owner_ty: MirType,
    pub(crate) actual_method_name: String,
    pub(crate) actual_method_dart_name: Option<String>,
    pub(crate) mode: MirFuncOwnerInfoMethodMode,
    pub(crate) trait_def_name: Option<NamespacedName>,
}

pub enum MirFuncOwnerInfoMethodMode {
    Static,
    Instance,
}

#[derive(Copy)]
pub enum MirFuncAccessorMode {
    Getter,
    Setter,
}
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, serde::Serialize, strum_macros::Display)]
pub enum OwnershipMode {
    /// "T"
    Owned,
    /// "&T"
    Ref,
    /// "&mut T"
    RefMut,
}

impl OwnershipMode {
    pub(crate) fn prefix(&self) -> &'static str {
        match self {
            OwnershipMode::Owned => "",
            OwnershipMode::Ref => "&",
            OwnershipMode::RefMut => "&mut ",
        }
    }
}

impl MirFunc {
    pub(crate) fn fallible(&self) -> bool {
        self.output.error.is_some()
    }

    pub(crate) fn visit_types<F: FnMut(&MirType) -> bool>(
        &self,
        f: &mut F,
        mir_context: &impl MirContext,
    ) {
        // inputs
        for field in &self.inputs {
            field.inner.ty.visit_types(f, mir_context);
        }

        // output
        self.output.normal.visit_types(f, mir_context);
        let error_output = (self.output.error.as_ref().cloned())
            .unwrap_or(MirType::Primitive(MirTypePrimitive::Unit));
        error_output.visit_types(f, mir_context);

        // extra (#1838)
        if let MirFuncOwnerInfo::Method(MirFuncOwnerInfoMethod {
            owner_ty: enum_or_struct_ty,
            ..
        }) = &self.owner
        {
            enum_or_struct_ty.visit_types(f, mir_context);
        }
    }

    pub(crate) fn default_constructor_mode(&self) -> Option<MirFuncDefaultConstructorMode> {
        let method_info =
            if_then_some!(let MirFuncOwnerInfo::Method(info) = &self.owner , info).unwrap();
        if method_info.actual_method_name == "new" {
            if method_info.mode == MirFuncOwnerInfoMethodMode::Static
                && self.mode == MirFuncMode::Sync
            {
                Some(MirFuncDefaultConstructorMode::DartConstructor)
            } else {
                Some(MirFuncDefaultConstructorMode::StaticMethod)
            }
        } else {
            None
        }
    }

    pub(crate) fn locator_dart_api(&self) -> MirFuncDartApiLocator {
        MirFuncDartApiLocator {
            accessor: self.accessor,
            inner: match &self.owner {
                MirFuncOwnerInfo::Function => MirFuncDartApiLocatorInner::Function {
                    name: self.name.clone(),
                },
                MirFuncOwnerInfo::Method(method) => MirFuncDartApiLocatorInner::Method {
                    owner_name: method.owner_ty.safe_ident(),
                    actual_method_dart_name: (method.actual_method_dart_name.clone())
                        .unwrap_or(method.actual_method_name.clone()),
                },
            },
        }
    }

    pub(crate) fn name_dart_api(&self) -> String {
        (self.dart_name.clone()).unwrap_or_else(|| self.name.name.to_owned().to_case(Case::Camel))
    }

    pub(crate) fn name_dart_wire(&self) -> String {
        let raw = format!(
            "{}_{}",
            self.name.namespace.path().into_iter().join("_"),
            self.name.name
        );
        raw.to_case(Case::Camel)
    }
}

impl MirFuncOwnerInfoMethod {
    pub(crate) fn owner_ty_name(&self) -> Option<NamespacedName> {
        compute_name_of_owner_ty(&self.owner_ty)
    }
}

pub(crate) fn compute_name_of_owner_ty(owner_ty: &MirType) -> Option<NamespacedName> {
    Some(match owner_ty {
        MirType::StructRef(ty) => ty.ident.0.clone(),
        MirType::EnumRef(ty) => ty.ident.0.clone(),
        MirType::Delegate(MirTypeDelegate::PrimitiveEnum(MirTypeDelegatePrimitiveEnum {
            mir,
            ..
        })) => mir.ident.0.clone(),
        MirType::RustAutoOpaqueImplicit(ty) => {
            NamespacedName::new(ty.self_namespace().unwrap(), ty.rust_api_type())
        }
        _ => return None,
    })
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum MirFuncDefaultConstructorMode {
    DartConstructor,
    StaticMethod,
}

impl MirFuncAccessorMode {
    pub(crate) fn verb_str(&self) -> &'static str {
        match self {
            MirFuncAccessorMode::Getter => "get",
            MirFuncAccessorMode::Setter => "set",
        }
    }
}

impl MirFuncOverridePriority {
    pub(crate) const FRB_OVERRIDE: MirFuncOverridePriority = MirFuncOverridePriority(1);
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub(crate) struct MirFuncDartApiLocator {
    accessor: Option<MirFuncAccessorMode>,
    inner: MirFuncDartApiLocatorInner,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub(crate) enum MirFuncDartApiLocatorInner {
    Function {
        name: NamespacedName,
    },
    Method {
        owner_name: String,
        actual_method_dart_name: String,
    },
}
