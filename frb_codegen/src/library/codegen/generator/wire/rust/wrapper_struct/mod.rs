use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::codegen::ir::ty::IrType;

mod ty;

fn generate(ty: &IrType, ir_pack: &IrPack) -> Option<String> {
    // the generated wrapper structs need to be public for the StreamSinkTrait impl to work
    match ty {
        IrType::StructRef(_)
        | IrType::EnumRef(_)
        | IrType::Delegate(IrTypeDelegate::PrimitiveEnum { .. }) => {
            TypeRustGenerator::new(ty.clone(), ir_pack, self.config)
                .wrapper_struct()
                .map(|wrapper| {
                    format!(
                        r###"
                            #[derive(Clone)]
                            pub struct {}({});
                        "###,
                        wrapper,
                        ty.rust_api_type(),
                    )
                })
        }
        _ => None,
    }
}
