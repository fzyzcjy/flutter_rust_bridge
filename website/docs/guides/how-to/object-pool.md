# Object pools

When there are some big objects, or even non-encodable objects in the Rust side,
you may not want (or impossible) to copy them between Rust and Dart over and over again.

## Opaque Rust types

This is, by default, handled via [opaque Rust types](../types/arbitrary/rust-auto-opaque),
and there is *no operation* needed to take care of the scenario.

## (Legacy) Real object pools

However, for legacy reasons (flutter_rust_bridge V1) and for completeness,
here we also document an alternative solution (object pool):
By using object pools,
you only pass around a "object handle" (just a few integers) between Rust and Dart,
and the Rust side will convert that handle from and to the real object.
For installation, please refer to [cancelable tasks](cancel).
