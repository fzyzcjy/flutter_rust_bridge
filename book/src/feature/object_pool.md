# Object pools

When there are some big objects in the Rust side, you may not want to copy them between Rust and Dart over and over again. That is when object pools become useful: You only pass around a "object handle" (indeed just a few integers) between Rust and Dart, and the Rust side will convert that handle from and to the real object.

Installation: Same as [cancelable tasks](cancelable_task.md), please see doc there.
