use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::IrType;

mod ty;

pub(crate) fn generate(ty: &IrType, ir_pack: &IrPack) -> Option<String> {
    todo!()
    // if let Some(fields) =
    //     TypeRustGenerator::new(ty.clone(), ir_pack, self.config).wire_struct_fields()
    // {
    //     Some(format!(
    //         r###"
    //             #[repr(C)]
    //             #[derive(Clone)]
    //             pub struct {} {{
    //                 {}
    //             }}
    //             "###,
    //         ty.rust_wire_type(Target::Io),
    //         fields.join(",\n"),
    //     ))
    // } else {
    //     None
    // }
}
