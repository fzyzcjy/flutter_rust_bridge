use crate::ir::*;

/// types that delegate to another type
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
