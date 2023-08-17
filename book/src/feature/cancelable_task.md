# Cancellable tasks

When the Rust code is computationally heavy, you may want to cancel it at the middle when, for example, the user does not need it anymore. Then the precious computation power can be saved.

Installation: Currently, the feature is complete, and I have used it in my own app for a long time. (I have not merge this PR to the main repo just because I need to figure out how to put those code as if in `api.rs`.) Thus, visit [#333](https://github.com/fzyzcjy/flutter_rust_bridge/pull/333) and copy the code directly to your project, and use it as normal.
