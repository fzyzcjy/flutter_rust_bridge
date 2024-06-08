use crate::simple_code_trait_impl;
use crate::codegen::generator::misc::target::TargetOrCommon;
use crate::codegen::generator::wire::dart::internal_config::DartOutputClassNamePack;
use crate::utils::basic_code::dart_header_code::DartHeaderCode;
use itertools::Itertools;
use serde::Serialize;
use std::ops::AddAssign;

#[derive(Default, Clone, Debug, Serialize)]
pub(crate) struct WireDartOutputCode {
    pub header: DartHeaderCode,
    pub body_top: String,
    pub api_class_body: String,
    pub api_impl_class_body: String,
    pub api_impl_class_methods: Vec<DartApiImplClassMethod>,
    pub body: String,
}

simple_code_trait_impl!(WireDartOutputCode);

impl AddAssign for WireDartOutputCode {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.header += rhs.header;
        self.body_top += &rhs.body_top;
        self.api_class_body += &rhs.api_class_body;
        self.api_impl_class_body += &rhs.api_impl_class_body;
        self.api_impl_class_methods
            .extend(rhs.api_impl_class_methods);
        self.body += &rhs.body;
    }
}

#[derive(Default, Clone, Debug, Serialize)]
pub(crate) struct DartApiImplClassMethod {
    pub signature: String,
    pub body: Option<String>,
}

impl WireDartOutputCode {
    pub fn parse(raw: &str) -> WireDartOutputCode {
        let (mut imports, mut body) = (Vec::new(), Vec::new());
        for line in raw.split('\n') {
            (if line.starts_with("import ") {
                &mut imports
            } else {
                &mut body
            })
            .push(line);
        }
        WireDartOutputCode {
            header: DartHeaderCode {
                import: imports.join("\n"),
                ..Default::default()
            },
            body: body.join("\n"),
            ..Default::default()
        }
    }

    pub(crate) fn all_code(
        &self,
        target: TargetOrCommon,
        dart_output_class_name_pack: &DartOutputClassNamePack,
    ) -> String {
        let DartOutputClassNamePack {
            api_class_name,
            api_impl_class_name,
            api_impl_platform_class_name,
            wire_class_name,
            ..
        } = &dart_output_class_name_pack;
        let WireDartOutputCode {
            api_class_body,
            api_impl_class_body,
            api_impl_class_methods,
            ..
        } = &self;

        let api_class_code = if target == TargetOrCommon::Common {
            format!(
                "
                abstract class {api_class_name} extends BaseApi {{
                  {api_class_body}
                }}
                ",
            )
        } else {
            assert_eq!(api_class_body, "");
            "".to_owned()
        };

        let api_impl_class_methods = api_impl_class_methods
            .iter()
            .map(|method| {
                let DartApiImplClassMethod { signature, body } = method;
                format!(
                    "@protected {signature}{}",
                    if let Some(body) = body {
                        format!("{{ {body} }}")
                    } else {
                        ";".to_owned()
                    }
                )
            })
            .join("\n\n");

        let api_impl_class_code = if target == TargetOrCommon::Common {
            format!(
                "
                class {api_impl_class_name} extends {api_impl_platform_class_name} implements {api_class_name} {{
                  {api_impl_class_name}({{
                    required super.handler,
                    required super.wire,
                    required super.generalizedFrbRustBinding,
                    required super.portManager,
                  }});

                  {api_impl_class_body}

                  {api_impl_class_methods}
                }}
                ",
            )
        } else {
            format!(
                "
                abstract class {api_impl_platform_class_name} extends BaseApiImpl<{wire_class_name}> {{
                  {api_impl_platform_class_name}({{
                    required super.handler,
                    required super.wire,
                    required super.generalizedFrbRustBinding,
                    required super.portManager,
                  }});

                  {api_impl_class_body}

                  {api_impl_class_methods}
                }}
                ",
            )
        };

        format!(
            "{}\n{}\n{}\n{}\n{}",
            self.header.all_code(),
            self.body_top,
            api_class_code,
            api_impl_class_code,
            self.body
        )
    }
}
