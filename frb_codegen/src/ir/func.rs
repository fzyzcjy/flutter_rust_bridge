use crate::consts::*;
use crate::{ir::*, target::Target};

#[derive(Debug, Clone)]
pub struct IrFunc {
    pub name: String,
    pub inputs: Vec<IrField>,
    pub output: IrType,
    pub fallible: bool,
    pub mode: IrFuncMode,
    pub comments: Vec<IrComment>,
}

/// A stand-in for [`IrFunc`] used for output only.
#[derive(Debug, Clone)]
pub struct IrFuncDisplay {
    pub name: String,
    pub inputs: Vec<IrParam>,
    pub output: String,
    pub has_port_argument: bool,
}

#[derive(Debug, Clone)]
pub struct IrParam {
    /// Rust-style ident.
    pub name: String,
    /// Dart wire type.
    pub ty: String,
}

impl IrFuncDisplay {
    pub fn from_ir(func: &IrFunc, target: Target) -> Self {
        Self {
            name: func.wire_func_name(),
            has_port_argument: func.mode.has_port_argument(),
            inputs: (func.mode.has_port_argument())
                .then(|| IrParam {
                    name: "port_".to_owned(),
                    ty: "NativePortType".to_owned(),
                })
                .into_iter()
                .chain(func.inputs.iter().map(|input| IrParam {
                    name: input.name.rust_style().to_owned(),
                    ty: input.ty.dart_wire_type(target),
                }))
                .collect(),
            output: if func.mode.has_port_argument() {
                VOID.to_owned()
            } else {
                func.output.dart_wire_type(target)
            },
        }
    }
}

impl IrFunc {
    pub fn wire_func_name(&self) -> String {
        format!("wire_{}", self.name)
    }
}

/// Represents a function's output type
#[derive(Debug, Clone)]
pub enum IrFuncOutput {
    ResultType(IrType),
    Type(IrType),
}

/// Represents the type of an argument to a function
#[derive(Debug, Clone)]
pub enum IrFuncArg {
    StreamSinkType(IrType),
    Type(IrType),
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Eq)]
pub enum IrFuncMode {
    Normal,
    Sync,
    Stream {
        // The index of StreamSink in the function arguments
        argument_index: usize,
    },
}

impl IrFuncMode {
    #[inline]
    pub fn dart_return_type(&self, inner: &str) -> String {
        match self {
            Self::Normal => format!("Future<{}>", inner),
            Self::Sync => inner.to_string(),
            Self::Stream { .. } => format!("Stream<{}>", inner),
        }
    }

    #[inline]
    pub fn ffi_call_mode(&self) -> &'static str {
        match self {
            Self::Normal => "Normal",
            Self::Sync => "Sync",
            Self::Stream { .. } => "Stream",
        }
    }

    #[inline]
    pub fn has_port_argument(&self) -> bool {
        !matches!(self, Self::Sync)
    }

    #[inline]
    pub fn dart_port_param<'a, T: From<&'a str>>(&self) -> Option<T> {
        self.has_port_argument()
            .then(|| "NativePortType port_".into())
    }

    #[inline]
    pub fn dart_port_var(&self) -> Option<&str> {
        self.has_port_argument().then_some("port_")
    }
}
