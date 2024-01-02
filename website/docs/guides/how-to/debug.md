# Debugging

## Debuggers

Flutter has [built-in support](https://docs.flutter.dev/testing/debugging) for using debuggers.
For example, you can run app [with breakpoints](https://docs.flutter.dev/tools/vs-code#run-app-with-breakpoints).
Since Rust is nothing but a compiled binary file (e.g. `something.so`) added to Flutter,
all Flutter utilities can be used as normal (for both native and web platform).

## Debugging by printing

Just use normal logging 
to [debug](https://stackoverflow.com/questions/189562/what-is-the-proper-name-for-doing-debugging-by-adding-print-statements) -
there is nothing special.

As for how to do logging:
For Dart, it is `print()`.
For Rust, it is `info!() / warn!() / ...` after [configuring the loggers](./logging).
