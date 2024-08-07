use std::any::Any;
use std::backtrace::Backtrace;
use std::cell::RefCell;
use std::panic::UnwindSafe;

thread_local! {
    static BACKTRACE: RefCell<Option<Backtrace>> = const { RefCell::new(None) };
}

/// Utility for tracking panic backtrace.
///
/// This is originally used internally, and only exposed because it is needed outside flutter_rust_bridge.
/// Therefore, the API may not follow semantics versioning.
pub struct PanicBacktrace;

impl PanicBacktrace {
    pub fn setup() {
        let old_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |arg| {
            let trace = Backtrace::capture();
            BACKTRACE.with(move |b| b.borrow_mut().replace(trace));

            old_hook(arg);
        }));
    }

    pub fn catch_unwind<F: FnOnce() -> R + UnwindSafe, R>(
        f: F,
    ) -> Result<R, CatchUnwindWithBacktrace> {
        std::panic::catch_unwind(f).map_err(|err| CatchUnwindWithBacktrace {
            err,
            backtrace: Self::take_last(),
        })
    }

    pub fn take_last() -> Option<Backtrace> {
        BACKTRACE.with(|b| b.borrow_mut().take())
    }
}

/// Similar to the output of catch_unwind, but with extra backtrace
pub struct CatchUnwindWithBacktrace {
    pub err: Box<dyn Any + Send + 'static>,
    pub backtrace: Option<Backtrace>,
}

impl CatchUnwindWithBacktrace {
    pub fn new(err: Box<dyn Any + Send + 'static>, backtrace: Option<Backtrace>) -> Self {
        Self { err, backtrace }
    }
}
