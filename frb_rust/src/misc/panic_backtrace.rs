use backtrace::Backtrace;
use std::cell::RefCell;

thread_local! {
    static backtrace: RefCell<Option<Backtrace>> = RefCell::new(None);
}

pub(crate) struct PanicBacktrace;

impl PanicBacktrace {
    pub(crate) fn setup() {
        let old_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|arg| {
            let trace = Backtrace::new();
            backtrace.with(move |b| b.borrow_mut().replace(trace));

            old_hook(arg);
        }));
    }

    pub(crate) fn take_last() -> Option<Backtrace> {
        backtrace.with(|b| b.borrow_mut().take())
    }
}
