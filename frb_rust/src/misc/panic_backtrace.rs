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

    pub(crate) fn catch_unwind<F: FnOnce() -> R + UnwindSafe, R>(
        f: F,
    ) -> Result<T, CatchUnwindWithBacktrace> {
        std::panic::catch_unwind(f).map_err(|err| CatchUnwindWithBacktrace {
            err,
            backtrace: Self::take_last(),
        })
    }

    fn take_last() -> Option<Backtrace> {
        backtrace.with(|b| b.borrow_mut().take())
    }
}

pub(crate) struct CatchUnwindWithBacktrace {
    err: Box<dyn Any + Send + 'static>,
    backtrace: Option<Backtrace>,
}
