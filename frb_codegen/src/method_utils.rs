use crate::ir::IrFile;

const STATIC_METHOD_MARKER: &str = "__static_method__";
const METHOD_MARKER: &str = "__method__";

pub(crate) struct MethodNamingUtil;

impl MethodNamingUtil {
    // Is the function name for a non static method?
    fn is_non_static_method(s: &str) -> bool {
        s.contains(METHOD_MARKER)
    }

    // Is the function name for a static method?
    fn is_static_method(s: &str) -> bool {
        s.contains(STATIC_METHOD_MARKER)
    }

    // Returns the name of the struct that this method is for
    fn static_method_return_struct_name(s: &str) -> String {
        s.split(STATIC_METHOD_MARKER).last().unwrap().to_string()
    }

    // Returns the name of method itself
    fn static_method_return_method_name(s: &str) -> String {
        s.split(STATIC_METHOD_MARKER).next().unwrap().to_string()
    }

    // Returns the name of the struct that this method is for
    fn non_static_method_return_struct_name(s: &str) -> String {
        s.split(METHOD_MARKER).last().unwrap().to_string()
    }

    // Returns the name of method itself
    fn non_static_method_return_method_name(s: &str) -> String {
        s.split(METHOD_MARKER).next().unwrap().to_string()
    }

    fn mark_as_static_method(s: &str, struct_name: &str) -> String {
        format!("{}{}{}", s, STATIC_METHOD_MARKER, struct_name)
    }

    fn mark_as_non_static_method(s: &str, struct_name: &str) -> String {
        format!("{}{}{}", s, METHOD_MARKER, struct_name)
    }

    //Does `ir_file` has any methods directed for `struct_name`?
    pub fn has_methods(struct_name: &str, ir_file: &IrFile) -> bool {
        ir_file.funcs.iter().any(|f| {
            let f = FunctionName::deserialize(&f.name);
            f.is_method_for_struct(struct_name) || f.is_static_method_for_struct(struct_name)
        })
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
        if MethodNamingUtil::is_static_method(s) {
            FunctionName {
                actual_name: MethodNamingUtil::static_method_return_method_name(s),
                method_info: MethodInfo::Static {
                    struct_name: MethodNamingUtil::static_method_return_struct_name(s),
                },
            }
        } else if MethodNamingUtil::is_non_static_method(s) {
            FunctionName {
                actual_name: MethodNamingUtil::non_static_method_return_method_name(s),
                method_info: MethodInfo::NonStatic {
                    struct_name: MethodNamingUtil::non_static_method_return_struct_name(s),
                },
            }
        } else {
            FunctionName {
                actual_name: s.to_string(),
                method_info: MethodInfo::Not,
            }
        }
    }

    pub fn method_name(&self) -> String {
        self.actual_name.clone()
    }

    pub fn static_method_name(&self) -> Option<String> {
        match &self.method_info {
            MethodInfo::Static { .. } => Some(self.actual_name.clone()),
            _ => None,
        }
    }

    pub fn struct_name(&self) -> Option<String> {
        match &self.method_info {
            MethodInfo::Not => None,
            MethodInfo::Static { struct_name } => Some(struct_name.clone()),
            MethodInfo::NonStatic { struct_name } => Some(struct_name.clone()),
        }
    }

    pub fn is_static_method(&self) -> bool {
        matches!(self.method_info, MethodInfo::Static { .. })
    }

    // Tests if the function is `f` is a static method for struct with name `struct_name`
    pub fn is_static_method_for_struct(&self, struct_name: &str) -> bool {
        self.is_static_method() && self.struct_name().unwrap() == struct_name
    }

    // Tests if the function in `f` is a method for struct with name `struct_name`
    pub fn is_method_for_struct(&self, struct_name: &str) -> bool {
        self.is_non_static_method() && self.struct_name().unwrap() == struct_name
    }

    pub fn is_non_static_method(&self) -> bool {
        matches!(self.method_info, MethodInfo::NonStatic { .. })
    }
}
