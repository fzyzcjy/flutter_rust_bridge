use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::*;

use anyhow::*;
#[cfg(feature = "extra_debug_output")]
use log::*;
#[cfg(feature = "extra_debug_output")]
use parking_lot::Mutex;

#[macro_export]
macro_rules! cancel_token_checkpoint {
    ($token:expr) => {
        $token.checkpoint(&concat!(module_path!(), ":", line!()))?;
    };
}

#[derive(Clone)]
struct CancelTokenDebugInfo {
    construct_time: Instant,
    last_cancel_time: Option<Instant>,
    last_checkpoint_time: Option<Instant>,
    last_checkpoint_name: Option<String>,
}

pub struct CancelToken {
    cancelled: AtomicBool,
    #[cfg(feature = "extra_debug_output")]
    debug_info: Mutex<CancelTokenDebugInfo>,
}

impl CancelToken {
    pub fn cancelled_to_null<F, Ret>(f: F) -> Result<Option<Ret>>
    where
        F: FnOnce() -> Result<Ret>,
    {
        match f() {
            std::result::Result::Ok(ret) => Ok(Some(ret)),
            Err(e) => {
                if e.downcast_ref::<CancelledError>().is_some() {
                    Ok(None)
                } else {
                    Err(e)
                }
            }
        }
    }
}

impl CancelToken {
    pub fn new() -> Self {
        CancelToken {
            cancelled: AtomicBool::new(false),
            #[cfg(feature = "extra_debug_output")]
            debug_info: Mutex::new(CancelTokenDebugInfo {
                construct_time: Instant::now(),
                last_cancel_time: None,
                last_checkpoint_time: None,
                last_checkpoint_name: None,
            }),
        }
    }

    pub fn cancel(&self) {
        #[cfg(feature = "extra_debug_output")]
        {
            self.debug_info.lock().last_cancel_time = Some(Instant::now());
        }

        self.cancelled.store(true, Ordering::Release);
    }

    pub fn checkpoint(&self, _debug_name: &str) -> Result<(), CancelledError> {
        let cancelled = self.cancelled.load(Ordering::Acquire);

        #[cfg(feature = "extra_debug_output")]
        {
            let now = Instant::now();
            let last_debug_info = self.debug_info.lock().clone();

            let last_checkpoint_elapsed = last_debug_info
                .last_checkpoint_time
                .map(|last_checkpoint_time| now - last_checkpoint_time);

            const CHECKPOINT_MAX_INTERVAL: Duration = Duration::from_millis(200);
            let checkpoint_too_coarse = last_checkpoint_elapsed.is_some()
                && last_checkpoint_elapsed.unwrap() > CHECKPOINT_MAX_INTERVAL;

            debug!(
                "[Checkpoint] name={} {}now_vs_last={}ms now_vs_init={}ms cancelled={}",
                _debug_name,
                if checkpoint_too_coarse {
                    "[WARN:TOO-COARSE] "
                } else {
                    ""
                },
                last_checkpoint_elapsed
                    .map(|t| t.as_millis() as i64)
                    .unwrap_or(-1),
                (now - last_debug_info.construct_time).as_millis(),
                cancelled,
            );

            if cancelled {
                let last_cancel_time = last_debug_info.last_cancel_time.unwrap();
                let time_to_act = now - last_cancel_time;
                const TIME_TO_ACT_MAX_ALLOWED: Duration = Duration::from_millis(200);

                info!(
                    "[CheckpointCancelled] name={} {}TIME-TO-ACT={}ms last_cancel_time_vs_init={:?} now_vs_init={:?}",
                    _debug_name,
                    if time_to_act > TIME_TO_ACT_MAX_ALLOWED {
                        "[WARN:TIME-TO-ACT-TOO-SLOW] "
                    } else {
                        ""
                    },
                    time_to_act.as_millis(),
                    (last_cancel_time - last_debug_info.construct_time).as_millis(),
                    (now - last_debug_info.construct_time).as_millis(),
                );
            }

            {
                let mut debug_info_guard = self.debug_info.lock();
                debug_info_guard.last_checkpoint_time = Some(Instant::now());
                debug_info_guard.last_checkpoint_name = Some(_debug_name.to_string());
            }
        }

        if cancelled {
            Err(CancelledError)
        } else {
            core::result::Result::Ok(())
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CancelledError;

impl fmt::Display for CancelledError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CancelledError")
    }
}

use super::pool::PoolObjectHandle;

impl std::error::Error for CancelledError {}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct CancelTokenHandle(pub i64, pub i64);

impl_pool_object_handle!(Arc<CancelToken>, CancelTokenHandle);

pub fn cancel_token_new(sub_pool_id: i64) -> Result<CancelTokenHandle> {
    Ok(CancelTokenHandle::get_pool().put_object(sub_pool_id, Arc::new(CancelToken::new())))
}

pub fn cancel_token_cancel(cancel_token: CancelTokenHandle) -> Result<()> {
    let cancel_token = CancelTokenHandle::get_pool().get_cloned_object(cancel_token)?;
    cancel_token.cancel();
    Ok(())
}

pub fn cancel_token_remove(cancel_token: CancelTokenHandle) -> Result<()> {
    CancelTokenHandle::get_pool().remove_object(cancel_token);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::thread::{sleep, spawn};

    use super::*;

    #[test]
    fn simple_with_cancel() -> Result<()> {
        let cancel_token = Arc::new(CancelToken::new());
        let cancel_token_2 = cancel_token.clone();

        let child_thread = spawn(move || match simple_counter(cancel_token_2.as_ref()) {
            Ok(_) => panic!("should not see Ok from counter"),
            Err(e) => assert_eq!(format!("{}", e), "CancelledError"),
        });

        println!("Parent thread: start sleep some time");
        sleep(Duration::from_millis(550));
        println!("Parent thread: call cancel");
        cancel_token.cancel();

        child_thread.join().unwrap();

        Ok(())
    }

    #[test]
    fn simple_with_succeed() -> Result<()> {
        let cancel_token = Arc::new(CancelToken::new());
        let cancel_token_2 = cancel_token.clone();

        let child_thread = spawn(move || match simple_counter(cancel_token_2.as_ref()) {
            Ok(_) => assert!(true),
            Err(_) => panic!("should not error or cancel"),
        });

        println!("Parent thread: start sleep some time");
        sleep(Duration::from_millis(1500)); // thus too late to cancel
        println!("Parent thread: call cancel");
        cancel_token.cancel();

        child_thread.join().unwrap();

        Ok(())
    }

    fn simple_counter(cancel_token: &CancelToken) -> Result<()> {
        for i in 0..10 {
            println!("Counter: i={}", i);
            cancel_token_checkpoint!(cancel_token);
            println!("Counter: start sleep some time");
            sleep(Duration::from_millis(100));
        }
        Ok(())
    }
}
