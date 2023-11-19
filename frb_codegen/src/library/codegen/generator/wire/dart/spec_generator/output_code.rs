use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use std::ops::{Add, AddAssign};

#[derive(Default, Clone)]
pub(crate) struct WireDartOutputCode {
    pub import: String,
    pub part: String,
    pub body: String,
}

impl From<String> for WireDartOutputCode {
    fn from(value: String) -> Self {
        todo!()
    }
}

impl From<&str> for WireDartOutputCode {
    fn from(value: &str) -> Self {
        value.to_owned().into()
    }
}

impl AddAssign for WireDartOutputCode {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl Add for &WireDartOutputCode {
    type Output = WireDartOutputCode;

    fn add(self, rhs: Self) -> Self::Output {
        WireDartOutputCode {
            import: format!("{}\n{}", self.import, rhs.import),
            part: format!("{}\n{}", self.part, rhs.part),
            body: format!("{}\n{}", self.body, rhs.body),
        }
    }
}

impl Add<&WireDartOutputCode> for WireDartOutputCode {
    type Output = WireDartOutputCode;

    fn add(self, rhs: &WireDartOutputCode) -> Self::Output {
        (&self).add(rhs)
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

    pub fn all_code(&self) -> String {
        format!("{}\n{}\n{}", self.import, self.part, self.body)
    }
}
