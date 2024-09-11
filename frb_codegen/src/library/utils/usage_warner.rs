use std::cell::RefCell;

#[derive(Clone, Eq, PartialEq, Debug)]
pub(crate) struct UsageWarner<T> {
    value: T,
    warning_message: Option<String>,
    warned: RefCell<bool>,
}

impl<T> UsageWarner<T> {
    pub(crate) fn new(value: T, warning_message: Option<String>) -> Self {
        Self {
            value,
            warning_message,
            warned: RefCell::new(false),
        }
    }

    // Cannot name this method `use` since it is keyword
    pub(crate) fn borrow(&self) -> &T {
        self.maybe_warn();
        self.get_without_use()
    }

    pub(crate) fn get_without_use(&self) -> &T {
        &self.value
    }

    fn maybe_warn(&self) {
        if let Some(warning_message) = self.warning_message.as_ref() {
            if *self.warned.borrow() {
                return;
            }

            *self.warned.borrow_mut() = true;
            log::warn!("{}", warning_message)
        }
    }
}
