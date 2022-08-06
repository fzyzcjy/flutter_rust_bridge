use crate::ir::*;

/// types that delegate to another type
#[derive(Debug, Clone)]
pub enum IrTypeDelegate {
    String,
    StringList,
    SyncReturnVecU8,
    ZeroCopyBufferVecPrimitive(IrTypePrimitive),
    ArrayPrimitive(usize, IrTypePrimitive),
    ArrayGeneral(usize, Box<IrType>),
}

impl IrTypeDelegate {
    pub fn get_delegate(&self) -> IrType {
        match self {
            IrTypeDelegate::String => IrType::PrimitiveList(IrTypePrimitiveList {
                primitive: IrTypePrimitive::U8,
            }),
            IrTypeDelegate::SyncReturnVecU8 => IrType::PrimitiveList(IrTypePrimitiveList {
                primitive: IrTypePrimitive::U8,
            }),
            IrTypeDelegate::ZeroCopyBufferVecPrimitive(primitive) => {
                IrType::PrimitiveList(IrTypePrimitiveList {
                    primitive: primitive.clone(),
                })
            }
            IrTypeDelegate::StringList => IrType::Delegate(IrTypeDelegate::String),
            IrTypeDelegate::ArrayGeneral(_, inner) => IrType::GeneralList(IrTypeGeneralList {
                inner: inner.clone(),
            }),
            IrTypeDelegate::ArrayPrimitive(_, primitive) => {
                IrType::PrimitiveList(IrTypePrimitiveList {
                    primitive: primitive.clone(),
                })
            }
        }
    }
}

impl IrTypeTrait for IrTypeDelegate {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_file: &IrFile) {
        self.get_delegate().visit_types(f, ir_file);
    }

    fn safe_ident(&self) -> String {
        match self {
            IrTypeDelegate::String => "String".to_owned(),
            IrTypeDelegate::StringList => "StringList".to_owned(),
            IrTypeDelegate::SyncReturnVecU8 => "SyncReturnVecU8".to_owned(),
            IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                "ZeroCopyBuffer_".to_owned() + &self.get_delegate().dart_api_type()
            }
            IrTypeDelegate::ArrayGeneral(len, _) | IrTypeDelegate::ArrayPrimitive(len, _) => {
                format!("Array{len}_") + &self.get_delegate().dart_api_type()
            }
        }
    }

    fn dart_api_type(&self) -> String {
        match self {
            IrTypeDelegate::String => "String".to_string(),
            IrTypeDelegate::StringList => "List<String>".to_owned(),
            IrTypeDelegate::SyncReturnVecU8
            | IrTypeDelegate::ZeroCopyBufferVecPrimitive(_)
            | IrTypeDelegate::ArrayGeneral(_, _)
            | IrTypeDelegate::ArrayPrimitive(_, _) => self.get_delegate().dart_api_type(),
        }
    }

    fn dart_wire_type(&self) -> String {
        match self {
            IrTypeDelegate::StringList => "ffi.Pointer<wire_StringList>".to_owned(),
            _ => self.get_delegate().dart_wire_type(),
        }
    }

    fn rust_api_type(&self) -> String {
        match self {
            IrTypeDelegate::String => "String".to_owned(),
            IrTypeDelegate::SyncReturnVecU8 => "SyncReturn<Vec<u8>>".to_string(),
            IrTypeDelegate::StringList => "Vec<String>".to_owned(),
            IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                format!("ZeroCopyBuffer<{}>", self.get_delegate().rust_api_type())
            }
            IrTypeDelegate::ArrayGeneral(len, inner) => {
                format!("[{};{len}]", inner.rust_api_type())
            }
            IrTypeDelegate::ArrayPrimitive(len, inner) => {
                format!("[{};{len}]", inner.rust_api_type())
            }
        }
    }

    fn rust_wire_type(&self) -> String {
        match self {
            IrTypeDelegate::StringList => "wire_StringList".to_owned(),
            _ => self.get_delegate().rust_wire_type(),
        }
    }

    fn rust_wire_is_pointer(&self) -> bool {
        self.get_delegate().rust_wire_is_pointer()
    }
}
