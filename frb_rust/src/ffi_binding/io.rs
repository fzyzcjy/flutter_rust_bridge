use crate::codec::dco::Rust2DartMessageDco;
use crate::codec::sse::Rust2DartMessageSse;
use crate::codec::Rust2DartMessageTrait;
use crate::platform_types::{WireSyncRust2DartDco, WireSyncRust2DartSse};
use allo_isolate::{
    ffi::{DartCObject, DartPort},
    store_dart_post_cobject,
};
use std::ffi::c_void;
use std::sync::{Mutex, MutexGuard, PoisonError};

/// # Safety
///
/// This function should never be called manually.
#[no_mangle]
pub unsafe extern "C" fn frb_init_frb_dart_api_dl(data: *mut std::ffi::c_void) -> isize {
    #[cfg(feature = "dart-opaque")]
    return dart_sys::Dart_InitializeApiDL(data);
    #[cfg(not(feature = "dart-opaque"))]
    return 0;
}

/// # Safety
///
/// This function should never be called manually.
#[no_mangle]
pub unsafe extern "C" fn frb_free_wire_sync_rust2dart_dco(value: WireSyncRust2DartDco) {
    let _ = Rust2DartMessageDco::from_raw_wire_sync(value);
}

/// # Safety
///
/// This function should never be called manually.
#[no_mangle]
pub unsafe extern "C" fn frb_free_wire_sync_rust2dart_sse(value: WireSyncRust2DartSse) {
    let _ = Rust2DartMessageSse::from_raw_wire_sync(value);
}

/// How many isolates hold a shutdown callback.
///
/// A lock rather than an atomic, because counting down to zero and disabling
/// the post function must be one step as seen by a new isolate. With an
/// atomic, a callback that has already counted down to zero but not yet stored
/// `devnull` can be overtaken by a new isolate that counts itself and stores
/// the real function; the late `devnull` then silences an isolate that is
/// alive.
///
/// The other direction needs no lock, only an order: Dart counts the isolate
/// (`frb_create_shutdown_callback`) before it stores the post function.
struct IsolateCount(Mutex<u64>);

impl IsolateCount {
    const fn new() -> Self {
        Self(Mutex::new(0))
    }

    fn lock(&self) -> MutexGuard<'_, u64> {
        // A poisoned lock is still usable here, and `unwrap` would put a panic
        // path into the shutdown callback, which must not unwind across the
        // FFI boundary.
        self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn increment(&self) {
        let mut count = self.lock();
        *count = count.saturating_add(1);
    }

    /// Counts one isolate out and, if it was the last one, runs `on_last`
    /// before any other isolate can be counted in.
    fn decrement(&self, on_last: impl FnOnce()) {
        let mut count = self.lock();
        // Saturating, like `increment`: an unbalanced call must not panic
        // inside the shutdown callback either.
        *count = count.saturating_sub(1);
        if *count == 0 {
            on_last();
        }
    }
}

/// # Safety
///
/// This function should never be called manually.
#[no_mangle]
pub unsafe extern "C" fn frb_create_shutdown_callback() -> unsafe extern "C" fn(*mut c_void) {
    /// Counter for how many active shutdown callbacks there are.
    ///
    /// It is incremented when `frb_shutdown_callback` is returned and
    /// decremented when it is called.
    static ISOLATES_NUM: IsolateCount = IsolateCount::new();

    ISOLATES_NUM.increment();

    /// Called by Dart's `NativeFinalizer` when the isolate that took it shuts
    /// down.
    unsafe extern "C" fn frb_shutdown_callback(_: *mut c_void) {
        unsafe extern "C" fn devnull(_: DartPort, _: *mut DartCObject) -> bool {
            // Returning true is wrong since message is not enqueued and this
            // might cause memory leaks. But since application is shutting down
            // we don't really care and just want it to die silently without
            // triggering any send errors.
            true
        }

        // If this is the last callback we assume that application is shutting
        // down.
        ISOLATES_NUM.decrement(|| {
            // So `Dart_PostCObject` won't do anything from now on. We need this
            // cause once shutdown have started `Dart_Cleanup` might be called any
            // moment from now and `Dart_PostCObject` can only be used before
            // `Dart_Cleanup` has been called
            // For more information refer to:
            // https://github.com/dart-lang/native/issues/2079
            store_dart_post_cobject(devnull);
        });
    }

    frb_shutdown_callback
}

#[cfg(test)]
mod tests {
    use super::IsolateCount;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::{mpsc, Arc, TryLockError};
    use std::thread;
    use std::time::Duration;

    #[test]
    /// `on_last` runs for the last isolate only, and again for the last one of
    /// a later generation (another test file, a hot restart).
    fn isolate_count_runs_on_last_only_when_no_isolate_is_left() {
        let count = IsolateCount::new();
        let last = AtomicUsize::new(0);
        let shut_down = || {
            count.decrement(|| {
                last.fetch_add(1, Ordering::SeqCst);
            })
        };

        count.increment();
        count.increment();
        shut_down();
        assert_eq!(last.load(Ordering::SeqCst), 0, "an isolate is left");
        shut_down();
        assert_eq!(last.load(Ordering::SeqCst), 1, "no isolate is left");

        count.increment();
        shut_down();
        assert_eq!(last.load(Ordering::SeqCst), 2, "no isolate is left again");
    }

    #[test]
    /// `on_last` runs while the count is still locked, so that no isolate can
    /// be counted in before the post function has been disabled.
    fn isolate_count_runs_on_last_under_the_lock() {
        let count = IsolateCount::new();
        count.increment();

        let mut ran = false;
        count.decrement(|| {
            assert!(
                matches!(count.0.try_lock(), Err(TryLockError::WouldBlock)),
                "`on_last` runs after the lock is released"
            );
            ran = true;
        });
        assert!(ran);
    }

    #[test]
    /// A new isolate is not counted in while the last shutdown is still inside
    /// `on_last`, that is, before the post function has been disabled. With an
    /// atomic counter it would be, and the post function it then stores would
    /// be overwritten by the one `on_last` stores.
    fn isolate_count_keeps_new_isolates_out_until_on_last_is_done() {
        let count = Arc::new(IsolateCount::new());
        count.increment();

        let (entered_tx, entered_rx) = mpsc::channel();
        let (release_tx, release_rx) = mpsc::channel::<()>();
        let shutting_down = {
            let count = count.clone();
            thread::spawn(move || {
                count.decrement(|| {
                    entered_tx.send(()).unwrap();
                    release_rx.recv().unwrap();
                })
            })
        };
        entered_rx.recv().unwrap();

        let counted_in = Arc::new(AtomicBool::new(false));
        let initializing = {
            let (count, counted_in) = (count.clone(), counted_in.clone());
            thread::spawn(move || {
                count.increment();
                counted_in.store(true, Ordering::SeqCst);
            })
        };
        // Long enough for that thread to get in if nothing keeps it out.
        // Correct code cannot fail here, however the threads are scheduled.
        thread::sleep(Duration::from_millis(100));
        assert!(
            !counted_in.load(Ordering::SeqCst),
            "counted in while `on_last` was still running"
        );

        release_tx.send(()).unwrap();
        shutting_down.join().unwrap();
        initializing.join().unwrap();
        assert!(counted_in.load(Ordering::SeqCst));
    }
}
