use convert_case::Case;
use crate::codegen::ir::ty::{IrType, IrTypeTrait};

crate::ir! {
pub struct IrTypeEnumRef {
    pub name: String,
    pub is_exception: bool,
}
}

impl IrTypeTrait for IrTypeEnumRef {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_file: &IrFile) {
        let enu = self.get(ir_file);
        for variant in enu.variants() {
            if let IrVariantKind::Struct(st) = &variant.kind {
                st.fields
                    .iter()
                    .for_each(|field| field.ty.visit_types(f, ir_file));
            }
        }
    }

    fn safe_ident(&self) -> String {
        self.dart_api_type().to_case(Case::Snake)
    }
}
