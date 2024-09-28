# Logging

Flutter Rust Bridge comes with logging build in - but you can override it with your own logging framework of choice.
More concrete, you can overwrite the log outputting function with the one(s) of your logging framework(s) of choice, which will automatically log statements from Rust as well.

This implementation assumes that one is treating the Rust code as a library in Dart. 
Rust log statements are forwarded to dart, all log settings (levels, etc) are to be set from the Dart code. 
If you need it the other way around feel free to create an issue ticket.

Under the hood (and in a nutshell) our implementation uses the [log crate](https://crates.io/crates/log) crate and the [logging package](https://pub.dev/packages/logging), which are maintained by the respective language teams.

Log messages are send via a FRB's Stream implementation from Rust to Dart. 

## Setup

First you need to add a dependency on the logging crate in your Cargo.toml file with `cargo add log` or by putting `log = "0.4.20"` in your Cargo.toml file under the `[dependencies]` section.
If you start with a new project (`flutter_rust_bridge_codegen create` instead of `flutter_rust_bridge_codegen generate`) this is already done for you.

Next, add the macro call `enable_frb_logging!();`in a **Rust** file that is part of your `rust_uinput` of your `flutter_rust_bridge.yaml` configuration. It doesn't need to be part of a function or struct - it just needs to be there so the codegeneration is picking it up and generates the needed bridge code for connecting Rust and Dart for logging.

Now you can issue log statements in Rust by simply calling `log::info!()` (or any of the [log crates log levels](https://docs.rs/log/latest/log/enum.Level.html)).
Note that the log levels are translated to [Darts logging package equivalents](https://pub.dev/documentation/logging/latest/logging/Level-class.html).

In **Dart** you are initializing the logging setup by calling `FRBLogger.init_logging();`.
This not only sets logging up for the Rust side, it returns the first logger you can use as well.
Doing this in `main.dart`, preferably as a global variable `final LOGGER = FRBLogger.init_logger();`, is recommended, so no Rust log statement is executed before this setup.

`FRBLogger.init_logging()` takes optional parameters for customizing the logging - see further down for how to do that.

## How to use it

All logs will be printed to `stdout`.

To log from **Rust** the only thing one needs to do (after the setup above) is calling `log::info!("Hello world")`, whereever you want to log something. (Use similar variants for other log levels.)

For **Dart**, as described in _setup_, you create a `LOGGER` instance by calling `final LOGGER = FRBLogger.init_logger();`.
To issue a log statement, you then call `LOGGER.info('Hello world');` (or similar variants) on that instance.

This instance is called `FRBLogger` - but by specifying `FRBLogger.init_logger(name: "MyClass");` you can give it another name.

While Rust's log crate captures the module, file and line number of the log statement, Dart's logging package does not. Here it is ideomatic to create a new instance of the logger in each file, and give it a name that tells you where your log statement is originating from.
Usually one calls `final OTHER_LOGGER = Logger("other_logger");` from the library directly.
Instead, use `final OTHER_LOGGER = FRBLogger.getLogger("other_logger");`; 
This avoids the need to import the Dart logging package (but if you do the direct `Logger("other_logger")` call works as well). 
Note that you can't call `FRBLogger.init_logger(name: "other logger");` more than once - if you do so the execution will fail with an error message.

## Customization
### Change the log level

To set the log level filter initialize logging with `FRBLogger.init_logger(maxLoglevel: "WARNING");`, where `newMaxLogLevel` is a log level of [Darts logging package equivalents](https://pub.dev/documentation/logging/latest/logging/Level-class.html). Like anything else, this level will be applied to both, logging statements in Rust and dart.

#### Via an environment variable

You can set the log level via an environment variable as well. 

Set `LOG_LEVEL=` to a value from the [Dart logging package](https://pub.dev/documentation/logging/latest/logging/Level-class.html). This will overwrite any programatically set level.

## Customize logging output
Out-of-the-box log messages are sent to _stdout_. If you want to customize the output or replace it with one from a more sophisticated logging framework (e.g. logging to a file), all you need to do is register your custom function for log outputs.

For this set the parameter `custom_log_function`, which is a `Function(Log2DartLogRecord)`. `Log2DartLogRecord` is FRBs own struct of a log message, combining the information from Dart's logging package and Rust's log crate.

### Customizing the log message

The log message can be customized by setting an own log function:
```Dart
LOGGER.init_logger(customLogFunction: (Log2DartLogRecord record) => {print("This is ${record.level}! ${record.message}")});
```

`Log2DartLogRecord` combines the information from Dart's [LogRecord](https://pub.dev/documentation/logging/latest/logging/LogRecord-class.html) and Rust's [log::Record](https://docs.rs/log/0.4.22/log/struct.Record.html) as follows

```Rust
pub struct Log2DartLogRecord {
    pub level_number: u16,   // The log level encoded. Decode with `FRBLogger.log_level_from_number(x)` : Rust::log::Recod::Level, Dart::Logger::LogRecord::Level
    pub message: String, // The String given to the log statement: Rust::log::Recod::args, Dart::Logger::LogRecord::message
    pub logger_name: String, // The name of the logger given by `FRBLogger.init_logger(name: "MyClass");`, Rust::log::Recod::target, Dart::Logger::LogRecord::loggerName
    // pub time: String, // log::Recod::?, Dart::Logger::LogRecord::time --> omitted, as there is no time record in the log crate's Record
    pub rust_log: bool, // true, if the log statement originates from Rust code
    pub module_path: Option<String>, // Rust::log::Recod::module_path, None for Dart
    pub file_name: Option<String>, // Rust::log::Recod::file_name, None for Dart
    pub line_number: Option<u32>, // Rust::log::Recod::line_number, None for Dart
}
```

### Customize the log target

If you want to use log messages produced by another framework and/or send the log messages to other outputs, like a file, you can do this as well by providing a custom log function, e.g.: `LOGGER.init_logger(customLogFunction: (Log2DartLogRecord record) => {print("This is ${record.level}! ${record.message}")});`.

Here you can use any Dart code, it doesn't need to be a closure as well.
Whatever you provide will be called whenever a log statemant is called in Rust or Dart.
If you want to combine multiple targets, like `stdout` and a file, combine them in one function.

You can call the default log function by calling `FRBLogger.default_log_function(record)`, which might be usefull if you just want to change where the log message is written to.

## implementation

In a nutshell our implementation uses the [log crate](https://crates.io/crates/log) crate and the [logging package](https://pub.dev/packages/logging), which are maintained by the respective language teams.

Log messages are send via a FRB's Stream implementation from Rust to Dart. 

### Log Levels

While the Dart logging level names are unusual, they are more fine-granular.
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
#### noop function

`FRBLogger` exposes a `noop` function, called `new()`. 
This is primarly needed so that the code is picked up for code generation, and not optimized away.
However, it serves as a reminder that a direct instance of `FRBLogger` is not to be created, instead you need to use `FRBLogger.init_logger()` in your Dart code.


## Troubleshooting
### Unrecognized literal: `(/*ERROR*/)`
If you you get the error message
```
Running `(...)/target/debug/flutter_rust_bridge_codegen generate`
thread 'main' panicked at (...)/.cargo/registry/src/index.crates.io-6f17d22bba15001f/syn-2.0.28/src/lit.rs:1095:13:
Unrecognized literal: `(/*ERROR*/)`
```
the code in the `enable_frb_logging!();` macro has not been expanded correctly - most ptovbably you forgot to add the dependency to `log = "0.4.20"` in your Cargo.toml file.