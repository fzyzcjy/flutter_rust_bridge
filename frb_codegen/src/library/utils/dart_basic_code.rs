use std::ops::Add;

#[derive(Default)]
pub struct DartBasicCode {
    pub import: String,
    pub part: String,
    pub body: String,
}

impl Add for &DartBasicCode {
    type Output = DartBasicCode;

    fn add(self, rhs: Self) -> Self::Output {
        DartBasicCode {
            import: format!("{}\n{}", self.import, rhs.import),
            part: format!("{}\n{}", self.part, rhs.part),
            body: format!("{}\n{}", self.body, rhs.body),
        }
    }
}

impl Add<&DartBasicCode> for DartBasicCode {
    type Output = DartBasicCode;

    fn add(self, rhs: &DartBasicCode) -> Self::Output {
        (&self).add(rhs)
    }
}

impl DartBasicCode {
    pub fn parse(raw: &str) -> DartBasicCode {
        let (mut imports, mut body) = (Vec::new(), Vec::new());
        for line in raw.split('\n') {
            (if line.starts_with("import ") {
                &mut imports
            } else {
                &mut body
            })
            .push(line);
        }
        DartBasicCode {
            import: imports.join("\n"),
            part: "".to_string(),
            body: body.join("\n"),
        }
    }

    pub fn to_text(&self) -> String {
        format!("{}\n{}\n{}", self.import, self.part, self.body)
    }
}
