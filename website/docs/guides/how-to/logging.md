# Logging

`flutter_rust_bridge` comes with out-of-the-box logging, but you can override it with your own as well.
More concretely, you can override the log outputting function with the one(s) of your logging framework(s) of choice, which will automatically log statements from Rust as well.

Under the hood (and in a nutshell) our implementation uses the [Rust 'log' crate](https://crates.io/crates/log) and the [Dart 'logging' package](https://pub.dev/packages/logging), which are maintained by the respective language teams.

Log messages are sent via a FRB's Stream implementation from Rust to Dart.

## Setup
### 1. add the logging dependency
First you need to add a dependency on the logging crate in your Cargo.toml file with `cargo add log` or by putting `log = "^0.4.20"` in your Cargo.toml file under the `[dependencies]` section.
If you start with a new project (`flutter_rust_bridge_codegen create` instead of `flutter_rust_bridge_codegen generate`) this dependency is already added for you.

### 2. expose the generated StreamSink
Next, you need to expose a generated `StreamSink` so the logging code can find it. 
If you have not disabled automatically configuring your `lib.rs` in your `flutter_rust_bridge.yaml` (with `add_mod_to_lib: false`) these lines are added for you. 

Otherwise, enter
```
// this export is needed for logging
pub use crate::frb_generated::StreamSink as __FrbStreamSinkForLogging;
```
into your project's `lib.rs`.

Replace `frb_generated` in `crate::frb_generated::StreamSink` with the module path where you configured the generated files to go to, i.e. the value you set for `rust_output` in the `flutter_rust_bridge.yaml` configuration file. If you did not set this option `crate::frb_generated` is the correct default, no need to change this.

### 3. activate logging
Finally, add the macro call `enable_frb_logging!();` in a **Rust** file that is part of your `rust_input` of your `flutter_rust_bridge.yaml` configuration, at any place outside of an item (e.g. function or struct). 
You need to make it available via `use flutter_rust_bridge::enable_frb_logging;`.

It needs to be there so the code generation is picking it up and generates the needed bridge code for connecting Rust and Dart for logging.

### 4. customize the log function
Per default any log output is written to `stdout`, via `println!` in Rust.
You can provide your own log function, as described further down in `utilize a logging framework`.

Note that this is needed, if you are developing a _Flutter_ application, as `stdout` is not shown.

## How to use it

All logs will be printed to `stdout`.

You can issue log statements in Rust and Dart by following the usual way done in the respective 3rd party packages:

In **Rust** simply call `log::info!()` (or any of the [log crate's log levels](https://docs.rs/log/latest/log/enum.Level.html)).

In **Dart** you first need to initialise logging with a call to `final LOGGER = FRBLogger.initLogger()`.
If you do this call more than once an Exception is thrown.
This not only sets up logging for the Rust side, it returns a logger you can use to record log statements as well.

Doing this in `main.dart`, preferably as a global variable `final LOGGER = FRBLogger.initLogger();`, is recommended, so no Rust log statement is executed before this setup.

To get the logger handle after the initialization you can call `final LOGGER = FRBLogger.getLogger()`.
This is in turn calling `FRBDartLogger.getLogger()`, which you can call directly instead (they are effectively the same).

In both calls (`.initLogger()` and `.getLogger()`) you can provide a `LoggerName` (which defaults to "FRBLogger").

To issue a log statement, you then call `LOGGER.info('Hello world');` (or other log levels) on that instance.
Note that in **Rust** code you use the [log crate's log levels](https://docs.rs/log/latest/log/enum.Level.html), while in **Dart** code you use a mixture of those and [Dart's logging package equivalents](https://pub.dev/documentation/logging/latest/logging/Level-class.html).

All of Dart's logging package log levels are available, but the more exotic ones have been replaced with the more common levels used in Rust:
- FINE -> trace
- CONFIG -> debug
- SEVERE -> error
- SHOUT -> fatal

While Rust's log crate captures the module, file and line number of the log statement, Dart's logging package does not. 
Here it is idiomatic to get a new handle of the logger in each file, and give it a name that tells you where your log statement is originating from.
Usually one calls `final OTHER_LOGGER = Logger("other_logger");` from the library directly, but again, use `final OTHER_LOGGER = FRBLogger.getLogger("other_logger");` instead.

## Customization

There are three parameters that can be customized:
1. `name`: Name of the root (=first) logger on the Dart side (default: "FRBLogger").
2. `maxLoglevel`: Maximum log level to be captured by Rust and Dart (default: "Info").
3. `customLogFunction` The function to use for logging (default: `println!`).

You can override the defaults by passing arguments to `enable_frb_logging!();` in your Rust code or `FRBLogger.initLogger()` in your Dart code.
The parameter names of these are the same for both languages.
Be aware that while the order doesn't matter in Dart, it matters in the Rust macro call!

Avoid setting these parameters on both sides (Rust and Dart) at the same time.
The implementation will take the values set in Dart if `FRBLogger.initLogger()` is called before any subsequent `FRBLogger.getLogger();` call - otherwise, it takes the values set with `enable_frb_logging!();`.

When you customize the logging in your Dart code with `FRBLogger.initLogger()`, the call returns an instance of a logger, which you can use.
There is no need for a call to `FRBLogger.getLogger();` (with the same name parameter), as this will return the same instance.

### Change the root logger name

To change the root logger name, either call `enable_frb_logging!(name = "My Name");` in Rust or `FRBLogger.initLogger(name = "My Name");` in Dart.
If you use the default and instantiate your logger in Dart with a specific name (e.g. `FRBLogger.getLogger("My Logger");`), log statements using this new named logger will display "my logger", but the root logger will retain its default name ("FRBLogger").
Unless you want to get the root logger (with `FRBLogger.getLogger("FRBLogger")`) you might not notice the difference.

### Change the log level

#### via Dart code

To set the log level filter in the Dart code initialize logging with `FRBLogger.initLogger(maxLoglevel: "WARNING");` (where `newMaxLogLevel` is a mixture of Rust's log levels and  [Dart's logging package equivalents](https://pub.dev/documentation/logging/latest/logging/Level-class.html), as described above). 
Like anything else, this level will be applied to both, logging statements in Rust and Dart.

#### via Rust code

To do the same from your Rust code, you can call `enable_frb_logging!(maxLoglevel: "WARNING");`.
Note that you must use the Dart logging package's log level names as well.

#### Via an environment variable

You can set the log level via an environment variable as well. 

Set `LOG_LEVEL=` to a value from the [Dart logging package](https://pub.dev/documentation/logging/latest/logging/Level-class.html). 
This will override any programmatically set level.

### Customize logging output

Out-of-the-box log messages are sent to _stdout_. If you want to customize the output or replace it with one from a more sophisticated logging framework (e.g. logging to a file), all you need to do is register your custom function for log outputs.

To do this, set the parameter `customLogFunction`.
This is a `void Function(MirLogRecord)` in Dart and `fn _default_log_fn(record: MirLogRecord)` in Rust.
Instead of a function you can pass a closure as well, in both languages.

To do so, in your **Dart** code, call 
```Dart
FRBLogger.initLogger(customLogFunction: (MirLogRecord record) => {print("This is ${record.level}! ${record.message}")});
``` 
or in your **Rust** code call:
```Rust
enable_frb_logging!(
  customLogFunction = (|record: MirLogRecord| {
      let lang = if record.rust_log {"Rust"} else {"Dart"};
      let line_number = record.line_number
        .map_or("".to_string(), |number| format!(":{}", number));
      format!("[{} {} @{lang} {}{line_number}] {})", record.timestamp,      
        record.level_name, record.logger_name, record.message)
  })
);
```
This Rust function is the implemented default.

Note that your custom log function should be written in the same language where you configure it, i.e. in Dart when using `FRBLogger.initLogger` and in Rust when using `enable_frb_logging!`.

If you specify a custom log function to both calls the Dart function will be used (and the custom function specified in Rust will be ignored).****

If you are using `FRBLogger.initLogger` (and not `enable_frb_logging!()`) be careful to call this before any `.getLogger()` call!
Otherwise the Rust logger doesn't get initialized correctly.

#### utilize a logging framework
The logging support in FRB is intentionally kept low profile, meaning that the logging harness of the language teams (Rust and Dart) are used.
This takes care of dealing with log levels and computing the log message - but not where to write it to. Per default log messages are written to `stdout`.

However, usually this is not enough - you might want to log into a file, a database or another output so you can see log messages from an application inside an emulator or physical device.

To do so you need to use a logging framework in addition.
Set up the framework as usual and configure its logging function as the custom log function of FRB, as shown above.

##### configure the logger package
For example, let's take the Flutter package [logger](https://pub.dev/packages/logger):

1. Add the package to your app, e.g. via `dart pub add logger`.
2. Customize the logging output before any call to `.getLogger`:
```Dart
// import the framework doing the customized output
import 'package:logger/logger.dart';
// if names are clashing you might have to alias the internally used Logger class. You don't need to add it to your `pubspec.yaml`.
import 'package:logging/logging.dart' as Logging;
// import FRBDartLogger and LogLevel to be used in the customization
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
```
(...)
```Dart
late final FRBDartLogger logger;
```
(...)
```Dart
  // setup logging
  var frameworkLogger = Logger();
  logger = FRBLogger.initLogger(
    name: "StateHandler",
    maxLogLevel: LogLevel.debug,
    customLogFunction: ({required record}) {
      final message =
          "[${record.timestamp} ${record.levelName} ${record.rustLog ? "Rust: " : "Dart: "} LoggerName: ${record.loggerName}:${record.lineNumber ?? ""}]\n ${record.message}";
      frameworkLogger.log(_toLoggerLevel(record.levelNumber), message);
    },
  );
```
The framework requires a log level to be set. Our neutral representation `MirLogRecord` provides a number, which should be mapped to the logging framework's level.
We are implementing this helper function for that:
```Dart
// convert MirLogRecord.levelNumber to logger.level
  static Level _toLoggerLevel(int levelNumber) {
    switch (levelNumber) {
      case <= 1000:
        return Level.trace;
      case <= 2000:
        return Level.debug;
      case <= 3000:
        return Level.info;
      case <= 4000:
        return Level.warning;
      case <= 5000:
        return Level.error;
      case <= 6000:
        return Level.fatal;
      default:
        return Level.all;
    }
  }
```

#### Anatomie of the log output

The log function in both languages takes a `MirLogRecord` as parameter:

`MirLogRecord` combines the information from Dart's [LogRecord](https://pub.dev/documentation/logging/latest/logging/LogRecord-class.html) and Rust's [log::Record](https://docs.rs/log/0.4.22/log/struct.Record.html) as follows

```Rust
pub struct MirLogRecord {
    pub level_number: u16,   // The log level encoded. Decode with `FRBLogger.logLevelFromNumber(x)` in Dart or `from_u16(x)` in Rust. : `Rust::log::Record::Level`, `Dart::Logger::LogRecord::Level`
    pub message: String, // The String given to the log statement: `Rust::log::Record::args`, `Dart::Logger::LogRecord::message`
    pub logger_name: String, // The name of the logger given by `FRBLogger.initLogger(name: "MyClass");`, `Rust::log::Record::target`, `Dart::Logger::LogRecord::loggerName`
    // pub time: String, // log::Record::?, Dart::Logger::LogRecord::time --> omitted, as there is no time record in the log crate's Record
    pub rust_log: bool, // true, if the log statement originates from Rust code
    pub module_path: Option<String>, // `Rust::log::Record::module_path`, None for Dart
    pub file_name: Option<String>, // `Rust::log::Record::file_name`, None for Dart
    pub line_number: Option<u32>, // `Rust::log::Record::line_number`, None for Dart
}
```

### Customize the log target

If you want to use log messages produced by another framework and/or send the log messages to other outputs, like a file, you can do this as well by providing a custom log function as described above.

Here you can use any Dart (or Rust) code, it doesn't need to be a closure, but that might be easier.
Your function only needs to take a single argument of type `LogRecord` as input and return nothing.

Whatever you provide will be called whenever a log statement is called in Rust or Dart.
If you want to combine multiple targets, like `stdout` and a file, combine them in one function.

## implementation

In a nutshell, our implementation uses the Rust [log crate](https://crates.io/crates/log) and the Dart [logging package](https://pub.dev/packages/logging), which are maintained by the respective language teams.

Log messages are sent via a FRB's Stream implementation from Rust to Dart. 

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

However, we replaced the more exotic levels on the Dart side with the more common levels from Rust:
- FINE -> trace
- CONFIG -> debug
- SEVERE -> error
- SHOUT -> fatal

Thus, when logging from Dart, you can use these levels:
```Dart
enum LogLevel {
  /// Maps to `Level.ALL` and integer levels below 300.
  all(
    level: Level.ALL,
    levelNumberThreshold: 300,
  ),

  /// Maps to `Level.FINEST` and integer levels below 400.
  finest(
    level: Level.FINEST,
    levelNumberThreshold: 400,
  ),

  /// Maps to `Level.FINER` and integer levels below 500.
  finer(
    level: Level.FINER,
    levelNumberThreshold: 500,
  ),

  /// Maps to `Level.FINE` and integer levels below 700. This is typically used for Trace.
  trace(
    level: Level.FINE,
    levelNumberThreshold: 700,
  ),

  /// Maps to `Level.CONFIG` and integer levels below 800. This is typically used for Debug.
  debug(
    level: Level.CONFIG,
    levelNumberThreshold: 800,
  ),

  /// Maps to `Level.INFO` and integer levels below 900.
  info(
    level: Level.INFO,
    levelNumberThreshold: 900,
  ),

  /// Maps to `Level.WARNING` and integer levels below 1000. This is typically used for Warn.
  warn(
    level: Level.WARNING,
    levelNumberThreshold: 1000,
  ),

  /// Maps to `Level.SEVERE` and integer levels below 1200. This is typically used for Error.
  error(
    level: Level.SEVERE,
    levelNumberThreshold: 1200,
  ),

  /// Maps to `Level.SHOUT` and integer levels below 2000. This is typically used for Fatal/Shout.
  fatal(
    level: Level.SHOUT,
    levelNumberThreshold: 2000,
  ),

  /// Maps to `Level.OFF` and integer levels equal to or above 2000.
  off(
    level: Level.OFF,
    levelNumberThreshold: 2000, // Or any value that signifies 'Off'
  );
}
```

As you can see you can define additional levels(`Level(String name, int value)`, stay between 0 and 2000) in Dart. 
They will convert to Rust seamlessly.

### Remarks
#### noop function

`FRBLogger` exposes a `no-op` function, called `newInstance()`. 
This is primarily needed so that the code is picked up for code generation, and not optimized away.
However, if called it panics, reminding that a direct instance of `FRBLogger` is not to be created, instead you need to use `FRBLogger.initLogger()` in your Dart code.


## Troubleshooting
### Unrecognized literal: `(/*ERROR*/)`
If you get the error message
```
Running `(...)/target/debug/flutter_rust_bridge_codegen generate`
thread 'main' panicked at (...)/.cargo/registry/src/index.crates.io-6f17d22bba15001f/syn-2.0.28/src/lit.rs:1095:13:
Unrecognized literal: `(/*ERROR*/)`
```
the code in the `enable_frb_logging!();` macro has not been expanded correctly - most probably you forgot to add the dependency to `log = "^0.4.20"` in your Cargo.toml file or you passed a syntactically incorrect custom function to the macro.

### no `__FrbStreamSinkForLogging` in the root
If you get the error message 
```
enable_frb_logging!();
   | ^^^^^^^^^^^^^^^^^^^^^ no `__FrbStreamSinkForLogging` in the root
   |
   = note: this error originates in the macro `enable_frb_logging` (in Nightly builds, run with -Z macro-backtrace for more info)
```

the `StreamSink` is probably not re-exported in your project's `lib.rs`.
Add
```
// this export is needed for logging
pub use crate::frb_generated::StreamSink as __FrbStreamSinkForLogging;
```
to do so.
Notice that `frb_generated` has to point to the module path where you configured the generated code to be written to, which defaults to `crate::frb_generated`.

### no log output in the emulator!
If you are testing a Flutter application in an emulator the output to `stdout` is not visible.
You need to configure a logging plugin to work with, as described above in `utilize a logging framework`.
