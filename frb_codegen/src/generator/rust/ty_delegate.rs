use crate::generator::rust::ty::*;
use crate::generator::rust::{
    generate_list_allocate_func, ExternFuncCollector, TypeGeneralListGenerator,
};
use crate::ir::*;
use crate::target::{Acc, Target};
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
    fn wire2api_body(&self) -> Acc<Option<String>> {
        match &self.ir {
            IrTypeDelegate::String => {
                Acc {
                    wasm: Some("self".into()),
                    io: Some("let vec: Vec<u8> = self.wire2api(); String::from_utf8_lossy(&vec).into_owned()".into()),
                    ..Default::default()
                }
            }
            IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                Acc::distribute(Some("ZeroCopyBuffer(self.wire2api())".into()))
            }
            IrTypeDelegate::StringList => Acc{
                io: Some(TypeGeneralListGenerator::WIRE2API_BODY_IO.into()),
                wasm: Some(TypeGeneralListGenerator::WIRE2API_BODY_WASM.into()),
                ..Default::default()
            },
            IrTypeDelegate::PrimitiveEnum { ir, .. } => {
                let enu = ir.get(self.context.ir_file);
                let variants = (enu
                    .variants()
                    .iter()
                    .enumerate())
                    .map(|(idx, variant)| format!("{} => {}::{},", idx, enu.name, variant.name))
                    .collect::<Vec<_>>()
                    .join("\n");
                format!(
                    "match self {{
                        {}
                        _ => unreachable!(\"Invalid variant for {}: {{}}\", self),
                    }}",
                    variants, enu.name
                ).into()
            },
            #[cfg(feature = "chrono")]
            IrTypeDelegate::Time(ir) => match ir {
                IrTypeTime::Naive => Acc {
                  io: Some("
                  let s = (self / 1_000_000) as i64;
                  let ns = (self.rem_euclid(1_000_000) * 1_000) as u32;
                  chrono::NaiveDateTime::from_timestamp(s, ns)".into()),
                  common: None,
                  wasm: Some("
                  let s = (self / 1_000) as i64;
                  let ns = (self.rem_euclid(1_000) * 1_000_000) as u32;
                  chrono::NaiveDateTime::from_timestamp(s, ns)".into()),
                },
                IrTypeTime::Local => Acc {
                  io: Some("
                  use chrono::TimeZone;
                  let s = (self / 1_000_000) as i64;
                  let ns = (self.rem_euclid(1_000_000) * 1_000) as u32;
                  chrono::Local.from_utc_datetime(&chrono::NaiveDateTime::from_timestamp(s, ns))".into()),
                  common: None,
                  wasm: Some("
                  let s = (self / 1_000) as i64;
                  let ns = (self.rem_euclid(1_000) * 1_000_000) as u32;
                  chrono::DateTime::<chrono::Local>::from(chrono::DateTime::<chrono::Utc>::from_utc(chrono::NaiveDateTime::from_timestamp(s, ns), chrono::Utc))
                  ".into()),
                },
                IrTypeTime::Utc => Acc {
                  io: Some("
                  let s = (self / 1_000_000) as i64;
                  let ns = (self.rem_euclid(1_000_000) * 1_000) as u32;
                  chrono::DateTime::<chrono::Utc>::from_utc(chrono::NaiveDateTime::from_timestamp(s, ns), chrono::Utc)".into()),
                  common: None,
                  wasm: Some("
                  let s = (self / 1_000) as i64;
                  let ns = (self.rem_euclid(1_000) * 1_000_000) as u32;
                  chrono::DateTime::<chrono::Utc>::from_utc(chrono::NaiveDateTime::from_timestamp(s, ns), chrono::Utc)
                  ".into()),
                },
                IrTypeTime::Duration => Acc {
                  common: None,
                  io: Some("chrono::Duration::microseconds(self)".into()),
                  wasm: Some("chrono::Duration::milliseconds(self)".into())
                },
            },
        }
    }

    fn wire_struct_fields(&self) -> Option<Vec<String>> {
        match &self.ir {
            ty @ IrTypeDelegate::StringList => Some(vec![
                format!(
                    "ptr: *mut *mut {}",
                    ty.get_delegate().rust_wire_type(Target::Io)
                ),
                "len: i32".to_owned(),
            ]),
            _ => None,
        }
    }

    fn allocate_funcs(
        &self,
        collector: &mut ExternFuncCollector,
        _: BlockIndex,
    ) -> Acc<Option<String>> {
        match &self.ir {
            list @ IrTypeDelegate::StringList => Acc {
                io: Some(generate_list_allocate_func(
                    collector,
                    &self.ir.safe_ident(),
                    list,
                    &list.get_delegate(),
                    self.context.config.block_index,
                )),
                ..Default::default()
            },
            _ => Default::default(),
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
                    fn into_dart(self) -> support::DartAbi {{
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

    fn wire2api_jsvalue(&self) -> Option<std::borrow::Cow<str>> {
        Some(match &self.ir {
            IrTypeDelegate::String => {
                "self.as_string().expect(\"non-UTF-8 string, or not a string\")".into()
            }
            IrTypeDelegate::PrimitiveEnum { repr, .. } => format!(
                "(self.unchecked_into_f64() as {}).wire2api()",
                repr.rust_wire_type(Target::Wasm)
            )
            .into(),
            IrTypeDelegate::ZeroCopyBufferVecPrimitive(_) => {
                "ZeroCopyBuffer(self.wire2api())".into()
            }
            _ => return None,
        })
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
