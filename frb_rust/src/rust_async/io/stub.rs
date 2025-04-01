use crate::BaseAsyncRuntime;
use std::future::Future;

#[derive(Default)]
pub struct SimpleAsyncRuntime();

impl BaseAsyncRuntime for SimpleAsyncRuntime {
    // Using a unit struct as our join handle since we don't have access to tokio or async-std
    type JoinHandle<O> = JoinHandle<O>;

    fn spawn<F>(&self, future: F) -> Self::JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        // Just execute the future immediately in the current thread
        let output = futures::executor::block_on(future);
        JoinHandle(std::marker::PhantomData, Some(output))
    }

    fn spawn_blocking<F>(&self, func: F) -> Self::JoinHandle<F::Output>
    where
        F: FnOnce() + Send + 'static,
        F::Output: Send + 'static,
    {
        // Execute the function immediately
        func();
        JoinHandle(std::marker::PhantomData, Some(()))
    }

    fn block_on<F: Future>(&self, future: F) -> F::Output {
        // Use the futures crate's executor to run the future
        futures::executor::block_on(future)
    }
}

pub struct JoinHandle<T>(std::marker::PhantomData<T>, Option<T>);

impl<T> Future for JoinHandle<T> {
    type Output = Result<T, Box<dyn std::error::Error + Send + Sync>>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut futures::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        // Safely access the inner value of Pin
        if let Some(output) = unsafe { std::pin::Pin::get_unchecked_mut(self) }.1.take() {
            std::task::Poll::Ready(Ok(output))
        } else {
            std::task::Poll::Ready(Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Task already completed",
            ))))
        }
    }
}

pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    // Create a dummy join handle with the result of executing the future immediately
    let output = futures::executor::block_on(future);
    JoinHandle(std::marker::PhantomData, Some(output))
}

pub fn spawn_local<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + 'static,
    F::Output: 'static,
{
    // For spawn_local, we also execute immediately since we don't have a real runtime
    let output = futures::executor::block_on(future);
    JoinHandle(std::marker::PhantomData, Some(output))
}

pub fn spawn_blocking<F>(func: F) -> JoinHandle<F::Output>
where
    F: FnOnce() + Send + 'static,
    F::Output: Send + 'static,
{
    // Execute the function immediately in the current thread
    func();
    JoinHandle(std::marker::PhantomData, Some(()))
}

pub fn spawn_blocking_with<F, R, TP>(f: F, _thread_pool_on_web: TP) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    // Ignore the thread pool parameter and execute the function immediately
    let output = f();
    JoinHandle(std::marker::PhantomData, Some(output))
}

pub struct RwLock<T> {
    inner: std::sync::RwLock<T>,
}

impl<T> RwLock<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: std::sync::RwLock::new(value),
        }
    }

    pub async fn read(&self) -> RwLockReadGuard<'_, T> {
        // Since we're in a dummy environment, we can just block
        RwLockReadGuard {
            guard: self.inner.read().unwrap(),
        }
    }

    pub async fn write(&self) -> RwLockWriteGuard<'_, T> {
        // Since we're in a dummy environment, we can just block
        RwLockWriteGuard {
            guard: self.inner.write().unwrap(),
        }
    }

    pub fn into_inner(self) -> T {
        self.inner
            .into_inner()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    pub fn blocking_read(&self) -> RwLockReadGuard<'_, T> {
        RwLockReadGuard {
            guard: self.inner.read().unwrap(),
        }
    }

    pub fn blocking_write(&self) -> RwLockWriteGuard<'_, T> {
        RwLockWriteGuard {
            guard: self.inner.write().unwrap(),
        }
    }

    pub fn try_read(&self) -> Option<RwLockReadGuard<'_, T>> {
        self.inner
            .try_read()
            .ok()
            .map(|guard| RwLockReadGuard { guard })
    }

    pub fn try_write(&self) -> Option<RwLockWriteGuard<'_, T>> {
        self.inner
            .try_write()
            .ok()
            .map(|guard| RwLockWriteGuard { guard })
    }
}

// Add the fmt implementation for Display
impl<T: std::fmt::Display> std::fmt::Display for RwLock<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.inner.read() {
            Ok(guard) => write!(f, "{}", *guard),
            Err(_) => write!(f, "<poisoned>"),
        }
    }
}

// Add the fmt implementation for Debug
impl<T: std::fmt::Debug> std::fmt::Debug for RwLock<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.inner.read() {
            Ok(guard) => f.debug_struct("RwLock").field("data", &*guard).finish(),
            Err(_) => write!(f, "RwLock(<poisoned>)"),
        }
    }
}

pub struct RwLockReadGuard<'a, T> {
    guard: std::sync::RwLockReadGuard<'a, T>,
}

impl<T> std::ops::Deref for RwLockReadGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

pub struct RwLockWriteGuard<'a, T> {
    guard: std::sync::RwLockWriteGuard<'a, T>,
}

impl<T> std::ops::Deref for RwLockWriteGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

impl<T> std::ops::DerefMut for RwLockWriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.guard
    }
}
