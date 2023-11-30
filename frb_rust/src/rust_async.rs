// TODO move this file etc

lazy_static! {
    static ref ASYNC_RUNTIME: Mutex<ThreadPool> = Mutex::new(ThreadPool::with_name(
        "frb_workerpool".into(),
        get_worker_count()
    ));
}

pub(crate) fn init() {
    todo!()
}
