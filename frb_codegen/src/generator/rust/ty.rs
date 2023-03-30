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

    fn wrap_obj(&self, obj: String, _wired_fallible_func: bool) -> String {
        obj
    }

    fn convert_to_dart(&self, obj: String) -> String {
        format!("{obj}.into_dart()")
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

    fn get_shared_mod_name_if_type_shared(&self, ty: &IrType) -> Option<&str> {
        if self.get_context().ir_file.is_type_shared(&ty) {
            if let Some(shared_mod_name) = self.get_context().shared_mod_name {
                if shared_mod_name.is_empty() {
                    panic!(
                        "type \"{}\" is shared, but the shared module name is empty",
                        self.get_context().type_name
                    );
                }
                return Some(shared_mod_name);
            } else {
                panic!(
                    "type \"{}\" is shared, but the shared module name is None",
                    self.get_context().type_name
                );
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct TypeGeneratorContext<'a> {
    pub type_name: String,
    pub shared: bool, // if the type bonded to this instance is shared among different regular blocks, it should be true.
    pub ir_file: &'a IrFile,
    pub config: &'a Opts,
    pub shared_mod_name: Option<&'a str>, // if there is only 1 API block, it should be None. Otherwise, it should not be None.
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
}

impl<'a> TypeRustGenerator<'a> {
    pub fn new(
        ty: IrType,
        ir_file: &'a IrFile,
        config: &'a Opts,
        shared_mod_name: Option<&'a str>,
    ) -> Self {
        let context = TypeGeneratorContext {
            type_name: format!("{ty:?}"),
            shared: ir_file.is_type_shared(&ty),
            ir_file,
            config,
            shared_mod_name,
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
        }
    }
}
