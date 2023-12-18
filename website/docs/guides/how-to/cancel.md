# Cancellable tasks

When the Rust code is computationally heavy, you may want to cancel it at the middle when,
for example, the user does not need it anymore.
Then the precious computation power can be saved.

## Approach 1: Simple CancelToken

This is just a simple struct that,
on one side can signal cancel commands,
and on the other side can observe whether it is signaled.

Installation: Currently, the feature is complete,
and I have used it in my own app for a long time.
(I have not merge this PR to the main repo just because I need to figure out how to put those code as if in `api.rs`.)
Thus, visit [#333](https://github.com/fzyzcjy/flutter_rust_bridge/pull/333) and copy the code directly to your project, and use it as normal.

## Approach 2: Tokio CancellationToken

If you are using asynchronous Rust, `tokio` does provide a cancel token utility
useful in the async environment:
https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html

## Approach 3: Whatever cancel token crates

Since the feature is so simple, it is easy to home-make one by yourself (e.g. I have made one above).
Or use any crate, e.g. https://crates.io/search?q=cancel shows many crates about this.