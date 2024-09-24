# Logging

Flutter Rust Bridge comes with logging build in - but you can override it with your own logging framework of choice.

// TODO update what needs to be done for setup

If using the template by `flutter_rust_bridge_codegen create/integrate`, the "print logs to console" is configured by default,
via the auto-generated call to `flutter_rust_bridge::setup_default_user_utils()`.

Thus, you do not need to do anything :)


This implementation assumes that one is treating the Rust code as a library in Dart. 
Thus, all log settings (levels, etc) are to be set from the Dart code. 
If you need it the other way around feel free to create an issue ticket.

Under the hood (and in a nutshell) our implementation uses the [log crate](https://crates.io/crates/log) crate and the [logging package](https://pub.dev/packages/logging), which are maintained by the respective language teams.

Log messages are send via a FRB's Stream implementation from Rust to Dart. 

## How to use it
If you want to see log outputs from Rust and Dart in stdout you have do do nothing - except writing log statements:

In Rust the only thing one needs to do is calling `log::info!("Hello world")`, whereever you want to log something. (Use similar variants for other log levels.) All importing, etc. is already done for you.

In Dart, you need to call `LOGGER.info('Hello world');` (or similar variants). 

In Dart a global variable `final LOGGER = Logger('frb_logger');` is available that can be used to log messages (though you can define your own variable anywhere, if you want to destinguish the logging source better. `frb_logger` is the name of the logger - one can have multiple names defined.).

### change the log level
Call `Logger.root.level = newMaxLoglevel;`, where `newMaxLogLevel` is a log level of `Logger.Level`. 

This is how one changes the log level in Dart's `logging` package usually.
FRB is taking care, that the level for the rust logs is changed as well.

You can change the log level as ofthen as you want.
Because logs are asynchronious, it is possible that some logs still or are nor yet showing up, when changing the level in the middle of the program execution (instead of the begining).

### via an environment variable
You can set the log level via an environment variable as well. 
Set `Log_Level` to a value from the [Dart logging package](https://pub.dev/documentation/logging/latest/logging/Level-class.html).
This will overwrite any programatically set level.
Like anything else, this level will be applied to both, logging statements in rust and dart.

While these dart logging level names are unusual, they are more fine-granular.

## customize logging output
Out-of-the-box log messages are sent to stdout. If you want to customize the output or use a more sophisticated logging framework, all you need to do is register your custom function for log output.

For this call `LOGGER.setLogFunction(log);`, where `log` if the function you wish to do the logging (e.g. from your logging framework of choice).

If you want to use several frameworks to sent your logs to different places combine their log functions into one and set it as described above.

If you only want to customize the log output you can porvide a closure as well, like 
```Dart
LOGGER.setLogFunction((LogRecord record) => {print("This is ${record.level}! ${record.message}")});
```
.
[LogRecord](https://pub.dev/documentation/logging/latest/logging/LogRecord-class.html) is the data format from Darts' logging package.

The rust logs will use this new function as well.
However, not that only the fields `LogRecord.message`, `LogRecord.level`, and `LogRecord.loggerName` are used.

You can change the log function as often as you want, but due to the asynchronious nature of the logging, it is possible that some log messages are not yet using the new function.

## implementation
In a nutshell our implementation uses the [log crate](https://crates.io/crates/log) crate and the [logging package](https://pub.dev/packages/logging), which are maintained by the respective language teams.

Log messages are send via a FRB's Stream implementation from Rust to Dart. 

### Log Levels
The Rust-side [log levels](https://docs.rs/log/0.4.22/log/enum.LevelFilter.html) are mapped to the Dart-side [log levels](https://pub.dev/documentation/logging/latest/logging/Level-class.html) as follows:

```Rust
    match level {
        // Level('ALL', 0);
        // Level('OFF', 2000);
        // Level('FINEST', 300);
        // Level('FINER', 400);
        // Level('FINE', 500);
        0..=500 => LevelFilter::Trace,
        // Level('CONFIG', 700);
        501..=700 => LevelFilter::Debug,
        // Level('INFO', 800);
        701..=800 => LevelFilter::Info,
        // Level('WARNING', 900);
        801..=900 => LevelFilter::Warn,
        // Level('SEVERE', 1000);
        // Level('SHOUT', 1200);
        901..2000 => LevelFilter::Error,
        // Level('OFF', 2000);
        2000.. => LevelFilter::Off,
    }
```

As you can see you can define additional levels(`Level(String name, int value)`, stay beween - and 2000) in Dart. 
They will convert to Rust seamless.

### Remarks
#### timestamps from Rust
Log messages from rust show the time they are processed on the Dart side. As log messages are processed asynchroniously, that might be a bit off, but usually there is little to no delay (basicalle the time it needs for the Stream to transport the message from Rust to Dart).

