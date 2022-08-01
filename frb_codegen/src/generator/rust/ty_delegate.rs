use crate::generator::rust::ty::*;
use crate::generator::rust::{
    generate_list_allocate_func, ExternFuncCollector, TypeGeneralListGenerator,
};
use crate::ir::*;
use crate::type_rust_generator_struct;
use crate::utils::BlockIndex;

type_rust_generator_struct!(TypeDelegateGenerator, IrTypeDelegate);

macro_rules! delegate_enum {
    ($self:ident, $func:ident($( $tokens:tt )*), $ret:expr) => {
        if let IrTypeDelegate::PrimitiveEnum { ir, .. } = &$self.ir {
            super::TypeEnumRefGenerator {
                ir: ir.clone(),
                context: $self.context.clone(),
            }
            .$func($( $tokens )*)
        } else {
            $ret
        }
    };
}

impl TypeRustGeneratorTrait for TypeDelegateGenerator<'_> {
    fn wire2api_body(&self) -> Option<String> {
        Some(match &self.ir {
            IrTypeDelegate::String => {
                if self.context.wasm() {
                    "self".into()
                } else {
                    "let vec: Vec<u8> = self.wire2api(); String::from_utf8_lossy(&vec).into_owned()"
                        .into()
                }
            }
            IrTypeDelegate::SyncReturnVecU8 => "/*unsupported*/".into(),
            IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                "ZeroCopyBuffer(self.wire2api())".into()
            }
            IrTypeDelegate::StringList => TypeGeneralListGenerator::WIRE2API_BODY.to_string(),
            IrTypeDelegate::PrimitiveEnum { ir, .. } => {
                let enu = ir.get(self.context.ir_file);
                let variants = enu
                    .variants()
                    .iter()
                    .enumerate()
                    .map(|(idx, variant)| format!("{} => {}::{},", idx, enu.name, variant.name))
                    .collect::<Vec<_>>()
                    .join("\n");
                format!(
                    "match self {{
                        {}
                        _ => unreachable!(\"Invalid variant for {}: {{}}\", self),
                    }}",
                    variants, enu.name
                )
            }
        })
    }

    fn wire_struct_fields(&self) -> Option<Vec<String>> {
        match &self.ir {
            ty @ IrTypeDelegate::StringList => Some(vec![
                format!(
                    "ptr: *mut *mut {}",
                    ty.get_delegate().rust_wire_type(self.context.config.wasm)
                ),
                "len: i32".to_owned(),
            ]),
            _ => None,
        }
    }

    fn allocate_funcs(&self, collector: &mut ExternFuncCollector, _: BlockIndex) -> String {
        match &self.ir {
            list @ IrTypeDelegate::StringList => generate_list_allocate_func(
                collector,
                &self.ir.safe_ident(),
                list,
                &list.get_delegate(),
                self.context.config,
            ),
            _ => "".to_string(),
        }
    }

    fn impl_intodart(&self) -> String {
        if let IrTypeDelegate::PrimitiveEnum { ir, .. } = &self.ir {
            let src = ir.get(self.context.ir_file);
            let (name, self_path): (&str, &str) = match &src.wrapper_name {
                Some(wrapper) => (wrapper, &src.name),
                None => (&src.name, "Self"),
            };
            let self_ref = self.self_access("self".to_owned());
            let variants = src
                .variants()
                .iter()
                .enumerate()
                .map(|(idx, variant)| format!("{}::{} => {},", self_path, variant.name, idx))
                .collect::<Vec<_>>()
                .join("\n");
            return format!(
                "impl support::IntoDart for {} {{
                    fn into_dart(self) -> support::DartCObject {{
                        match {} {{
                            {}
                        }}.into_dart()
                    }}
                }}",
                name, self_ref, variants
            );
        }

        "".into()
    }

    fn wasm2api_body(&self) -> Option<std::borrow::Cow<str>> {
        matches!(self.ir, IrTypeDelegate::String)
            .then(|| "self.as_string().expect(\"non-UTF-8 string, or not a string\")".into())
    }

    fn imports(&self) -> Option<String> {
        delegate_enum!(self, imports(), None)
    }

    fn wrapper_struct(&self) -> Option<String> {
        delegate_enum!(self, wrapper_struct(), None)
    }

    fn wrap_obj(&self, obj: String) -> String {
        delegate_enum!(self, wrap_obj(obj), obj)
    }

    fn self_access(&self, obj: String) -> String {
        delegate_enum!(self, self_access(obj), obj)
    }

    fn static_checks(&self) -> Option<String> {
        delegate_enum!(self, static_checks(), None)
    }
}
