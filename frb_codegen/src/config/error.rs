#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(
        "build_runner configured to run, but Dart root could not be inferred.
Please specify --dart-root, or disable build_runner with --no-build-runner."
    )]
    FailedInferDartRoot,
}
