# Logging

`flutter_rust_bridge` comes with out-of-the-box logging, but you can override it with your own as well.
More concrete, you can overwrite the log outputting function with the one(s) of your logging framework(s) of choice, which will automatically log statements from Rust as well.

This implementation assumes that one is treating the Rust code as a library in Dart. 
Rust log statements are forwarded to dart, all log settings (levels, etc) are to be set from the Dart code. 
If you need it the other way around feel free to create an issue ticket.

Under the hood (and in a nutshell) our implementation uses the [log crate](https://crates.io/crates/log) crate and the [logging package](https://pub.dev/packages/logging), which are maintained by the respective language teams.

Log messages are send via a FRB's Stream implementation from Rust to Dart. 

## Setup

First you need to add a dependency on the logging crate in your Cargo.toml file with `cargo add log` or by putting `log = "0.4.20"` in your Cargo.toml file under the `[dependencies]` section.
Additionally you need to add a dependency to the chrono crate in the same way, e.g. `cargo add chrono` or by putting `chrono = "0.4.23"` in your Cargo.toml file under the `[dependencies]` section.

If you start with a new project (`flutter_rust_bridge_codegen create` instead of `flutter_rust_bridge_codegen generate`) these dependencies are already added for you.

Next, add the macro call `enable_frb_logging!();` in a **Rust** file that is part of your `rust_input` of your `flutter_rust_bridge.yaml` configuration, at any place outside of an item (e.g. function or struct). 
It needs to be there so the codegeneration is picking it up and generates the needed bridge code for connecting Rust and Dart for logging.

Your need to make it available via `use use flutter_rust_bridge::enable_frb_logging;` or `use flutter_rust_bridge::*;`.

## How to use it

All logs will be printed to `stdout`.

You can issue log statements in Rust and Dart by following the usual way done in the respective 3rd party packaged:

In **Rust** simply call `log::info!()` (or any of the [log crates log levels](https://docs.rs/log/latest/log/enum.Level.html)).
Note that the log levels are translated to [Darts logging package equivalents](https://pub.dev/documentation/logging/latest/logging/Level-class.html).

In **Dart** one is getting a logger instance per file, usually by `final LOGGER = Logger.('FileName');`. Instead, call `final LOGGER = FRBLogger.getLogger('LoggerName');` (where the logger name is optional and defaults to "FRBLogger").
This not only sets logging up for the Rust side, it returns a logger you can use as well.
To issue a log statement, you then call `LOGGER.info('Hello world');` (or similar variants) on that instance.
Doing this in `main.dart`, preferably as a global variable `final LOGGER = FRBLogger.getLogger();`, is recommended, so no Rust log statement is executed before this setup.

While Rust's log crate captures the module, file and line number of the log statement, Dart's logging package does not. 
Here it is ideomatic to create a new instance of the logger in each file, and give it a name that tells you where your log statement is originating from.
Usually one calls `final OTHER_LOGGER = Logger("other_logger");` from the library directly, again use `final OTHER_LOGGER = FRBLogger.getLogger("other_logger");` instead. 
This avoids the need to import the Dart logging package (but if you do the direct `Logger("other_logger")` after one initial `FRBLogger.getLogger();`, this works as well). 

## Customization

There are three parameters that can be customized:
1. `name`:The name of root (=first) logger on the Dart side (default: "FRBLogger").
2. `maxLoglevel`: The maximum log level to be captured by Rust and Dart (default: "Info").
3. `customLogFunction` The function to use for logging (default: `println!`).

You can overwrite the defaults by passing arguments to `enable_frb_logging!();` in your Rust code or `FRBLogger.initLogger()` in your Dart code.
The names of these are the same for both languages.
Be aware, that while the order doesn't matter in Dart, it matters in the Rust macro call!

Avoid setting these parameters on both sides, Rust and Dart, at the same time.
The implementation will take the values set in Dart, if `FRBLogger.initLogger()` is called before `FRBLogger.getLogger();` - otherwise it takes the values set with `enable_frb_logging!();`.

When you customize the logging in your Dart code with `FRBLogger.initLogger()`, the call returns an instance of a Logger, which you can use.
THere is no need for a call to `FRBLogger.getLogger();` (with the same name parameter), as this will return the same instance.

### Change the root logger name

To change the root logger name, either call `enable_frb_logging!(name = "My Name");` in Rust or `FRBLogger.initLogger(name = "My Name");` in Dart.
If you use the default and instanciate your logger in Dart with a name (`FRBLogger.getLogger("My Logger");`), you log statements using this logger instance will display "my logger", but the root logger will be named with the default name ("FRBLogger").
Unless you want to get the root logger (with `FRBLogger.getLogger("FRBLogger")`) you might not notice the difference.

### Change the log level

#### via Dart code

To set the log level filter in the Dart code initialize logging with `FRBLogger.initLogger(maxLoglevel: "WARNING");`, where `newMaxLogLevel` is a log level of [Darts logging package equivalents](https://pub.dev/documentation/logging/latest/logging/Level-class.html). Like anything else, this level will be applied to both, logging statements in Rust and dart.

#### via Rust code

To do the same from your Rust code, you can call `enable_frb_logging!(maxLoglevel: "WARNING");`.
Note that you must use the Dart logging package's log level names as well.

#### Via an environment variable

You can set the log level via an environment variable as well. 

Set `LOG_LEVEL=` to a value from the [Dart logging package](https://pub.dev/documentation/logging/latest/logging/Level-class.html). 
This will overwrite any programatically set level.

### Customize logging output

Out-of-the-box log messages are sent to _stdout_. If you want to customize the output or replace it with one from a more sophisticated logging framework (e.g. logging to a file), all you need to do is register your custom function for log outputs.

For this set the parameter `customLogFunction`.
This is a `void Function(Log2DartLogRecord)` in Dart and `fn _default_log_fn (record: Log2DartLogRecord)` in Rust.
Instead of a function you can pass a closure as well, in both languages.

To do so, in your **Dart** code call 
```Dart
FRBLogger.initLogger(customLogFunction: (Log2DartLogRecord record) => {print("This is ${record.level}! ${record.message}")});
``` 
and in your **Rust** code call:
```Rust
enable_frb_logging!(
  customLogFunction = (|record: Log2DartLogRecord| {
    let timestamp = chrono::Local::now();
    let max_log_level = from_u16(record.level_number);
    let lang = if record.rust_log { "Rust" } else { "Dart" };
    let logger_name = record.logger_name;
    let message = record.message;
    println!("[{timestamp:?} {max_log_level} @{lang} {logger_name}] {message})");
  })
);
```
This Rust function is the implemented default.

Note that your custom log function should be written in the same language where you configure it, i.e. in Dart when using `FRBLogger.initLogger` and in Rust when using `enable_frb_logging!`.

#### Anatomie of the log output

THe log function in both languages takes a `Log2DartLogRecord` as parameter.

`Log2DartLogRecord` combines the information from Dart's [LogRecord](https://pub.dev/documentation/logging/latest/logging/LogRecord-class.html) and Rust's [log::Record](https://docs.rs/log/0.4.22/log/struct.Record.html) as follows

```Rust
pub struct Log2DartLogRecord {
    pub level_number: u16,   // The log level encoded. Decode with `FRBLogger.logLevelFromNumber(x)` in Dart or `from_u16(x) in Rust. : Rust::log::Recod::Level, Dart::Logger::LogRecord::Level
    pub message: String, // The String given to the log statement: Rust::log::Recod::args, Dart::Logger::LogRecord::message
    pub logger_name: String, // The name of the logger given by `FRBLogger.initLogger(name: "MyClass");`, Rust::log::Recod::target, Dart::Logger::LogRecord::loggerName
    // pub time: String, // log::Recod::?, Dart::Logger::LogRecord::time --> omitted, as there is no time record in the log crate's Record
    pub rust_log: bool, // true, if the log statement originates from Rust code
    pub module_path: Option<String>, // Rust::log::Recod::module_path, None for Dart
    pub file_name: Option<String>, // Rust::log::Recod::file_name, None for Dart
    pub line_number: Option<u32>, // Rust::log::Recod::line_number, None for Dart
}
```

### Customize the log target

If you want to use log messages produced by another framework and/or send the log messages to other outputs, like a file, you can do this as well by providing a custom log function as described above.

Here you can use any Dart (or Rust) code, it doesn't need to be a closure, but that might be easier.
Your function only needs to take a single argument of type `LogRecord` as input, and return nothing.

Whatever you provide will be called whenever a log statemant is called in Rust or Dart.
If you want to combine multiple targets, like `stdout` and a file, combine them in one function.

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
However, it serves as a reminder that a direct instance of `FRBLogger` is not to be created, instead you need to use `FRBLogger.getLogger()` in your Dart code.


## Troubleshooting
### Unrecognized literal: `(/*ERROR*/)`
If you you get the error message
```
Running `(...)/target/debug/flutter_rust_bridge_codegen generate`
thread 'main' panicked at (...)/.cargo/registry/src/index.crates.io-6f17d22bba15001f/syn-2.0.28/src/lit.rs:1095:13:
Unrecognized literal: `(/*ERROR*/)`
```
the code in the `enable_frb_logging!();` macro has not been expanded correctly - most probably you forgot to add the dependency to `log = "0.4.20"` in your Cargo.toml file or you passed a syntactically incorrect custom function to the macro.