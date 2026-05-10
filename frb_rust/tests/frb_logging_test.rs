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

flutter_rust_bridge::enable_frb_logging!(
    max_level = log::LevelFilter::Debug,
    setup_dart_logging_output = false
);

#[test]
fn test_enable_frb_logging_macro_exposes_configuration_functions() {
    assert_eq!(frb_logging_max_level(), log::LevelFilter::Debug as u16);
    assert!(!frb_logging_setup_dart_logging_output());
}
