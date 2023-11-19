use crate::basic_code_impl;
use crate::utils::basic_code::BasicCode;
use std::ops::{Add, AddAssign};

#[derive(Default, Clone)]
pub(crate) struct WireDartOutputCode {
    pub import: String,
    pub part: String,
    pub body: String,
}

basic_code_impl!(WireDartOutputCode);

impl BasicCode for WireDartOutputCode {
    fn all_code(&self) -> String {
        format!("{}\n{}\n{}", self.import, self.part, self.body)
    }
}

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
        }
    }
}
