pub mod frb_generated {
    pub struct StreamSink<T> {
        marker: std::marker::PhantomData<T>,
    }

    impl<T> StreamSink<T> {
        pub fn add(&self, _value: T) -> Result<(), String> {
            Ok(())
        }
    }
}

flutter_rust_bridge::enable_frb_rust_to_dart_logging!(
    max_level = log::LevelFilter::Debug,
    setup_dart_logging_output = false
);

#[test]
fn test_enable_frb_rust_to_dart_logging_macro_exposes_configuration_functions() {
    assert_eq!(
        frb_internal_logging_max_level(),
        log::LevelFilter::Debug.to_string()
    );
    assert!(!frb_internal_logging_setup_dart_logging_output());
}

#[test]
fn test_parse_logging_max_level_accepts_known_levels() {
    assert_eq!(frb_parse_logging_max_level("OFF"), log::LevelFilter::Off);
    assert_eq!(
        frb_parse_logging_max_level("ERROR"),
        log::LevelFilter::Error
    );
    assert_eq!(frb_parse_logging_max_level("WARN"), log::LevelFilter::Warn);
    assert_eq!(frb_parse_logging_max_level("INFO"), log::LevelFilter::Info);
    assert_eq!(
        frb_parse_logging_max_level("DEBUG"),
        log::LevelFilter::Debug
    );
    assert_eq!(
        frb_parse_logging_max_level("TRACE"),
        log::LevelFilter::Trace
    );
}

#[test]
fn test_parse_logging_max_level_is_case_insensitive() {
    assert_eq!(
        frb_parse_logging_max_level("debug"),
        log::LevelFilter::Debug
    );
}
