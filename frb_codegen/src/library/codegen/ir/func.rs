use crate::codegen::generator::codec::structs::CodecModePack;
use crate::codegen::ir::comment::IrComment;
use crate::codegen::ir::field::IrField;
use crate::codegen::ir::namespace::NamespacedName;
use crate::codegen::ir::ty::{IrContext, IrType};
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::if_then_some;

crate::ir! {
pub struct IrFunc {
    pub name: NamespacedName,
    pub id: i32,
    pub inputs: Vec<IrField>,
    pub output: IrType,
    pub error_output: Option<IrType>,
    pub owner: IrFuncOwnerInfo,
    pub mode: IrFuncMode,
    pub rust_async: bool,
    pub initializer: bool,
    // When later we support setter, etc, we should refactor it into an enum
    pub getter: bool,
    pub comments: Vec<IrComment>,
    pub codec_mode_pack: CodecModePack,
    // Currently, we use serde only for tests. Since lineno can be unstable, we skip this field for comparison
    #[serde(skip_serializing)]
    pub src_lineno: usize,
}

#[derive(Copy)]
pub enum IrFuncMode {
    Normal,
    Sync,
    Stream {
        // The index of StreamSink in the function arguments
        argument_index: usize,
    },
}

pub enum IrFuncOwnerInfo {
    Function,
    Method(IrFuncOwnerInfoMethod),
}

pub struct IrFuncOwnerInfoMethod {
    pub(crate) enum_or_struct_ty: IrType,
    pub(crate) actual_method_name: String,
    pub(crate) mode: IrFuncOwnerInfoMethodMode,
}

pub enum IrFuncOwnerInfoMethodMode {
    Static,
    Instance,
}
}

impl IrFunc {
    pub(crate) fn fallible(&self) -> bool {
        self.error_output.is_some()
    }

    pub(crate) fn visit_types<F: FnMut(&IrType) -> bool>(
        &self,
        f: &mut F,
        ir_context: &impl IrContext,
    ) {
        // inputs
        for field in &self.inputs {
            field.ty.visit_types(f, ir_context);
        }

        // output
        self.output.visit_types(f, ir_context);
        let error_output = (self.error_output.as_ref().cloned())
            .unwrap_or(IrType::Primitive(IrTypePrimitive::Unit));
        error_output.visit_types(f, ir_context);

        // extra (#1838)
        if let IrFuncOwnerInfo::Method(IrFuncOwnerInfoMethod {
            enum_or_struct_ty, ..
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
}

impl IrFuncOwnerInfoMethod {
    pub(crate) fn enum_or_struct_name(&self) -> NamespacedName {
        match &self.enum_or_struct_ty {
            IrType::StructRef(ty) => ty.ident.0.clone(),
            IrType::EnumRef(ty) => ty.ident.0.clone(),
            IrType::RustAutoOpaque(ty) => TODO,
            ty => unimplemented!("enum_or_struct_name does not know {ty:?}"),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) enum IrFuncDefaultConstructorMode {
    DartConstructor,
    StaticMethod,
}
