use convert_case::{Case, Casing};
use std::collections::{HashMap, HashSet};
use ApiType::*;

pub type ApiStructPool = HashMap<String, ApiStruct>;

#[derive(Debug, Clone)]
pub struct ApiFile {
    pub funcs: Vec<ApiFunc>,
    pub struct_pool: ApiStructPool,
}

impl ApiFile {
    /// [f] returns [true] if it wants to stop going to the *children* of this subtree
    pub fn visit_types<F: FnMut(&ApiType) -> bool>(&self, f: &mut F) {
        for func in &self.funcs {
            for field in &func.inputs {
                field.ty.visit_types(f, self);
            }
            func.output.visit_types(f, self);
        }
    }

    pub fn distinct_types(&self) -> Vec<ApiType> {
        let mut seen_idents = HashSet::new();
        let mut ans = Vec::new();
        self.visit_types(&mut |ty| {
            let ident = ty.safe_ident();
            let contains = seen_idents.contains(&ident);
            if !contains {
                seen_idents.insert(ident);
                ans.push(ty.clone());
            }
            contains
        });
        ans
    }
}

#[derive(Debug, Clone)]
pub struct ApiFunc {
    pub name: String,
    pub inputs: Vec<ApiField>,
    pub output: ApiType,
}

impl ApiFunc {
    pub fn wire_func_name(&self) -> String {
        format!("wire_{}", self.name)
    }
}

#[derive(Debug, Clone)]
pub struct ApiIdent {
    pub raw: String,
}

impl ApiIdent {
    pub fn new(raw: String) -> ApiIdent {
        ApiIdent { raw }
    }

    pub fn rust_style(&self) -> &str {
        &self.raw
    }

    pub fn dart_style(&self) -> String {
        self.raw.to_case(Case::Camel)
    }
}

#[derive(Debug, Clone)]
pub enum ApiType {
    Primitive(ApiTypePrimitive),
    Delegate(ApiTypeDelegate),
    PrimitiveList(ApiTypePrimitiveList),
    GeneralList(Box<ApiTypeGeneralList>),
    StructRef(ApiTypeStructRef),
    Boxed(Box<ApiTypeBoxed>),
}

impl ApiType {
    pub fn visit_types<F: FnMut(&ApiType) -> bool>(&self, f: &mut F, api_file: &ApiFile) {
        if f(self) {
            return;
        }

        match &self {
            PrimitiveList(inner) => {
                f(&ApiType::Primitive(inner.primitive.clone()));
            }
            GeneralList(inner) => inner.inner.visit_types(f, api_file),
            StructRef(struct_ref) => {
                for field in &struct_ref.get(api_file).fields {
                    field.ty.visit_types(f, api_file);
                }
            }
            Boxed(inner) => inner.inner.visit_types(f, api_file),
            Delegate(d) => d.get_delegate().visit_types(f, api_file),
            Primitive(_) => {}
        }
    }

    pub fn safe_ident(&self) -> String {
        match self {
            Primitive(inner) => inner.safe_ident(),
            Delegate(inner) => inner.safe_ident(),
            PrimitiveList(inner) => inner.safe_ident(),
            GeneralList(inner) => inner.safe_ident(),
            StructRef(inner) => inner.safe_ident(),
            Boxed(inner) => inner.safe_ident(),
        }
    }

    pub fn dart_api_type(&self) -> String {
        match self {
            Primitive(inner) => inner.dart_api_type(),
            Delegate(inner) => inner.dart_api_type(),
            PrimitiveList(inner) => inner.dart_api_type(),
            GeneralList(inner) => inner.dart_api_type(),
            StructRef(inner) => inner.dart_api_type(),
            Boxed(inner) => inner.dart_api_type(),
        }
    }
    pub fn dart_wire_type(&self) -> String {
        match self {
            Primitive(inner) => inner.dart_wire_type(),
            Delegate(inner) => inner.dart_wire_type(),
            PrimitiveList(inner) => inner.dart_wire_type(),
            GeneralList(inner) => inner.dart_wire_type(),
            StructRef(inner) => inner.dart_wire_type(),
            Boxed(inner) => inner.dart_wire_type(),
        }
    }
    pub fn rust_api_type(&self) -> String {
        match self {
            Primitive(inner) => inner.rust_api_type(),
            Delegate(inner) => inner.rust_api_type(),
            PrimitiveList(inner) => inner.rust_api_type(),
            GeneralList(inner) => inner.rust_api_type(),
            StructRef(inner) => inner.rust_api_type(),
            Boxed(inner) => inner.rust_api_type(),
        }
    }
    pub fn rust_wire_type(&self) -> String {
        match self {
            Primitive(inner) => inner.rust_wire_type(),
            Delegate(inner) => inner.rust_wire_type(),
            PrimitiveList(inner) => inner.rust_wire_type(),
            GeneralList(inner) => inner.rust_wire_type(),
            StructRef(inner) => inner.rust_wire_type(),
            Boxed(inner) => inner.rust_wire_type(),
        }
    }
    pub fn rust_wire_modifier(&self) -> String {
        match self {
            Primitive(inner) => inner.rust_wire_modifier(),
            Delegate(inner) => inner.rust_wire_modifier(),
            PrimitiveList(inner) => inner.rust_wire_modifier(),
            GeneralList(inner) => inner.rust_wire_modifier(),
            StructRef(inner) => inner.rust_wire_modifier(),
            Boxed(inner) => inner.rust_wire_modifier(),
        }
    }
    pub fn rust_wire_is_pointer(&self) -> bool {
        match self {
            Primitive(inner) => inner.rust_wire_is_pointer(),
            Delegate(inner) => inner.rust_wire_is_pointer(),
            PrimitiveList(inner) => inner.rust_wire_is_pointer(),
            GeneralList(inner) => inner.rust_wire_is_pointer(),
            StructRef(inner) => inner.rust_wire_is_pointer(),
            Boxed(inner) => inner.rust_wire_is_pointer(),
        }
    }
}

pub trait ApiTypeChild {
    fn safe_ident(&self) -> String;

    fn dart_api_type(&self) -> String;

    fn dart_wire_type(&self) -> String;

    fn rust_api_type(&self) -> String;

    fn rust_wire_type(&self) -> String;

    fn rust_wire_modifier(&self) -> String {
        if self.rust_wire_is_pointer() {
            "*mut ".to_string()
        } else {
            "".to_string()
        }
    }

    fn rust_wire_is_pointer(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone)]
pub enum ApiTypePrimitive {
    U8,
    I8,
    I32,
    I64,
    F64,
    Bool,
}

impl ApiTypeChild for ApiTypePrimitive {
    fn safe_ident(&self) -> String {
        self.rust_api_type()
    }

    fn dart_api_type(&self) -> String {
        match self {
            ApiTypePrimitive::U8 => "int",
            ApiTypePrimitive::I8 => "int",
            ApiTypePrimitive::I32 => "int",
            ApiTypePrimitive::I64 => "int",
            ApiTypePrimitive::F64 => "double",
            ApiTypePrimitive::Bool => "bool",
        }
        .to_string()
    }

    fn dart_wire_type(&self) -> String {
        self.dart_api_type()
    }

    fn rust_api_type(&self) -> String {
        self.rust_wire_type()
    }

    fn rust_wire_type(&self) -> String {
        match self {
            ApiTypePrimitive::U8 => "u8",
            ApiTypePrimitive::I8 => "i8",
            ApiTypePrimitive::I32 => "i32",
            ApiTypePrimitive::I64 => "i64",
            ApiTypePrimitive::F64 => "f64",
            ApiTypePrimitive::Bool => "bool",
        }
        .to_string()
    }
}

impl ApiTypePrimitive {
    pub fn try_from_rust_str(s: &str) -> Option<Self> {
        match s {
            "u8" => Some(ApiTypePrimitive::U8),
            "i8" => Some(ApiTypePrimitive::I8),
            "i32" => Some(ApiTypePrimitive::I32),
            "i64" => Some(ApiTypePrimitive::I64),
            "f64" => Some(ApiTypePrimitive::F64),
            "bool" => Some(ApiTypePrimitive::Bool),
            _ => None,
        }
    }
}

// types that delegate to another type
#[derive(Debug, Clone)]
pub enum ApiTypeDelegate {
    String,
}

impl ApiTypeDelegate {
    fn get_delegate(&self) -> ApiType {
        match self {
            ApiTypeDelegate::String => ApiType::PrimitiveList(ApiTypePrimitiveList {
                primitive: ApiTypePrimitive::U8,
            }),
        }
    }
}

impl ApiTypeChild for ApiTypeDelegate {
    fn safe_ident(&self) -> String {
        self.dart_api_type().to_case(Case::Snake)
    }

    fn dart_api_type(&self) -> String {
        match self {
            ApiTypeDelegate::String => "String",
        }
        .to_string()
    }

    fn dart_wire_type(&self) -> String {
        self.get_delegate().dart_wire_type()
    }

    fn rust_api_type(&self) -> String {
        match self {
            ApiTypeDelegate::String => "String",
        }
        .to_string()
    }

    fn rust_wire_type(&self) -> String {
        self.get_delegate().rust_wire_type()
    }

    fn rust_wire_is_pointer(&self) -> bool {
        match self {
            ApiTypeDelegate::String => true,
        }
    }
}

impl ApiTypeDelegate {
    pub fn try_from_rust_str(s: &str) -> Option<Self> {
        match s {
            "String" => Some(ApiTypeDelegate::String),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ApiTypePrimitiveList {
    pub primitive: ApiTypePrimitive,
}

impl ApiTypeChild for ApiTypePrimitiveList {
    fn safe_ident(&self) -> String {
        self.dart_api_type().to_case(Case::Snake)
    }

    fn dart_api_type(&self) -> String {
        match &self.primitive {
            ApiTypePrimitive::U8 => "Uint8List",
            ApiTypePrimitive::I8 => "Int8List",
            ApiTypePrimitive::I64 => "Int64List",
            ApiTypePrimitive::F64 => "Float64List",
            _ => panic!("does not support {:?} yet", &self.primitive),
        }
        .to_string()
    }

    fn dart_wire_type(&self) -> String {
        format!("ffi.Pointer<wire_{}>", self.safe_ident())
    }

    fn rust_api_type(&self) -> String {
        format!("Vec<{}>", self.primitive.rust_api_type())
    }

    fn rust_wire_type(&self) -> String {
        format!("wire_{}", self.safe_ident())
    }

    fn rust_wire_is_pointer(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct ApiTypeGeneralList {
    pub inner: ApiType,
}

impl ApiTypeChild for ApiTypeGeneralList {
    fn safe_ident(&self) -> String {
        format!("list_{}", self.inner.safe_ident())
    }

    fn dart_api_type(&self) -> String {
        format!("List<{}>", self.inner.dart_api_type())
    }

    fn dart_wire_type(&self) -> String {
        format!("ffi.Pointer<wire_{}>", self.safe_ident())
    }

    fn rust_api_type(&self) -> String {
        format!("Vec<{}>", self.inner.rust_api_type())
    }

    fn rust_wire_type(&self) -> String {
        format!("wire_{}", self.safe_ident())
    }

    fn rust_wire_is_pointer(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct ApiTypeStructRef {
    pub name: String,
}

impl ApiTypeStructRef {
    pub fn get<'a>(&self, f: &'a ApiFile) -> &'a ApiStruct {
        &f.struct_pool[&self.name]
    }
}

impl ApiTypeChild for ApiTypeStructRef {
    fn safe_ident(&self) -> String {
        self.dart_api_type().to_case(Case::Snake)
    }

    fn dart_api_type(&self) -> String {
        self.name.to_string()
    }

    fn dart_wire_type(&self) -> String {
        self.rust_wire_type()
    }

    fn rust_api_type(&self) -> String {
        self.name.to_string()
    }

    fn rust_wire_type(&self) -> String {
        format!("wire_{}", self.name)
    }
}

#[derive(Debug, Clone)]
pub struct ApiStruct {
    pub name: String,
    pub fields: Vec<ApiField>,
    pub is_fields_named: bool,
}

#[derive(Debug, Clone)]
pub struct ApiField {
    pub ty: ApiType,
    pub name: ApiIdent,
}

impl ApiField {
    pub fn name_rust_style(&self, is_fields_named: bool) -> String {
        if is_fields_named {
            self.name.rust_style().to_string()
        } else {
            // TO DO this is so hacky...
            self.name.rust_style().replace("field", "")
        }
    }
}

#[derive(Debug, Clone)]
pub struct ApiTypeBoxed {
    /// if false, means that we automatically add it when transforming it - it does not exist in real api.
    pub exist_in_real_api: bool,
    pub inner: ApiType,
}

impl ApiTypeChild for ApiTypeBoxed {
    fn safe_ident(&self) -> String {
        format!(
            "box_{}{}",
            if self.exist_in_real_api {
                ""
            } else {
                "autoadd_"
            },
            self.inner.safe_ident()
        )
    }

    fn dart_api_type(&self) -> String {
        self.inner.dart_api_type()
    }

    fn dart_wire_type(&self) -> String {
        format!("ffi.Pointer<{}>", self.inner.dart_wire_type())
    }

    fn rust_api_type(&self) -> String {
        if self.exist_in_real_api {
            format!("Box<{}>", self.inner.rust_api_type())
        } else {
            self.inner.rust_api_type()
        }
    }

    fn rust_wire_type(&self) -> String {
        self.inner.rust_wire_type()
    }

    fn rust_wire_is_pointer(&self) -> bool {
        true
    }
}
