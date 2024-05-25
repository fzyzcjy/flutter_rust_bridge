use crate::codegen::generator::codec::structs::CodecModePack;
use crate::codegen::ir::comment::IrComment;
use crate::codegen::ir::field::IrField;
use crate::codegen::ir::namespace::NamespacedName;
use crate::codegen::ir::ty::delegate::{IrTypeDelegate, IrTypeDelegatePrimitiveEnum};
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::{IrContext, IrType, IrTypeTrait};
use crate::if_then_some;
use convert_case::{Case, Casing};
use itertools::Itertools;

crate::ir! {
pub struct IrFunc {
    pub name: NamespacedName,
    pub dart_name: Option<String>,
    pub id: Option<i32>,
    pub inputs: Vec<IrFuncInput>,
    pub output: IrFuncOutput,
    pub owner: IrFuncOwnerInfo,
    pub mode: IrFuncMode,
    pub stream_dart_await: bool,
    pub rust_async: bool,
    pub initializer: bool,
    pub accessor: Option<IrFuncAccessorMode>,
    pub comments: Vec<IrComment>,
    pub codec_mode_pack: CodecModePack,
    // Currently, we use serde only for tests. Since lineno can be unstable, we skip this field for comparison
    #[serde(skip_serializing)]
    pub src_lineno: usize,
}

pub struct IrFuncInput {
    pub ownership_mode: Option<OwnershipMode>,
    pub inner: IrField,
}

pub struct IrFuncOutput {
    pub normal: IrType,
    pub error: Option<IrType>,
}

#[derive(Copy)]
pub enum IrFuncMode {
    Normal,
    Sync,
}

pub enum IrFuncOwnerInfo {
    Function,
    Method(IrFuncOwnerInfoMethod),
}

pub struct IrFuncOwnerInfoMethod {
    pub(crate) owner_ty: IrType,
    pub(crate) actual_method_name: String,
    pub(crate) actual_method_dart_name: Option<String>,
    pub(crate) mode: IrFuncOwnerInfoMethodMode,
}

pub enum IrFuncOwnerInfoMethodMode {
    Static,
    Instance,
}

#[derive(Copy)]
pub enum IrFuncAccessorMode {
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

impl IrFunc {
    pub(crate) fn fallible(&self) -> bool {
        self.output.error.is_some()
    }

    pub(crate) fn visit_types<F: FnMut(&IrType) -> bool>(
        &self,
        f: &mut F,
        ir_context: &impl IrContext,
    ) {
        // inputs
        for field in &self.inputs {
            field.inner.ty.visit_types(f, ir_context);
        }

        // output
        self.output.normal.visit_types(f, ir_context);
        let error_output = (self.output.error.as_ref().cloned())
            .unwrap_or(IrType::Primitive(IrTypePrimitive::Unit));
        error_output.visit_types(f, ir_context);

        // extra (#1838)
        if let IrFuncOwnerInfo::Method(IrFuncOwnerInfoMethod {
            owner_ty: enum_or_struct_ty,
            ..
        }) = &self.owner
        {
            enum_or_struct_ty.visit_types(f, ir_context);
        }
    }

    pub(crate) fn default_constructor_mode(&self) -> Option<IrFuncDefaultConstructorMode> {
        let method_info =
            if_then_some!(let IrFuncOwnerInfo::Method(info) = &self.owner , info).unwrap();
        if method_info.actual_method_name == "new" {
            if method_info.mode == IrFuncOwnerInfoMethodMode::Static
                && self.mode == IrFuncMode::Sync
            {
                Some(IrFuncDefaultConstructorMode::DartConstructor)
            } else {
                Some(IrFuncDefaultConstructorMode::StaticMethod)
            }
        } else {
            None
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

impl IrFuncOwnerInfoMethod {
    pub(crate) fn owner_ty_name(&self) -> NamespacedName {
        match &self.owner_ty {
            IrType::StructRef(ty) => ty.ident.0.clone(),
            IrType::EnumRef(ty) => ty.ident.0.clone(),
            IrType::Delegate(IrTypeDelegate::PrimitiveEnum(IrTypeDelegatePrimitiveEnum {
                ir,
                ..
            })) => ir.ident.0.clone(),
            IrType::RustAutoOpaqueImplicit(ty) => {
                NamespacedName::new(ty.self_namespace().unwrap(), ty.rust_api_type())
            }
            ty => unimplemented!("enum_or_struct_name does not know {ty:?}"),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum IrFuncDefaultConstructorMode {
    DartConstructor,
    StaticMethod,
}

impl IrFuncAccessorMode {
    pub(crate) fn verb_str(&self) -> &'static str {
        match self {
            IrFuncAccessorMode::Getter => "get",
            IrFuncAccessorMode::Setter => "set",
        }
    }
}
