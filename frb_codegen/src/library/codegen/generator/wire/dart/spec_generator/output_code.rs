use crate::basic_code_impl;
use crate::codegen::generator::misc::target::TargetOrCommon;
use crate::codegen::generator::wire::dart::internal_config::DartOutputClassNamePack;
use std::ops::AddAssign;

#[derive(Default, Clone)]
pub(crate) struct WireDartOutputCode {
    pub import: String,
    pub part: String,
    pub body_top: String,
    /// Code inside the generated dispatcher class
    pub dispatcher_body: String,
    pub body: String,
}

basic_code_impl!(WireDartOutputCode);

impl From<String> for WireDartOutputCode {
    fn from(body: String) -> Self {
        Self {
            body,
            ..Default::default()
        }
    }
}

impl AddAssign for WireDartOutputCode {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.import += &rhs.import;
        self.part += &rhs.part;
        self.body_top += &rhs.body_top;
        self.dispatcher_body += &rhs.dispatcher_body;
        self.body += &rhs.body;
    }
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
            import: imports.join("\n"),
            body: body.join("\n"),
            ..Default::default()
        }
    }

    pub(crate) fn all_code(
        &self,
        target: TargetOrCommon,
        dart_output_class_name_pack: &DartOutputClassNamePack,
    ) -> String {
        let DartOutputClassNamePack { .. } = &dart_output_class_name_pack;

        let dispatcher_class_code = if target == TargetOrCommon::Common {
            format!(
                "
                class {dispatcher_name} extends BaseDispatcher {{
                  {dispatcher_name}({{super.handler}});

                  {dispatcher_body}
                }}
                ",
                dispatcher_body = self.dispatcher_body,
            )
        } else {
            assert_eq!(&self.dispatcher_body, "");
            "".into()
        };

        format!(
            "{}\n{}\n{}\n{}\n{}",
            self.import, self.part, self.body_top, dispatcher_class_code, self.body
        )
    }
}
