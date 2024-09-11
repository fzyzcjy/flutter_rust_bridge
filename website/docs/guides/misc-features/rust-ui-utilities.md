# Rust-GUI-via-Flutter Utilities

If you want to use Flutter as GUI of Rust,
please refer to the article in https://cjycode.com/posts/rust-ui-flutter/,
which contains background, pros, cons, tutorial, examples, etc.

## Under the hood

Firstly, this is just one of the infinitely many approaches to let Flutter be the GUI of Rust,
and you are free to choose whatever other approaches that suits your need.

In that article, the `#[frb(ui_state)]` and `#[frb(ui_mutation)]` lightweight attributes are utilized.
They are not stabilized, i.e. can have breaking changes without major version bumping.
More details are as follows:

* `#[frb(ui_state)]`: Annotate on your state struct.
* `#[frb(ui_mutation)]`: Annotate on state methods that mutates the state. It can also be annotated on the `impl` block,
  which is equivalent to annotate on each method in that block.

Then, whenever a method with `#[frb(ui_mutation)]` is called,
the Flutter UI will know the state is changed and needs to automatically refresh the UI.
