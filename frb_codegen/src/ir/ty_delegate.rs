use crate::ir::*;

/// types that delegate to another type
#[derive(Debug, Clone)]
pub enum IrTypeDelegate {
    String,
    StringList,
    SyncReturnVecU8,
    ZeroCopyBufferVecPrimitive(IrTypePrimitive),
    PrimitiveEnum {
        ir: IrTypeEnumRef,
        /// Allows for `#[repr]`'s other than [i32]
        repr: IrTypePrimitive,
    },
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
            IrTypeDelegate::PrimitiveEnum { repr, .. } => IrType::Primitive(repr.clone()),
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
            IrTypeDelegate::PrimitiveEnum { ir, .. } => ir.safe_ident(),
        }
    }

    fn dart_api_type(&self) -> String {
        match self {
            IrTypeDelegate::String => "String".to_string(),
            IrTypeDelegate::StringList => "List<String>".to_owned(),
            IrTypeDelegate::SyncReturnVecU8 | IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                self.get_delegate().dart_api_type()
            }
            IrTypeDelegate::PrimitiveEnum { ir, .. } => ir.dart_api_type(),
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
            IrTypeDelegate::PrimitiveEnum { ir, .. } => ir.rust_api_type(),
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
