use crate::{generator::dart::*, Opts};
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait TypeDartGeneratorTrait {
    fn api2wire_body(
        &self,
        shared_dart_api2wire_funcs: &Option<Acc<String>>,
    ) -> Acc<Option<String>>;

    fn api_fill_to_wire_body(
        &self,
        _shared_dart_api2wire_funcs: &Option<Acc<String>>,
    ) -> Option<String> {
        None
    }

    fn wire2api_body(&self) -> String {
        "".to_string()
    }

    fn structs(&self) -> String {
        "".to_string()
    }
}

#[derive(Debug, Clone)]
pub struct TypeGeneratorContext<'a> {
    pub ir_file: &'a IrFile,
    pub config: &'a Opts,
}

#[macro_export]
macro_rules! type_dart_generator_struct {
    ($cls:ident, $ir_cls:ty) => {
        #[derive(Debug, Clone)]
        pub struct $cls<'a> {
            pub ir: $ir_cls,
            pub context: $crate::generator::dart::ty::TypeGeneratorContext<'a>,
        }

        impl $cls<'_> {
            #[allow(unused)]
            fn get_context(&self) -> &TypeGeneratorContext {
                &self.context
            }
            #[allow(unused)]
            /// this method only cares if a specifc type is shared or not, but not
            /// care about the type of the shared mode.
            fn is_type_shared(&self, ty: &$crate::ir::IrType) -> bool {
                match self.get_context().ir_file.is_type_shared_by_safe_ident(ty) {
                    $crate::utils::misc::ShareMode::Unique => false,
                    $crate::utils::misc::ShareMode::Shared => true,
                }
            }
            #[allow(unused)]
            fn get_private_prefix(&self) -> String {
                if self.get_context().config.shared {
                    ""
                } else {
                    "_"
                }
                .into()
            }
        }
    };
}

#[enum_dispatch(TypeDartGeneratorTrait)]
#[derive(Debug, Clone)]
pub enum TypeDartGenerator<'a> {
    Primitive(TypePrimitiveGenerator<'a>),
    Delegate(TypeDelegateGenerator<'a>),
    PrimitiveList(TypePrimitiveListGenerator<'a>),
    Optional(TypeOptionalGenerator<'a>),
    GeneralList(TypeGeneralListGenerator<'a>),
    StructRef(TypeStructRefGenerator<'a>),
    Boxed(TypeBoxedGenerator<'a>),
    EnumRef(TypeEnumRefGenerator<'a>),
    SyncReturn(TypeSyncReturnGenerator<'a>),
    DartOpaque(TypeDartOpaqueGenerator<'a>),
    RustOpaque(TypeRustOpaqueGenerator<'a>),
    Dynamic(TypeDynamicGenerator<'a>),
    Record(TypeRecordGenerator<'a>),
}

impl<'a> TypeDartGenerator<'a> {
    pub fn new(ty: IrType, ir_file: &'a IrFile, config: &'a Opts) -> Self {
        let context = TypeGeneratorContext { ir_file, config };
        match ty {
            Primitive(ir) => TypePrimitiveGenerator { ir, context }.into(),
            Delegate(ir) => TypeDelegateGenerator { ir, context }.into(),
            PrimitiveList(ir) => TypePrimitiveListGenerator { ir, context }.into(),
            Optional(ir) => TypeOptionalGenerator { ir, context }.into(),
            GeneralList(ir) => TypeGeneralListGenerator { ir, context }.into(),
            StructRef(ir) => TypeStructRefGenerator { ir, context }.into(),
            Boxed(ir) => TypeBoxedGenerator { ir, context }.into(),
            EnumRef(ir) => TypeEnumRefGenerator { ir, context }.into(),
            SyncReturn(ir) => TypeSyncReturnGenerator::new(ir, context).into(),
            DartOpaque(ir) => TypeDartOpaqueGenerator { ir, context }.into(),
            RustOpaque(ir) => TypeRustOpaqueGenerator { ir, context }.into(),
            Dynamic(ir) => TypeDynamicGenerator { ir, context }.into(),
            Record(ir) => TypeRecordGenerator { ir, context }.into(),
            Unencodable(IrTypeUnencodable { string, .. }) => {
                panic!("Cannot generate Dart code for {}", string)
            }
        }
    }
}
