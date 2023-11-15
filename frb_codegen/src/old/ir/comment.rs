impl IrComment {
    pub fn comment(&self) -> &str {
        &self.0
    }
}

impl From<&str> for IrComment {
    fn from(input: &str) -> Self {
        if input.contains('\n') {
            // Dart's formatter has issues with block comments
            // so we convert them ahead of time.
            let formatted = input
                .split('\n')
                .map(|line| format!("///{line}"))
                .collect::<Vec<_>>()
                .join("\n");
            Self(formatted)
        } else {
            Self(format!("///{input}"))
        }
    }
}
