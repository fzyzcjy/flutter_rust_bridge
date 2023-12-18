# Isolate

Flutter web does not have the `isolate` functionalities, and those only exist on non-web platforms.
However, we do want to transfer messages between Dart and Rust, which used the ports of isolates, etc.
Therefore, we utilize web functionalities to make an API mimicking isolate on web, and use the real isolate otherwise.

This mainly includes the following API, both available on web and non-web:

* Ports (sending data from Rust to Dart)
* `IntoDart` (converting Rust data into Dart data)

This is quite low-level, thus some of them are not exported publicly to ensure our API to be minimal for maintainability.
However, if you want to use them, do not hesitate to create an issue/PR and we are happy to export them.
