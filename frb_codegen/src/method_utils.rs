use crate::ir::IrType::{self, StructRef};
use crate::ir::{IrField, IrTypeStructRef};
use crate::ir::{IrFile, IrFunc};
use convert_case::{Case, Casing};

const STATIC_METHOD_MARKER: &str = "__static_method___";
const METHOD_MARKER: &str = "__method___";

pub(crate) struct MethodNamingUtil;

impl MethodNamingUtil {
    // Is the function name for a non static method?
    pub fn is_non_static_method(s: &str) -> bool {
        s.contains(METHOD_MARKER)
    }

    // Is the function name for a static method?
    pub fn is_static_method(s: &str) -> bool {
        s.contains(STATIC_METHOD_MARKER)
    }

    // Tests if the function is `f` is a static method for struct with name `struct_name`
    pub fn is_static_method_for_struct(f: &&IrFunc, struct_name: &str) -> bool {
        f.name.contains(STATIC_METHOD_MARKER)
            && f.name.split(STATIC_METHOD_MARKER).last().unwrap() == struct_name
    }

    //Does `ir_file` has any methods directed for `struct_name`?
    pub fn has_methods(struct_name: &String, ir_file: &IrFile) -> bool {
        ir_file.funcs.iter().any(|f| {
            Self::is_method_for_struct(&f, struct_name)
                || Self::is_static_method_for_struct(&f, struct_name)
        })
    }

    // Tests if the function in `f` is a method for struct with name `struct_name`
    pub fn is_method_for_struct(f: &&IrFunc, struct_name: &String) -> bool {
        f.name.contains(METHOD_MARKER) && f.name.split(METHOD_MARKER).last().unwrap() == struct_name
    }

    // Returns the name of the struct that this method is for
    pub fn static_method_return_struct_name(s: &str) -> String {
        s.split(STATIC_METHOD_MARKER).last().unwrap().to_string()
    }

    // Returns the name of method itself
    pub fn static_method_return_method_name(s: &str) -> String {
        s.split(STATIC_METHOD_MARKER).next().unwrap().to_string()
    }

    // Returns the name of the struct that this method is for
    pub fn non_static_method_return_struct_name(s: &str) -> String {
        s.split(METHOD_MARKER).last().unwrap().to_string()
    }

    // Returns the name of method itself
    pub fn non_static_method_return_method_name(s: &str) -> String {
        s.split(METHOD_MARKER).next().unwrap().to_string()
    }

    // Tests if a given struct has methods, that is, if the `ir_file` contains
    // a function that receives the struct as first argument
    pub fn struct_has_methods(file: &IrFile, the_struct: Option<&IrType>) -> bool {
        let the_struct = if let Some(s) = the_struct {
            s
        } else {
            return false;
        };
        let struct_name = if let StructRef(IrTypeStructRef { name, freezed: _ }) = the_struct {
            name
        } else {
            return false;
        };

        file.funcs.iter().any(|f| {
            if let Some(IrField { name, .. }) = f.inputs.get(0) {
                name.raw.to_case(Case::UpperCamel) == *struct_name
            } else {
                false
            }
        })
    }

    fn mark_as_static_method(s: &str, struct_name: &str) -> String {
        format!("{}{}{}", s, STATIC_METHOD_MARKER, struct_name)
    }

    fn mark_as_non_static_method(s: &str, struct_name: &str) -> String {
        format!("{}{}{}", s, METHOD_MARKER, struct_name)
    }
}

#[derive(Debug)]
pub enum MethodInfo {
    Not,
    Static { struct_name: String },
    NonStatic { struct_name: String },
}

#[derive(Debug)]
pub struct FunctionName {
    actual_name: String,
    method_info: MethodInfo,
}

impl FunctionName {
    pub fn new(name: &str, method_info: MethodInfo) -> FunctionName {
        FunctionName {
            actual_name: name.to_string(),
            method_info,
        }
    }
    // assemble it into a string, e.g. `method_name__static_method___TheStructName` etc
    pub fn serialize(&self) -> String {
        match &self.method_info {
            MethodInfo::Not => self.actual_name.clone(),
            MethodInfo::Static { struct_name } => {
                MethodNamingUtil::mark_as_static_method(&self.actual_name, struct_name)
            }
            MethodInfo::NonStatic { struct_name } => {
                MethodNamingUtil::mark_as_non_static_method(&self.actual_name, struct_name)
            }
        }
    }

    pub fn deserialize(s: &str) -> Self {
        let actual_name: String;
        let method_info = {
            if MethodNamingUtil::is_static_method(s) {
                actual_name = MethodNamingUtil::static_method_return_method_name(s);
                MethodInfo::Static {
                    struct_name: MethodNamingUtil::static_method_return_struct_name(s),
                }
            } else if MethodNamingUtil::is_non_static_method(s) {
                actual_name = MethodNamingUtil::non_static_method_return_method_name(s);
                MethodInfo::NonStatic {
                    struct_name: MethodNamingUtil::non_static_method_return_struct_name(s),
                }
            } else {
                actual_name = s.to_string();
                MethodInfo::Not
            }
        };
        FunctionName {
            actual_name,
            method_info,
        }
    }

    pub fn method_name(&self) -> String {
        self.actual_name.clone()
    }

    pub fn struct_name(&self) -> Option<String> {
        match &self.method_info {
            MethodInfo::Not => None,
            MethodInfo::Static { struct_name } => Some(struct_name.clone()),
            MethodInfo::NonStatic { struct_name } => Some(struct_name.clone()),
        }
    }

    pub fn is_static_method(&self) -> bool {
        matches!(
            self,
            FunctionName {
                actual_name: _,
                method_info: MethodInfo::Static { .. }
            }
        )
    }

    pub fn is_non_static_method(&self) -> bool {
        matches!(
            self,
            FunctionName {
                actual_name: _,
                method_info: MethodInfo::NonStatic { .. }
            }
        )
    }
}
