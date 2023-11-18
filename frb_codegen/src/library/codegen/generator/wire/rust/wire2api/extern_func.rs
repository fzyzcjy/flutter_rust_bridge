use crate::codegen::generator::misc::Target;
use itertools::{concat, Itertools};
use std::ops::Add;

#[derive(Default)]
pub(crate) struct CodeWithExternFunc {
    pub(crate) code: String,
    pub(crate) extern_funcs: Vec<ExternFunc>,
}

impl CodeWithExternFunc {
    pub fn code(code: String) -> Self {
        Self {
            code,
            ..Default::default()
        }
    }
}

impl Add for CodeWithExternFunc {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            code: self.code + &rhs.code,
            extern_funcs: concat([self.extern_funcs, rhs.extern_funcs]),
        }
    }
}

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

pub(crate) struct ExternFuncParam {
    name: String,
    rust_type: String,
    dart_type: String,
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
                .map(|param| format!("{}: {}", param.name, param.rust_type))
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
