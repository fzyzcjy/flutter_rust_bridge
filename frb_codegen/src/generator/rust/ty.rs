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

    fn get_context(&self) -> &TypeGeneratorContext;

    fn get_shared_mod_name_if_type_shared(&self, ty: &IrType) -> Option<String> {
        if self.get_context().ir_file.is_type_shared_by_safe_ident(ty) {
            return SHARED_MODULE.with(|data| {
                let cloned = data.borrow().clone();
                if cloned.is_none() {
                    panic!("in instance in charge of `{}`: checking shared for type \"{:?}\", it is shared indeed, but the shared module name is None",
                        self.get_context().type_name, ty
                    );
                }
                cloned
            });
        }
        None
    }

    fn get_shared_module_of_a_type(&self, ir_type: &IrType) -> Option<String> {
        match ir_type {
            IrType::Optional(inner_type) => {
                self.get_shared_mod_name_if_type_shared(&inner_type.inner)
            }
            _ => self.get_shared_mod_name_if_type_shared(ir_type),
        }
    }

    fn get_wire2api_prefix(&self, ir_type: &IrType) -> String {
        let shared_mod_name = self.get_shared_module_of_a_type(ir_type);

        if !self.get_context().config.shared && shared_mod_name.is_some() {
            format!("{}::Wire2Api", shared_mod_name.unwrap())
        } else {
            "Wire2Api".into()
        }
    }
}

#[derive(Debug, Clone)]
pub struct TypeGeneratorContext<'a> {
    pub type_name: String,
    pub ir_file: &'a IrFile,
    pub config: &'a Opts,
}

#[macro_export]
macro_rules! type_rust_generator_struct {
    ($cls:ident, $ir_cls:ty) => {
        #[derive(Debug, Clone)]
        pub struct $cls<'a> {
            pub ir: $ir_cls,
            pub context: $crate::generator::rust::ty::TypeGeneratorContext<'a>,
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
    pub fn new(ty: IrType, ir_file: &'a IrFile, config: &'a Opts) -> Self {
        let context = TypeGeneratorContext {
            type_name: format!("{ty:?}"),
            ir_file,
            config,
        };
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
                panic!("Cannot generate Rust code for {}", string)
            }
        }
    }
}
