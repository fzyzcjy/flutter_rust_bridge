use crate::codegen::generator::misc::Target;
use itertools::Itertools;

pub(crate) struct ExternFunc {
    // TODO handle platform
    // if matches!(target, Io) {
    //     self.names.push(func_name.to_string());
    // } else if (target == Target::Wasm) && !func_name.starts_with("wire_") {
    //     self.wasm_exports.push(...)
    // }
    pub(crate) func_name: String,
    pub(crate) params: Vec<ExternFuncParam>,
    pub(crate) return_type: Option<String>,
    pub(crate) body: String,
    pub(crate) target: Target,
}

#[derive(Clone)]
pub(crate) struct ExternFuncParam {
    pub(crate) name: String,
    pub(crate) rust_type: String,
    pub(crate) dart_type: Option<String>,
}

impl ExternFunc {
    pub(crate) fn generate(&self) -> String {
        let call_convention = match self.target {
            Target::Io => "extern \"C\"",
            Target::Wasm => "",
        };
        let attribute = match self.target {
            Target::Io => "#[no_mangle]",
            Target::Wasm => "#[wasm_bindgen]",
        };
        let ExternFunc {
            func_name, body, ..
        } = self;

        format!(
            r#"
                {attribute}
                pub {call_convention} fn {func_name}({}) {} {{
                    {body}
                }}
            "#,
            self.params
                .iter()
                .map(|param| param.rust_name_and_type())
                .join(", "),
            self.return_type
                .as_ref()
                .map_or("".to_owned(), |r| format!("-> {r}")),
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
    //                 // TODO (verbatim, dart) ---> (name + rust_type, dart_type)
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

impl ExternFuncParam {
    pub(crate) fn rust_name_and_type(&self) -> String {
        format!("{}: {}", self.name, self.rust_type)
    }
}
