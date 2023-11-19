use std::ops::AddAssign;

// TODO merge with `DartBasicCode`
#[derive(Default)]
pub(crate) struct WireDartOutputCode(pub String);

impl From<String> for WireDartOutputCode {
    fn from(value: String) -> Self {
        Self(value)
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
        self.0 += &rhs.0;
    }
}
