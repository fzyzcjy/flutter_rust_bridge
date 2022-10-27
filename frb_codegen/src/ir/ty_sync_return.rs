use crate::{ir::*, target::Target};

/// Types that have synchronized return
/// NOTE for maintainer: Please make sure all the types here
/// can be parsed by `executeSync` function in basic.dart.
#[derive(Debug, Clone)]
pub enum IrTypeSyncReturn {
    Option(Box<IrTypeSyncReturn>),
    Primitive(IrTypePrimitive),
    String,
    VecU8,
}

impl IrTypeTrait for IrTypeSyncReturn {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_file: &IrFile) {
        self.get_inner().visit_types(f, ir_file)
    }

    fn safe_ident(&self) -> String {
        match self {
            IrTypeSyncReturn::Primitive(_) => {
                // We use Rust API type here because some primitive types in Dart share the same API type.
                "SyncReturn_".to_owned() + &self.get_inner().rust_api_type()
            }
            IrTypeSyncReturn::Option(_) => {
                // We use Rust API type here because some primitive types in Dart share the same API type.
                let mut temp = self.get_inner().dart_api_type();
                temp.pop();
                "SyncReturn_Option_".to_owned() + &temp
            }
            _ => "SyncReturn_".to_owned() + &self.get_inner().dart_api_type(),
        }
    }

    fn dart_api_type(&self) -> String {
        self.get_inner().dart_api_type()
    }

    fn dart_wire_type(&self, target: Target) -> String {
        self.get_inner().dart_wire_type(target)
    }

    fn rust_api_type(&self) -> String {
        format!("SyncReturn<{}>", self.get_inner().rust_api_type())
    }

    fn rust_wire_type(&self, _: Target) -> String {
        unimplemented!("SyncReturn: rust_wire_type is not supported")
    }

    fn rust_wire_is_pointer(&self, target: Target) -> bool {
        self.get_inner().rust_wire_is_pointer(target)
    }

    fn dart_param_type(&self) -> &'static str {
        "dynamic"
    }
}

impl IrTypeSyncReturn {
    pub fn get_inner(&self) -> IrType {
        match self {
            IrTypeSyncReturn::Primitive(primitive) => IrType::Primitive(primitive.clone()),
            IrTypeSyncReturn::String => IrType::Delegate(IrTypeDelegate::String),
            IrTypeSyncReturn::VecU8 => IrType::PrimitiveList(IrTypePrimitiveList {
                primitive: IrTypePrimitive::U8,
            }),
            IrTypeSyncReturn::Option(o) => IrType::Optional(IrTypeOptional {
                inner: Box::new(o.get_inner()),
            }),
        }
    }
}
