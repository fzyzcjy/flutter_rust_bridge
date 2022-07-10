use crate::ir::IrType::{self, Boxed, StructRef};
use crate::ir::{IrField, IrTypeStructRef};
use crate::ir::{IrFile, IrFunc, IrTypeBoxed};
use convert_case::{Case, Casing};

const STATIC_METHOD_MARKER: &str = "__static_method___";
const METHOD_MARKER: &str = "__method";

pub fn is_non_static_method(s: &str) -> bool {
    s.ends_with(METHOD_MARKER)
}

pub fn is_static_method(s: &str) -> bool {
    s.contains(STATIC_METHOD_MARKER)
}

// Tests if the function is `f` is a static method for struct with name `struct_name`
pub fn is_static_method_for_struct(f: &&IrFunc, struct_name: &str) -> bool {
    f.name.contains(STATIC_METHOD_MARKER)
        && f.name.split(STATIC_METHOD_MARKER).last().unwrap() == struct_name
}

pub fn has_methods(struct_name: &String, ir_file: &IrFile) -> bool {
    ir_file.funcs.iter().any(|f| {
        is_method_for_struct(&f, struct_name) || is_static_method_for_struct(&f, struct_name)
    })
}

// Tests if the function in `f` is a method for struct with name `struct_name`
pub fn is_method_for_struct(f: &&IrFunc, struct_name: &String) -> bool {
    f.name.ends_with(METHOD_MARKER)
        && if let Boxed(IrTypeBoxed {
            exist_in_real_api: _,
            inner,
        }) = &f.inputs[0].ty
        {
            if let StructRef(IrTypeStructRef { name, freezed: _ }) = &**inner {
                *name == *struct_name
            } else {
                false
            }
        } else {
            false
        }
}

// Returns the name of the struct that this method is for
pub fn static_method_return_struct_name(s: &str) -> String {
    s.split(STATIC_METHOD_MARKER).last().unwrap().to_string()
}

// Returns the name of method itself
pub fn static_method_return_method_name(s: &str) -> String {
    s.split(STATIC_METHOD_MARKER).next().unwrap().to_string()
}

// Clears the method marker from this method name
pub fn clear_method_marker(s: &str) -> String {
    s.replace(METHOD_MARKER, "")
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
        if let Some(IrField {
            ty: _,
            name,
            is_final: _,
            comments: _,
        }) = f.inputs.get(0)
        {
            name.raw.to_case(Case::UpperCamel) == *struct_name
        } else {
            false
        }
    })
}

pub fn mark_as_static_method(s: &str, struct_name: &str) -> String {
    format!("{}{}{}", s, STATIC_METHOD_MARKER, struct_name)
}

pub fn mark_as_non_static_method(s: &str) -> String {
    format!("{}{}", s, METHOD_MARKER)
}

//tests if a given `func` is a method, and also returns the struct name that it is a method for
pub fn is_method_return_struct_name(func: &IrFunc) -> (bool, Option<String>) {
    if is_non_static_method(&func.name) {
        let input = func.inputs[0].clone();
        (true, Some(input.name.to_string().to_case(Case::UpperCamel)))
    } else if is_static_method(&func.name) {
        (
            true,
            Some(static_method_return_struct_name(&func.name).to_case(Case::UpperCamel)),
        )
    } else {
        (false, None)
    }
}
