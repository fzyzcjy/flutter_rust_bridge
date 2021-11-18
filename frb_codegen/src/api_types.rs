use std::collections::{HashMap, HashSet};

use convert_case::{Case, Casing};

use ApiType::*;

pub type ApiStructPool = HashMap<String, ApiStruct>;

#[derive(Debug, Clone)]
pub struct ApiFile {
    pub funcs: Vec<ApiFunc>,
    pub struct_pool: ApiStructPool,
    pub has_executor: bool,
}

impl ApiFile {
    /// [f] returns [true] if it wants to stop going to the *children* of this subtree
    pub fn visit_types<F: FnMut(&ApiType) -> bool>(
        &self,
        f: &mut F,
        include_func_inputs: bool,
        include_func_output: bool,
    ) {
        for func in &self.funcs {
            if include_func_inputs {
                for field in &func.inputs {
                    field.ty.visit_types(f, self);
                }
            }
            if include_func_output {
                func.output.visit_types(f, self);
            }
        }
    }

    pub fn distinct_types(
        &self,
        include_func_inputs: bool,
        include_func_output: bool,
    ) -> Vec<ApiType> {
        let mut seen_idents = HashSet::new();
        let mut ans = Vec::new();
        self.visit_types(
            &mut |ty| {
                let ident = ty.safe_ident();
                let contains = seen_idents.contains(&ident);
                if !contains {
                    seen_idents.insert(ident);
                    ans.push(ty.clone());
                }
                contains
            },
            include_func_inputs,
            include_func_output,
        );

        // make the output change less when input change
        ans.sort_by_key(|ty| ty.safe_ident());

        ans
    }
}

#[derive(Debug, Clone)]
pub struct ApiFunc {
    pub name: String,
    pub inputs: Vec<ApiField>,
    pub output: ApiType,
    pub mode: ApiFuncMode,
    pub comments: Vec<Comment>,
}

impl ApiFunc {
    pub fn wire_func_name(&self) -> String {
        format!("wire_{}", self.name)
    }
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum ApiFuncMode {
    Normal,
    Sync,
    Stream,
}

impl ApiFuncMode {
    pub fn dart_return_type(&self, inner: &str) -> String {
        match self {
            Self::Normal => format!("Future<{}>", inner),
            Self::Sync => inner.to_string(),
            Self::Stream => format!("Stream<{}>", inner),
        }
    }

    pub fn ffi_call_mode(&self) -> &'static str {
        match self {
            Self::Normal => "Normal",
            Self::Sync => "Sync",
            Self::Stream => "Stream",
        }
    }

    pub fn has_port_argument(&self) -> bool {
        self != &Self::Sync
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
    Optional(ApiTypeOptional),
    GeneralList(Box<ApiTypeGeneralList>),
    StructRef(ApiTypeStructRef),
    Boxed(Box<ApiTypeBoxed>),
}

macro_rules! api_type_call_child {
    ($func:ident, $ret:ty) => {
        pub fn $func(&self) -> $ret {
            match self {
                Primitive(inner) => inner.$func(),
                Delegate(inner) => inner.$func(),
                PrimitiveList(inner) => inner.$func(),
                GeneralList(inner) => inner.$func(),
                StructRef(inner) => inner.$func(),
                Boxed(inner) => inner.$func(),
                Optional(inner) => inner.$func(),
            }
        }
    };
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
            Optional(inner) => inner.inner.visit_types(f, api_file),
            Primitive(_) => {}
        }
    }

    api_type_call_child!(safe_ident, String);
    api_type_call_child!(dart_api_type, String);
    api_type_call_child!(dart_wire_type, String);
    api_type_call_child!(rust_api_type, String);
    api_type_call_child!(rust_wire_type, String);
    api_type_call_child!(rust_wire_modifier, String);
    api_type_call_child!(rust_wire_is_pointer, bool);

    #[inline]
    pub fn required_modifier(&self) -> &'static str {
        match self {
            Optional(_) => "",
            _ => "required ",
        }
    }

    // api_fill functions target this type instead of the delegate.
    #[inline]
    pub fn optional_inner(&self) -> &ApiType {
        match self {
            Optional(inner) => &inner.inner,
            _ => self,
        }
    }

    /// Additional indirection for types put behind a vector
    #[inline]
    pub fn optional_ptr_modifier(&self) -> &'static str {
        match self {
            Optional(_) | Delegate(ApiTypeDelegate::String) => "*mut ",
            _ => "",
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
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    F32,
    F64,
    Bool,
    Unit,
}

impl ApiTypeChild for ApiTypePrimitive {
    fn safe_ident(&self) -> String {
        self.rust_api_type()
    }

    fn dart_api_type(&self) -> String {
        match self {
            ApiTypePrimitive::U8
            | ApiTypePrimitive::I8
            | ApiTypePrimitive::U16
            | ApiTypePrimitive::I16
            | ApiTypePrimitive::U32
            | ApiTypePrimitive::I32
            | ApiTypePrimitive::U64
            | ApiTypePrimitive::I64 => "int",
            ApiTypePrimitive::F32 | ApiTypePrimitive::F64 => "double",
            ApiTypePrimitive::Bool => "bool",
            ApiTypePrimitive::Unit => "void",
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
            ApiTypePrimitive::U16 => "u16",
            ApiTypePrimitive::I16 => "i16",
            ApiTypePrimitive::U32 => "u32",
            ApiTypePrimitive::I32 => "i32",
            ApiTypePrimitive::U64 => "u64",
            ApiTypePrimitive::I64 => "i64",
            ApiTypePrimitive::F32 => "f32",
            ApiTypePrimitive::F64 => "f64",
            ApiTypePrimitive::Bool => "bool",
            ApiTypePrimitive::Unit => "unit",
        }
        .to_string()
    }
}

impl ApiTypePrimitive {
    /// Representations of primitives within Dart's pointers, e.g. `ffi.Pointer<ffi.Uint8>`.
    /// This is enforced on Dart's side, and should be used instead of `dart_wire_type`
    /// whenever primitives are put behind a pointer.
    pub fn dart_native_type(&self) -> &'static str {
        match self {
            ApiTypePrimitive::U8 | ApiTypePrimitive::Bool => "ffi.Uint8",
            ApiTypePrimitive::I8 => "ffi.Int8",
            ApiTypePrimitive::U16 => "ffi.Uint16",
            ApiTypePrimitive::I16 => "ffi.Int16",
            ApiTypePrimitive::U32 => "ffi.Uint32",
            ApiTypePrimitive::I32 => "ffi.Int32",
            ApiTypePrimitive::U64 => "ffi.Uint64",
            ApiTypePrimitive::I64 => "ffi.Int64",
            ApiTypePrimitive::F32 => "ffi.Float",
            ApiTypePrimitive::F64 => "ffi.Double",
            ApiTypePrimitive::Unit => "ffi.Void",
        }
    }
    pub fn try_from_rust_str(s: &str) -> Option<Self> {
        match s {
            "u8" => Some(ApiTypePrimitive::U8),
            "i8" => Some(ApiTypePrimitive::I8),
            "u16" => Some(ApiTypePrimitive::U16),
            "i16" => Some(ApiTypePrimitive::I16),
            "u32" => Some(ApiTypePrimitive::U32),
            "i32" => Some(ApiTypePrimitive::I32),
            "u64" => Some(ApiTypePrimitive::U64),
            "i64" => Some(ApiTypePrimitive::I64),
            "f32" => Some(ApiTypePrimitive::F32),
            "f64" => Some(ApiTypePrimitive::F64),
            "bool" => Some(ApiTypePrimitive::Bool),
            "()" => Some(ApiTypePrimitive::Unit),
            _ => None,
        }
    }
}

// types that delegate to another type
#[derive(Debug, Clone)]
pub enum ApiTypeDelegate {
    String,
    StringList,
    SyncReturnVecU8,
    ZeroCopyBufferVecPrimitive(ApiTypePrimitive),
}

impl ApiTypeDelegate {
    pub fn get_delegate(&self) -> ApiType {
        match self {
            ApiTypeDelegate::String => ApiType::PrimitiveList(ApiTypePrimitiveList {
                primitive: ApiTypePrimitive::U8,
            }),
            ApiTypeDelegate::SyncReturnVecU8 => ApiType::PrimitiveList(ApiTypePrimitiveList {
                primitive: ApiTypePrimitive::U8,
            }),
            ApiTypeDelegate::ZeroCopyBufferVecPrimitive(primitive) => {
                ApiType::PrimitiveList(ApiTypePrimitiveList {
                    primitive: primitive.clone(),
                })
            }
            ApiTypeDelegate::StringList => ApiType::Delegate(ApiTypeDelegate::String),
        }
    }
}

impl ApiTypeChild for ApiTypeDelegate {
    fn safe_ident(&self) -> String {
        match self {
            ApiTypeDelegate::String => "String".to_owned(),
            ApiTypeDelegate::StringList => "StringList".to_owned(),
            ApiTypeDelegate::SyncReturnVecU8 => "SyncReturnVecU8".to_owned(),
            ApiTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                "ZeroCopyBuffer_".to_owned() + &self.get_delegate().dart_api_type()
            }
        }
    }

    fn dart_api_type(&self) -> String {
        match self {
            ApiTypeDelegate::String => "String".to_string(),
            ApiTypeDelegate::StringList => "List<String>".to_owned(),
            ApiTypeDelegate::SyncReturnVecU8 | ApiTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                self.get_delegate().dart_api_type()
            }
        }
    }

    fn dart_wire_type(&self) -> String {
        match self {
            ApiTypeDelegate::StringList => "ffi.Pointer<wire_StringList>".to_owned(),
            _ => self.get_delegate().dart_wire_type(),
        }
    }

    fn rust_api_type(&self) -> String {
        match self {
            ApiTypeDelegate::String => "String".to_owned(),
            ApiTypeDelegate::SyncReturnVecU8 => "SyncReturn<Vec<u8>>".to_string(),
            ApiTypeDelegate::StringList => "Vec<String>".to_owned(),
            ApiTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                format!("ZeroCopyBuffer<{}>", self.get_delegate().rust_api_type())
            }
        }
    }

    fn rust_wire_type(&self) -> String {
        match self {
            ApiTypeDelegate::StringList => "wire_StringList".to_owned(),
            _ => self.get_delegate().rust_wire_type(),
        }
    }

    fn rust_wire_is_pointer(&self) -> bool {
        self.get_delegate().rust_wire_is_pointer()
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
            ApiTypePrimitive::U16 => "Uint16List",
            ApiTypePrimitive::I16 => "Int16List",
            ApiTypePrimitive::U32 => "Uint32List",
            ApiTypePrimitive::I32 => "Int32List",
            ApiTypePrimitive::U64 => "Uint64List",
            ApiTypePrimitive::I64 => "Int64List",
            ApiTypePrimitive::F32 => "Float32List",
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
    pub comments: Vec<Comment>,
}

#[derive(Debug, Clone)]
pub struct ApiField {
    pub ty: ApiType,
    pub name: ApiIdent,
    pub comments: Vec<Comment>,
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
        let wire_type = if let Primitive(prim) = &self.inner {
            prim.dart_native_type().to_owned()
        } else {
            self.inner.dart_wire_type()
        };
        format!("ffi.Pointer<{}>", wire_type)
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

#[derive(Debug, Clone)]
pub struct ApiTypeOptional {
    pub inner: Box<ApiType>,
}

impl ApiTypeOptional {
    pub fn new_prim(prim: ApiTypePrimitive) -> Self {
        Self {
            inner: Box::new(Boxed(Box::new(ApiTypeBoxed {
                inner: Primitive(prim),
                exist_in_real_api: false,
            }))),
        }
    }

    pub fn new_ptr(ptr: ApiType) -> Self {
        Self {
            inner: Box::new(ptr),
        }
    }

    pub fn is_primitive(&self) -> bool {
        matches!(&*self.inner, Boxed(boxed) if matches!(boxed.inner, ApiType::Primitive(_)))
    }

    pub fn is_list(&self) -> bool {
        matches!(&*self.inner, GeneralList(_) | PrimitiveList(_))
    }

    pub fn is_delegate(&self) -> bool {
        matches!(&*self.inner, Delegate(_))
    }

    pub fn needs_initialization(&self) -> bool {
        !(self.is_primitive() || self.is_delegate())
    }
}

impl ApiTypeChild for ApiTypeOptional {
    fn safe_ident(&self) -> String {
        format!("opt_{}", self.inner.safe_ident())
    }
    fn rust_wire_type(&self) -> String {
        self.inner.rust_wire_type()
    }
    fn rust_api_type(&self) -> String {
        format!("Option<{}>", self.inner.rust_api_type())
    }
    fn dart_wire_type(&self) -> String {
        self.inner.dart_wire_type()
    }
    fn dart_api_type(&self) -> String {
        format!("{}?", self.inner.dart_api_type())
    }
    fn rust_wire_is_pointer(&self) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct Comment(String);

impl Comment {
    pub fn comment(&self) -> &str {
        &self.0
    }
}

impl From<&str> for Comment {
    fn from(input: &str) -> Self {
        if input.contains('\n') {
            // Dart's formatter has issues with block comments
            // so we convert them ahead of time.
            let formatted = input
                .split('\n')
                .into_iter()
                .map(|e| format!("///{}", e))
                .collect::<Vec<_>>()
                .join("\n");
            Self(formatted)
        } else {
            Self(format!("///{}", input))
        }
    }
}
