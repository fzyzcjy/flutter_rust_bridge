use crate::codegen::ir::ty::IrType::*;

macro_rules! generate_code {
    ($($name:ident),*,) => (
        paste! {
            #[enum_dispatch(DartApiClassGeneratorTrait)]
            pub enum DartApiGenerator<'a> {
                $(
                    $name([<$name DartApiGenerator>]<'a>),
                )*
            }

            impl<'a> DartApiGenerator<'a> {
                pub(crate) fn new(ty: IrType, ir_pack: &'a IrPack) -> Option<Self> {
                    let context = DartApiGeneratorContext { ir_pack };
                    Some(match ty {
                        $(
                            $name(ir) => [<$name DartApiGenerator>]::new(ir, context).into(),
                        )*
                    })
                }
            }

            $(
                #[derive(Debug, Clone)]
                pub(super) struct [<$name DartApiGenerator>]<'a> {
                    ir: [<IrType $name>],
                    ir_pack: &'a crate::codegen::ir::pack::IrPack,
                }

                impl<'a> [<$name DartApiGenerator>]<'a> {
                    pub(super) fn new(ir: [<IrType $name>], context: DartApiGeneratorContext<'a>) -> Self {
                        Self { ir, ir_pack }
                    }
                }
            )*
        }
    )
}

generate_code!(
    Boxed,
    DartOpaque,
    Delegate,
    Dynamic,
    EnumRef,
    GeneralList,
    Optional,
    OptionalList,
    Primitive,
    PrimitiveList,
    Record,
    RustOpaque,
    StructRef,
    Unencodable,
);
