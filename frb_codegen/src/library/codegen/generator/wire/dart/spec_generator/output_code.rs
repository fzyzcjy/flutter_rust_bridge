use crate::basic_code_impl;
use pathdiff::diff_paths;
use std::ops::{Add, AddAssign};

#[derive(Default, Clone)]
pub(crate) struct WireDartOutputCode {
    pub import: String,
    pub part: String,
    pub body: String,
    /// Code inside the generated dispatcher class
    pub dispatcher_body: String,
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
        self.body += &rhs.body;
        self.dispatcher_body += &rhs.dispatcher_body;
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
            part: "".to_string(),
            body: body.join("\n"),
            dispatcher_body: "".to_string(),
        }
    }

    pub(crate) fn all_code(&self, entrypoint_class_name: &str) -> String {
        let dispatcher_name = format!("{}Dispatcher", entrypoint_class_name);
        let dispatcher_class_code = format!(
            "
            class {dispatcher_name} extends BaseDispatcher {{
              {dispatcher_name}({{super.handler}});

              {dispatcher_body}
            }}
            ",
            dispatcher_body = self.dispatcher_body,
        );

        format!(
            "{}\n{}\n{}\n{}",
            self.import, self.part, dispatcher_class_code, self.body
        )
    }
}
