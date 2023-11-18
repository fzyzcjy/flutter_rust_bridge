use crate::codegen::generator::wire::dart::base::*;
use crate::codegen::generator::wire::dart::wire2api::ty::WireDartGeneratorWire2apiTrait;

impl<'a> WireDartGeneratorWire2apiTrait for StructRefWireDartGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> String {
        let src = self.ir.get(self.context.ir_pack);
        let s = self.ir.get(self.context.ir_pack);

        let mut methods = self.context.ir_pack.funcs.iter().filter(|f| {
            let f = FunctionName::deserialize(&f.name);
            f.is_instance_method_for_struct(&src.name) || f.is_static_method_for_struct(&src.name)
        });
        let has_methods = methods.next().is_some();
        let mut inner = s
            .fields
            .iter()
            .enumerate()
            .map(|(idx, field)| {
                format!(
                    "{}: _wire2api_{}(arr[{}]),",
                    field.name.dart_style(),
                    field.ty.safe_ident(),
                    idx
                )
            })
            .collect_vec();
        if has_methods && self.context.config.use_bridge_in_method {
            inner.insert(0, "bridge: this,".to_string());
        }

        let inner = inner.join("\n");
        let cast = "final arr = raw as List<dynamic>;".to_string();
        let safe_check = format!("if (arr.length != {}) throw Exception('unexpected arr length: expect {} but see ${{arr.length}}');", s.fields.len(), s.fields.len());
        format!(
            "{}
                {}
                return {}({});",
            cast, safe_check, s.name, inner,
        )
    }
}
