use std::borrow::Cow;

use crate::generator::rust::*;
use crate::target::Acc;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait TypeRustGeneratorTrait {
    fn wire2api_body(&self) -> Acc<Option<String>>;

    /// Handles JsValue to Self conversions.
    fn wire2api_jsvalue(&self) -> Option<Cow<str>> {
        None
    }

    fn wire_struct_fields(&self) -> Option<Vec<String>> {
        None
    }

    fn static_checks(&self) -> Option<String> {
        None
    }

    fn wrapper_struct(&self) -> Option<String> {
        None
    }

    fn self_access(&self, obj: String) -> String {
        obj
    }

    fn convert_to_dart(&self, obj: String) -> String {
        format!("{obj}.into_into_dart().into_dart()")
    }

    fn structs(&self) -> String {
        "".to_string()
    }

    fn allocate_funcs(&self, _collector: &mut ExternFuncCollector) -> Acc<Option<String>> {
        Acc::default()
    }

    fn related_funcs(&self, _collector: &mut ExternFuncCollector) -> Acc<Option<String>> {
        Acc::default()
    }

    fn impl_intodart(&self) -> String {
        "".to_string()
    }

    fn new_with_nullptr(&self, _collector: &mut ExternFuncCollector) -> String {
        "".to_string()
    }

    fn imports(&self) -> Option<String> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct TypeGeneratorContext<'a> {
    pub type_name: String,
    pub config: &'a Opts,
    pub ir_file: &'a IrFile,
    pub all_configs: &'a AllConfigs,
}

#[macro_export]
macro_rules! type_rust_generator_struct {
    ($cls:ident, $ir_cls:ty) => {
        #[derive(Debug, Clone)]
        pub struct $cls<'a> {
            pub ir: $ir_cls,
            pub context: $crate::generator::rust::ty::TypeGeneratorContext<'a>,
        }

        impl $cls<'_> {
            #[allow(unused)]
            fn get_context(&self) -> &TypeGeneratorContext {
                &self.context
            }
            #[allow(unused)]
            /// Get the shared module name if the input type is defined there.
            /// Thus, if the type is NOT defined in a shared block, the method returns None.
            fn get_type_share_module(&self, ir_type: &$crate::ir::IrType) -> Option<String> {
                if self.context.all_configs.is_type_shared(ir_type, true){
                    let shared_module = self.context.all_configs.get_rust_module_name(None);
                    if shared_module.is_none() {
                        panic!("For instance in charge of `{}`: when checking shared module for type \"{:?}\", the type is shared indeed, but the shared module name is None",
                            self.get_context().type_name, ir_type
                        );
                    }
                    shared_module
                }else{
                    None
                }


            }
            #[allow(unused)]
            fn get_wire2api_prefix(&self, ir_type: &$crate::ir::IrType) -> String {
                let shared_mod_name = self.get_type_share_module(ir_type);
                if !self.get_context().config.shared && shared_mod_name.is_some() {
                    format!("{}::Wire2Api", shared_mod_name.unwrap())
                } else {
                    "Wire2Api".into()
                }
            }
        }
    };
}

#[enum_dispatch(TypeRustGeneratorTrait)]
#[derive(Debug, Clone)]
pub enum TypeRustGenerator<'a> {
    Primitive(TypePrimitiveGenerator<'a>),
    Delegate(TypeDelegateGenerator<'a>),
    PrimitiveList(TypePrimitiveListGenerator<'a>),
    Optional(TypeOptionalGenerator<'a>),
    OptionalList(TypeOptionalListGenerator<'a>),
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

impl<'a> TypeRustGenerator<'a> {
    pub fn new(ty: IrType, config: &'a Opts, all_configs: &'a AllConfigs) -> Self {
        let context = TypeGeneratorContext {
            type_name: format!("{ty:?}"),
            config,
            ir_file: all_configs
                .get_ir_file(config.block_index)
                .as_ref()
                .unwrap(),
            all_configs,
        };
        match ty {
            Primitive(ir) => TypePrimitiveGenerator { ir, context }.into(),
            Delegate(ir) => TypeDelegateGenerator { ir, context }.into(),
            PrimitiveList(ir) => TypePrimitiveListGenerator { ir, context }.into(),
            Optional(ir) => TypeOptionalGenerator { ir, context }.into(),
            OptionalList(ir) => TypeOptionalListGenerator { ir, context }.into(),
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
                panic!("Cannot generate Rust code for {}", string)
            }
        }
    }
}
