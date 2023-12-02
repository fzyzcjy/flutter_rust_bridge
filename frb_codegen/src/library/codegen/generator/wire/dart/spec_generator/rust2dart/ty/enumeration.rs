use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::rust2dart::ty::WireDartGeneratorRust2DartTrait;
use crate::codegen::ir::ty::enumeration::{IrEnumMode, IrVariantKind};
use crate::library::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;

impl<'a> WireDartGeneratorRust2DartTrait for EnumRefWireDartGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> String {
        let enu = self.ir.get(self.context.ir_pack);
        assert_eq!(enu.mode, IrEnumMode::Complex);

        let variants = enu
            .variants()
            .iter()
            .enumerate()
            .map(|(idx, variant)| {
                let args = match &variant.kind {
                    IrVariantKind::Value => "".to_owned(),
                    IrVariantKind::Struct(st) => st
                        .fields
                        .iter()
                        .enumerate()
                        .map(|(idx, field)| {
                            let val =
                                format!("_wire2api_{}(raw[{}]),", field.ty.safe_ident(), idx + 1);
                            if st.is_fields_named {
                                format!("{}: {}", field.name.dart_style(), val)
                            } else {
                                val
                            }
                        })
                        .collect_vec()
                        .join(""),
                };
                format!("case {}: return {}({});", idx, variant.wrapper_name, args)
            })
            .collect_vec();
        format!(
            "switch (raw[0]) {{
                {}
                default: throw Exception(\"unreachable\");
            }}",
            variants.join("\n"),
        )
    }
}
