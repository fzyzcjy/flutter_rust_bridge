use crate::codegen::generator::misc::Target;

pub(crate) struct ExternFunc {
    // TODO handle platform
    // if matches!(target, Io) {
    //     self.names.push(func_name.to_string());
    // } else if (target == Target::Wasm) && !func_name.starts_with("wire_") {
    //     self.wasm_exports.push(...)
    // }
    pub(crate) func_name: String,
    pub(crate) params: impl IntoIterator<Item = (impl Display, impl Display)>,
    pub(crate) return_type: Option<String>,
    pub(crate) body: String,
    pub(crate) target: Target,
}

impl ExternFunc {
    pub(crate) fn generate(&self) -> String {
        format!(
            r#"
                {attr}
                pub {call_conv} fn {}({}) {} {{
                    {}
                }}
            "#,
            self.func_name,
            self.params.into_iter().map(|param| param.0).join(","),
            self.return_type
                .map_or("".to_string(), |r| format!("-> {r}")),
            self.body,
            attr = self.target.extern_func_attr(),
            call_conv = self.target.call_convention(),
        )
    }

    // TODO migrated from: ExternFuncCollector.wasm_exports.push(...)
    // comment out, since not migrate `IrFuncDisplay` yet
    // pub(crate) fn wasm_export() -> Option<IrFuncDisplay> {
    //     /// Functions starting with "wire_" are assumed to be from the original set of IrFuncs
    //     /// and not re-exported to WASM.
    //     if func_name.starts_with("wire_") {
    //         None
    //     } else {
    //         Some(IrFuncDisplay {
    //             name: func_name.to_owned(),
    //             inputs: params
    //                 .iter()
    //                 .map(|(verbatim, dart)| {
    //                     let verbatim = format!("{verbatim}");
    //                     let (key, _) = verbatim.split_once(':').expect("Missing middle colon");
    //                     IrParam {
    //                         name: key.to_owned(),
    //                         ty: format!("{dart}"),
    //                     }
    //                 })
    //                 .collect(),
    //             output: return_type.map(String::from).unwrap_or_default(),
    //             has_port_argument: false,
    //         })
    //     }
    // }
}
