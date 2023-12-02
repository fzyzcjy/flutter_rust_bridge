/// The default error handler used by generated code.
#[derive(Clone, Copy)]
pub struct ReportDartErrorHandler;

impl ErrorHandler for ReportDartErrorHandler {
    fn handle_error(&self, port: MessagePort, error: Error) {
        match error {
            e @ Error::CustomError(_) => Rust2Dart::new(port).error(e),
            e @ Error::Panic(_) => Rust2Dart::new(port).panic(e),
        };
    }

    fn handle_error_sync(&self, error: Error) -> WireSyncReturn {
        let result_code = (&error).into();
        wire_sync_from_data(error.into_dart(), result_code)
    }
}

fn wire_sync_from_data<T: IntoDart>(data: T, result_code: Rust2DartAction) -> WireSyncReturn {
    let sync_return = vec![result_code.into_dart(), data.into_dart()].into_dart();

    #[cfg(not(wasm))]
    return crate::support::new_leak_box_ptr(sync_return);

    #[cfg(wasm)]
    return sync_return;
}
