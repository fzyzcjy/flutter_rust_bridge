use std::collections::HashSet;

use crate::utils::consts::*;

use crate::utils::misc::IrTypeUseRange;
use crate::{ir::*, target::Target};

// The struct `IrFunc` represents an Api in the IR.
// Semantically, it can be either a rust function or method,
// as long as it can be called from Dart.
crate::ir! {
pub struct IrFunc {
    pub name: String,
    pub inputs: Vec<IrField>,
    pub output: IrType,
    pub error_output: Option<IrType>,
    pub fallible: bool,
    pub mode: IrFuncMode,
    pub comments: Vec<IrComment>,
    // TODO: delete pub shared: bool,// `true` if it is a method of a shared type; otherwise `false`
}
}
impl IrFunc {
    pub fn refine(&mut self) {
        crate::utils::misc::check_func_dart_keywords(&self.name).unwrap();
        crate::transformer::transform(self);
    }
    pub fn get_type_use_range(&self, ir_type: &IrType) -> HashSet<IrTypeUseRange> {
        let mut type_use_range = HashSet::new();
        for field in self.inputs.iter() {
            if &field.ty == ir_type {
                type_use_range.insert(IrTypeUseRange::Input);
            }
        }
        if &self.output == ir_type {
            type_use_range.insert(IrTypeUseRange::Output);
        }
        if let Some(error_output) = &self.error_output {
            if error_output == ir_type {
                type_use_range.insert(IrTypeUseRange::Output);
            }
        }
        type_use_range
    }
    /// Return all distinct types
    /// (including sub field types of the parent struct/enum types or inner type(s) of a type  if there are)
    /// used in this func instance
    pub fn get_types_used(
        &self,
        include_func_inputs: bool,
        include_func_output: bool,
        diff_by_safe_ident: bool,
        ir_file: &IrFile,
    ) -> Vec<IrType> {
        assert!(include_func_inputs || include_func_output);
        let mut seen = HashSet::new();
        let mut seen_idents = HashSet::new();
        let mut ans = Vec::new();
        self.visit_types_recursively_used_in_the_func(
            &mut |ty| {
                if diff_by_safe_ident {
                    let ident = ty.safe_ident();
                    let contains = seen_idents.contains(&ident);
                    if !contains {
                        seen_idents.insert(ident);
                        ans.push(ty.clone());
                    }
                    contains
                } else {
                    let contains = seen.contains(ty);
                    if !contains {
                        seen.insert(ty.clone());
                        ans.push(ty.clone());
                    }
                    contains
                }
            },
            include_func_inputs,
            include_func_output,
            ir_file,
        );
        ans.into_iter().collect()
    }
    /// [f] returns [true] if it wants to stop going to the *children* of this subtree.
    /// That is, if a type A contains fields, the fields type would also be visited.
    pub fn visit_types_recursively_used_in_the_func<F: FnMut(&IrType) -> bool>(
        &self,
        f: &mut F,
        include_func_inputs: bool,
        include_func_output: bool,
        ir_file: &IrFile,
    ) {
        if include_func_inputs {
            for field in &self.inputs {
                field.ty.visit_self_types_recursively(f, ir_file);
            }
        }
        if include_func_output {
            self.output.visit_self_types_recursively(f, ir_file);
            if let Some(error_output) = &self.error_output {
                error_output.visit_self_types_recursively(f, ir_file);
            }
        }
    }
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
    ResultType { ok: IrType, error: Option<IrType> },
    Type(IrType),
}

/// Represents the type of an argument to a function
#[derive(Debug, Clone)]
pub enum IrFuncArg {
    StreamSinkType(IrType),
    Type(IrType),
}

crate::ir! {
pub enum IrFuncMode {
    Normal,
    Sync,
    Stream {
        // The index of StreamSink in the function arguments
        argument_index: usize,
    },
}
}

impl IrFuncMode {
    #[inline]
    pub fn dart_return_type(&self, inner: &str) -> String {
        match self {
            Self::Normal => format!("Future<{inner}>"),
            Self::Sync => inner.to_string(),
            Self::Stream { .. } => format!("Stream<{inner}>"),
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
