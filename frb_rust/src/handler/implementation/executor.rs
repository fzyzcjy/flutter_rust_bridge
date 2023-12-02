// TODO do not name "ThreadPool", since it has tokio etc
/// The default executor used.
/// It creates an internal thread pool, and each call to a Rust function is
/// handled by a different thread.
pub struct ThreadPoolExecutor<EH: ErrorHandler> {
    error_handler: EH,
}

impl<EH: ErrorHandler> ThreadPoolExecutor<EH> {
    /// Create a new executor backed by a thread pool.
    pub fn new(error_handler: EH) -> Self {
        ThreadPoolExecutor { error_handler }
    }
}

impl<EH: ErrorHandler + Sync> Executor for ThreadPoolExecutor<EH> {
    fn execute<TaskFn, TaskRet, D, Er>(&self, wrap_info: WrapInfo, task: TaskFn)
    where
        TaskFn: FnOnce(TaskCallback) -> Result<TaskRet, Er> + Send + UnwindSafe + 'static,
        TaskRet: IntoIntoDart<D>,
        D: IntoDart,
        Er: IntoDart + 'static,
    {
        let eh = self.error_handler;
        let eh2 = self.error_handler;

        let WrapInfo { port, mode, .. } = wrap_info;

        spawn!(|port: Option<MessagePort>| {
            let port2 = port.as_ref().cloned();
            let thread_result = panic::catch_unwind(move || {
                let port2 = port2.expect("(worker) thread");
                #[allow(clippy::clone_on_copy)]
                let rust2dart = Rust2Dart::new(port2.clone());

                let ret = task(TaskCallback::new(rust2dart.clone()))
                    .map(|e| e.into_into_dart().into_dart());

                match ret {
                    Ok(result) => {
                        match mode {
                            FfiCallMode::Normal => {
                                rust2dart.success(result);
                            }
                            FfiCallMode::Stream => {
                                // nothing - ignore the return value of a Stream-typed function
                            }
                            FfiCallMode::Sync => {
                                panic!("FfiCallMode::Sync should not call execute, please call execute_sync instead")
                            }
                        }
                    }
                    Err(error) => {
                        eh2.handle_error(port2, Error::CustomError(Box::new(error)));
                    }
                };
            });

            if let Err(error) = thread_result {
                eh.handle_error(port.expect("(worker) eh"), Error::Panic(error));
            }
        });
    }

    fn execute_sync<SyncTaskFn, TaskRet, D, Er>(
        &self,
        _wrap_info: WrapInfo,
        sync_task: SyncTaskFn,
    ) -> Result<TaskRet, Er>
    where
        SyncTaskFn: FnOnce() -> Result<TaskRet, Er> + UnwindSafe,
        TaskRet: IntoIntoDart<D>,
        D: IntoDart,
        Er: IntoDart,
    {
        sync_task()
    }

    #[cfg(feature = "rust-async")]
    fn execute_async<TaskFn, TaskRet, TaskRetFut, D, Er>(&self, wrap_info: WrapInfo, task: TaskFn)
    where
        TaskFn: FnOnce(TaskCallback) -> TaskRetFut + Send + UnwindSafe + 'static,
        TaskRet: IntoIntoDart<D>,
        TaskRetFut: Future<Output = Result<TaskRet, Er>> + TaskRetFutTrait + UnwindSafe,
        D: IntoDart,
        Er: IntoDart + 'static,
    {
        // TODO merge with `execute` case later
        // TODO avoid lock later

        let eh = self.error_handler;
        let eh2 = self.error_handler;

        rust_async::spawn(async move {
            let WrapInfo { port, mode, .. } = wrap_info;
            let port2 = port.as_ref().cloned();

            // TODO rename variable (not "thread" anymore)
            let thread_result = async {
                let port2 = port2.expect("(worker) thread");
                #[allow(clippy::clone_on_copy)]
                    let rust2dart = Rust2Dart::new(port2.clone());

                let ret = task(TaskCallback::new(rust2dart.clone()))
                    .await
                    .map(|e| e.into_into_dart().into_dart());

                match ret {
                    Ok(result) => {
                        match mode {
                            FfiCallMode::Normal => {
                                rust2dart.success(result);
                            }
                            FfiCallMode::Stream => {
                                // nothing - ignore the return value of a Stream-typed function
                            }
                            FfiCallMode::Sync => {
                                panic!("FfiCallMode::Sync should not call execute, please call execute_sync instead")
                            }
                        }
                    }
                    Err(error) => {
                        eh2.handle_error(port2, Error::CustomError(Box::new(error)));
                    }
                };
            }.catch_unwind().await;

            if let Err(error) = thread_result {
                eh.handle_error(port.expect("(worker) eh"), Error::Panic(error));
            }
        });
    }
}
