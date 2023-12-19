use crate::dart_opaque::auto_drop_dart_persistent_handle::AutoDropDartPersistentHandle;
use crate::dart_opaque::boxes::guarded_box::{GuardedBox, GuardedBoxContext};
use delegate_attr::delegate;
use std::fmt::Debug;

/// Only allows manipulation of inner value at the Dart Isolate which it is created.
/// See the documentation of [GuardedBox] for more details.
#[derive(Debug)]
pub struct DartIsolateBox<T: Debug>(GuardedBox<T, GuardedBoxContextDartIsolate>);

impl<T: Debug> DartIsolateBox<T> {
    #[allow(unused)]
    pub fn new(inner: T) -> Self {
        Self(GuardedBox::new(inner))
    }
}

#[delegate(self.0)]
impl<T: Debug> DartIsolateBox<T> {
    pub fn check_context(&self) -> bool {}

    pub fn into_inner(self) -> T {}
}

#[delegate(self.0)]
impl<T: Debug> AsRef<T> for DartIsolateBox<T> {
    fn as_ref(&self) -> &T {}
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GuardedBoxContextDartIsolate(dart_sys_fork::Dart_Isolate);

impl GuardedBoxContext for GuardedBoxContextDartIsolate {
    fn current() -> Self {
        Self(unsafe { dart_sys_fork::Dart_CurrentIsolate_DL.unwrap()() })
    }
}

/// # Safety
///
/// We can safely implement Send/Sync because `AutoDropDartPersistentHandle` is a Dart VM object
/// that cna be used on the same isolate.
///
/// Note we *cannot** implement Send/Sync for arbitrary `DartIsolatedBox<T>`,
/// because if `T` is e.g. unrelated to Dart VM, then surely we are still unsafe.
unsafe impl Send for DartIsolateBox<AutoDropDartPersistentHandle> {}

/// # Safety
///
/// See documentation for `Send`
unsafe impl Sync for DartIsolateBox<AutoDropDartPersistentHandle> {}
