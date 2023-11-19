pub(crate) trait BasicCode: From<String> + From<&str> {
    fn all_code(&self) -> String;
}
